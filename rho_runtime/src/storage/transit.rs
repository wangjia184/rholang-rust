

use super::*;
use std::sync::Arc;
use std::collections::hash_map::Entry;
use blake3::Hash;
use rustc_hash::{ FxHashMap, FxHashSet };

// The following code is just for prototype test, they need be refactored for production!!!

struct IndependentConsumer {
    pub(self) bind_pattern : BindPattern,
    pub(self) continuation : TaggedContinuation,
}

struct  JoinedConsumer {
    next_dataum_id : usize, // the next dataum id to be scanned
    bind_pattern : BindPattern,
    pub(self) share : Arc<SharedJoinedConsumer>,
}

pub(super) struct  SharedJoinedConsumer {
    pub(super) channels : FxHashSet<Hash>,
    continuation : TaggedContinuation,
    persistent : bool,
}

pub(super) struct  Transit {
    id_base : usize,
    dataums : ShortVector<Dataum>,

    consumers : ShortVector<IndependentConsumer>,
    persistented_consumers : ShortVector<IndependentConsumer>,
    joined_consumers : ShortVector<JoinedConsumer>,
}

#[derive(Debug)]
pub (super) struct  Dataum {
    id : usize,
    data : ListParWithRandom,
}

impl Default for Transit {
    fn default() -> Self {
        Self {
            id_base : 0,
            dataums : ShortVector::default(),
            consumers : ShortVector::default(),
            persistented_consumers : ShortVector::default(),
            joined_consumers : ShortVector::default(),
        }
    }
}




// only allow to update the passed-in transit(s)
// coordinator ensured there is no others are working on them when we are called here
impl Transit {

    pub(super) fn install(transit : &mut Transit, task : InstallTask) {

        let independent_consumer = IndependentConsumer {
            bind_pattern : task.channel.1,
            continuation : TaggedContinuation::Callback(task.callback),
        };
        transit.persistented_consumers.push(independent_consumer);

    }

    // first check single consumers, if no match, then check joined consumers
    pub(super) fn produce(transit : &mut Transit, task : ProduceTask) -> Option<JoinChannelTask> {

       
        //println!("Produce : data : {:?}, channel : {:?}", &task.data, &transit.dataums);

        // first try to search in temp consumers
        match transit.consumers.iter().position( |consumer| {
            Transit::is_matched(&task.data, &consumer.bind_pattern)
        }) 
        {
            Some(idx) => {
                // a match is found
                let consumer = transit.consumers.remove(idx);

                let reply = Some(smallvec![(consumer.continuation, smallvec![task.data])]);
                if let Err(_) = task.replier.send(reply) {
                    error!("task.replier.send(reply) failed");
                }
                None
            },
            None => {
                // then search in persistented consumers
                match transit.persistented_consumers.iter().find( |persistented_consumer| {
                    Transit::is_matched(&task.data, &persistented_consumer.bind_pattern)
                }) 
                {
                    Some(persistented_consumer) => {
                        // a match is found
                        let reply = Some(smallvec![(persistented_consumer.continuation.clone(), smallvec![task.data])]);
                        if let Err(_) = task.replier.send(reply) {
                            error!("task.replier.send(reply) failed");
                        }
                        None
                    },
                    None => {
                        // store it for later match
                        transit.id_base += 1;
                        let dataum = Dataum {
                            id : transit.id_base,
                            data : task.data,
                        };
                        transit.dataums.push(dataum);

                        let dataums = &transit.dataums;
                        // check joined consumers
                        // here we only need find an eligibile joined consumer, then notify coordinator to schedual on that again
                        for JoinedConsumer { ref mut next_dataum_id, ref bind_pattern, ref share} 
                            in &mut transit.joined_consumers 
                        {
                            if *next_dataum_id < dataums[0].id {
                                *next_dataum_id = transit.dataums[0].id; // the next id is too old, reset to head of dataums
                            }
                            let mut idx = *next_dataum_id - transit.dataums[0].id;
                            if Transit::find_first_dataum_position(&dataums, bind_pattern, &mut idx, next_dataum_id) {
                                return Some(JoinChannelTask {
                                    replier : task.replier,
                                    consumer : share.clone(),
                                });
                            }
                        }

                        if let Err(_) = task.replier.send(None) {
                            error!("task.replier.send(None) failed");
                        }
                        None
                    }
                }
                
                
            }
        }

    }


    // A dedicated implementation for performance
    #[inline]
    pub(super) fn consume_single( transit : &mut Transit, task : ConsumeTask){

        // TODO: implement a dedicated version
        Transit::consume_multiple(smallvec![transit], task);
    }

    // temporary consumer -
    //     1. try to match one dataum
    //     2. if no match, store the consumer
    // persistent consumer -
    //     1. try to match all dataums
    //     2. store the consumer
    pub(super) fn consume_multiple( mut transits : ShortVector<&mut Transit>, task : ConsumeTask){

        // Not only the count are the same, their order must be the same!
        assert_eq!(transits.len(), task.channels.len());

        // first we need find out the position of matched dataums in each channel
        // the first u64 represents dataum_index last accessed (might be matched)
        // the second u64 represents unmatched dataum's id  
        let mut dataum_indexes: SmallVec<[[usize;2]; 5]> = smallvec![[0;2]; transits.len()];

        

        if task.persistent {

            let mut vector = ShortVector::new();

            let mut matched = true;
            while matched {
                for ( (transit, (_, bind_pattern) ), [dataum_index, dataum_id]  ) 
                    in transits.iter_mut().zip(&task.channels).zip(&mut dataum_indexes)
                {
                    matched = Transit::find_first_dataum_position( &transit.dataums, bind_pattern, dataum_index, dataum_id);
                    if !matched {
                        
                        break;
                    }
                }// for

                if !matched {
                    break;
                }

                let data_list = transits
                    .iter_mut()
                    .zip(&dataum_indexes)
                    .map( |(transit, [idx, _]) | {
                        transit.dataums.remove(*idx).data
                    })
                    .collect();

                 vector.push((task.continuation.clone(), data_list));
            }

            // for prototype only, it is not a good idea to find all dataums and send at once since there might be a lot
            if !vector.is_empty() {
                if let Err(_) = task.replier.send(Some(vector)) {
                    error!("task.replier.send(Some(vector)) failed");
                }
            }

            if transits.len() == 1 {
                assert_eq!(task.channels.len(), 1);
                transits[0].persistented_consumers.push(IndependentConsumer {
                    bind_pattern : task.channels.into_iter().next().unwrap().1,
                    continuation : task.continuation,
                });
            } else {
                //todo!("Store the joined consumer, be careful with the order of patterns");
                let share = Arc::new(SharedJoinedConsumer{
                    channels : task.channels.iter().map(|(hash,_)| *hash).collect(),
                    continuation : task.continuation,
                    persistent : true,
                } );
                for ( (transit, (_, bind_pattern) ), [_, dataum_id]  ) 
                    in transits.iter_mut().zip(task.channels).zip(&mut dataum_indexes)
                {
                    let joined_consumer = JoinedConsumer {
                        bind_pattern : bind_pattern,
                        next_dataum_id : *dataum_id,
                        share : share.clone(),
                    };
                    transit.joined_consumers.push(joined_consumer);
                }
            }
        }
        else {

            let mut matched = true;
            for ( (transit, (_, bind_pattern) ), [dataum_index, dataum_id]  ) 
                    in transits.iter_mut().zip(&task.channels).zip(&mut dataum_indexes)
            {
                matched = Transit::find_first_dataum_position( &transit.dataums, bind_pattern, dataum_index, dataum_id);
                if !matched {
                    break;
                }
            }// for

            // for temporary consumer, send the dataum immediately if matches; otherwise store the continuation
            if matched {
                let data_list = transits
                    .iter_mut()
                    .zip(&dataum_indexes)
                    .map( |(transit, [idx, _]) | {
                        transit.dataums.remove(*idx).data
                    })
                    .collect();

                let reply = Some(smallvec![(task.continuation, data_list)]);
                if let Err(_) = task.replier.send(reply) {
                    error!("task.replier.send(reply) failed");
                }
            } else {

                if transits.len() == 1 {
                    assert_eq!(task.channels.len(), 1);
                    transits[0].consumers.push(IndependentConsumer {
                        bind_pattern : task.channels.into_iter().next().unwrap().1,
                        continuation : task.continuation,
                    });
                } else {
                    // store the joined consumer
                    let share = Arc::new(SharedJoinedConsumer{
                        channels : task.channels.iter().map(|(hash,_)| *hash).collect(),
                        continuation : task.continuation,
                        persistent : false,
                    } );
                    for ( (transit, (_, bind_pattern) ), [_, dataum_id]  ) 
                        in transits.iter_mut().zip(task.channels).zip(&mut dataum_indexes)
                    {
                        let joined_consumer = JoinedConsumer {
                            bind_pattern : bind_pattern,
                            next_dataum_id : *dataum_id,
                            share : share.clone(),
                        };
                        transit.joined_consumers.push(joined_consumer);
                    }
                }
                if let Err(_) = task.replier.send(None) {
                    error!("task.replier.send(Reply::None) failed");
                }

            }

        }
    }


    // try to find the position of first match dataum since start_index
    #[inline]
    fn find_first_dataum_position(dataums : &ShortVector<Dataum>, bind_pattern : &BindPattern, start_index : &mut usize, dataum_id : &mut usize) -> bool {
        while *start_index < dataums.len() {
            *dataum_id = dataums[*start_index].id;
            if Transit::is_matched(&dataums[*start_index].data, bind_pattern) {
                return true;
            }
            *start_index = *start_index + 1;
        }
        *dataum_id += 1; // no match, expect next one
        false
    }

    #[inline]
    fn is_matched(list_par_with_random : &ListParWithRandom, bind_pattern : &BindPattern) -> bool {
        // only match length for now
        bind_pattern.patterns.len() == list_par_with_random.pars.len()
    }


    pub(super) fn join( mut transits : ShortVector<(Hash, &mut Transit)>, join_task : JoinChannelTask){

        // the value records the number of channels are qualified
        let mut full_joined_consumers : FxHashMap<*const SharedJoinedConsumer, usize> = FxHashMap::default();
        
        for (hash, transit) in transits 
        {

            if transit.dataums.is_empty() {
                break;
            }

            for JoinedConsumer { next_dataum_id, bind_pattern, share } 
                in &mut transit.joined_consumers 
            {
                if *next_dataum_id < transit.dataums[0].id {
                    *next_dataum_id = transit.dataums[0].id; // the next id is too old, reset to head of dataums
                }
                let mut idx = *next_dataum_id - transit.dataums[0].id;
                if Transit::find_first_dataum_position(&transit.dataums, bind_pattern, &mut idx, next_dataum_id) {
                    if share.channels.iter().all(|h| join_task.consumer.channels.contains(h) ) {
                        // all transits are joined
                        match full_joined_consumers.entry(&*share as *const SharedJoinedConsumer ) {
                            Entry::Occupied(o) => {
                                let count = o.into_mut();
                                *count += 1;
                                if *count >= join_task.consumer.channels.len() {
                                    // all channels are ready
                                }
                            },
                            Entry::Vacant(v) => {
                                v.insert(1);
                            },
                        };
                    } else {
                        // not all transit are joined
                    }
                }
            }

        }

    }
}





