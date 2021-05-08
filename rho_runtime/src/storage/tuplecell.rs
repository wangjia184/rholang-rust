

use super::*;
use std::{cell::RefMut, sync::Arc};
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use blake3::Hash;
use rustc_hash::{ FxHashMap };

// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!
// The following code is just for prototype, they need be refactored for production!!!

struct IndependentConsumer {
    pub(self) bind_pattern : BindPattern,
    pub(self) continuation : TaggedContinuation,
}

struct  JoinedConsumer {
    last_dataum_id : usize, // last scaned but unmatched dataum
    bind_pattern : BindPattern,
    pub(self) share : Arc<SharedJoinedConsumer>,
}

pub(super) struct  SharedJoinedConsumer {
    pub(super) channels : FxHashMap<Hash, usize>, // the value is the order : 0/1/2...
    continuation : TaggedContinuation,
    persistent : bool,
}

pub(super) type TupleCell = RefCell<Box<TuplespaceChannel>>;

pub(super) struct  TuplespaceChannel {
    hash : Hash,
    id_base : usize,
    dataums : RefCell<ShortVector<Dataum>>,

    consumers : ShortVector<IndependentConsumer>,
    persistented_consumers : ShortVector<IndependentConsumer>,
    joined_consumers : RefCell<ShortVector<JoinedConsumer>>,
}

#[derive(Debug)]
pub (super) struct  Dataum {
    id : usize, // id will not be zero
    data : ListParWithRandom,
}





// only allow to update the passed-in cell(s)
// coordinator ensured there is no others are working on them when we are called here
impl TuplespaceChannel {

    #[inline]
    pub(super) fn new(hash : Hash) -> RefCell<Box<Self>> {
        RefCell::new(
            Box::new(
                Self {
                    hash : hash,
                    id_base : 1,
                    dataums : RefCell::new( ShortVector::default() ),
                    consumers : ShortVector::default(),
                    persistented_consumers : ShortVector::default(),
                    joined_consumers : RefCell::new( ShortVector::default() ),
                }
            )
        )
        
    }

    pub(super) fn install(mut cell : RefMut<Box<TuplespaceChannel>>, task : InstallTask) {

        let independent_consumer = IndependentConsumer {
            bind_pattern : task.channel.1,
            continuation : TaggedContinuation::Callback(task.callback),
        };
        cell.persistented_consumers.push(independent_consumer);

    }

    // first check single consumers, if no match, then check joined consumers
    pub(super) fn produce(mut cell : RefMut<Box<TuplespaceChannel>>, task : ProduceTask) -> Option<JoinChannelTask> {

        //println!("produce {:?} ==> {:?}", &task.data.pars[0].exprs[0], &task.channel.0);

        // first try to search in temp consumers
        match cell.consumers.iter().position( |consumer| {
            TuplespaceChannel::is_matched(&task.data, &consumer.bind_pattern)
        }) 
        {
            Some(idx) => {
                // a match is found
                let consumer = cell.consumers.remove(idx);

                let reply = Some(smallvec![(consumer.continuation, smallvec![task.data])]);
                if let Err(_) = task.replier.send(reply) {
                    error!("task.replier.send(reply) failed");
                }
                None
            },
            None => {
                // then search in persistented consumers
                match cell.persistented_consumers.iter().find( |persistented_consumer| {
                    TuplespaceChannel::is_matched(&task.data, &persistented_consumer.bind_pattern)
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
                        cell.id_base += 1;
                        let dataum = Dataum {
                            id : cell.id_base,
                            data : task.data,
                        };
                        let mut dataums = cell.dataums.borrow_mut();
                        dataums.push(dataum);

                        // check joined consumers
                        // here we only need find an eligibile joined consumer, then notify coordinator to schedual on that again
                        for JoinedConsumer { ref mut last_dataum_id, ref bind_pattern, ref share} 
                            in cell.joined_consumers.borrow_mut().iter_mut()
                        {
                            let mut idx = 0; // TODO : here should be more smart to determine index from ID
                            if TuplespaceChannel::find_first_dataum_position(&dataums, bind_pattern, &mut idx, last_dataum_id) {
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
    pub(super) fn consume_single( cell : RefMut<Box<TuplespaceChannel>>, task : ConsumeTask){

        // TODO: implement a dedicated version
        TuplespaceChannel::consume_multiple(smallvec![cell], task);
    }

    // temporary consumer -
    //     1. try to match one dataum
    //     2. if no match, store the consumer
    // persistent consumer -
    //     1. try to match all dataums
    //     2. store the consumer
    pub(super) fn consume_multiple( mut cells : ShortVector<RefMut<Box<TuplespaceChannel>>>, task : ConsumeTask){

        // Not only the count are the same, their order must be the same!
        assert_eq!(cells.len(), task.channels.len());

        // first we need find out the position of matched dataums in each channel
        // the first u64 represents dataum_index last accessed (might be matched)
        // the second u64 represents unmatched dataum's id  
        let mut dataum_indexes: SmallVec<[[usize;2]; 5]> = smallvec![[0;2]; cells.len()];

        

        if task.persistent {

            let mut vector = ShortVector::new();

            let mut matched = true;
            while matched {
                for ( (cell, (_, bind_pattern) ), [dataum_index, dataum_id]  ) 
                    in cells.iter_mut().zip(&task.channels).zip(&mut dataum_indexes)
                {
                    matched = TuplespaceChannel::find_first_dataum_position( &cell.dataums.borrow(), bind_pattern, dataum_index, dataum_id);
                    if !matched {
                        
                        break;
                    }
                }// for

                if !matched {
                    break;
                }

                let data_list = cells
                    .iter_mut()
                    .zip(&dataum_indexes)
                    .map( |(cell, [idx, _]) | {
                        cell.dataums.borrow_mut().remove(*idx).data
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

            if cells.len() == 1 {
                assert_eq!(task.channels.len(), 1);
                cells[0].persistented_consumers.push(IndependentConsumer {
                    bind_pattern : task.channels.into_iter().next().unwrap().1,
                    continuation : task.continuation,
                });
            } else {
                // Store the joined consumer
                let share = Arc::new(SharedJoinedConsumer{
                    channels : task.channels.iter().enumerate().map(|(idx, (hash,_))| (*hash, idx)).collect(),
                    continuation : task.continuation,
                    persistent : true,
                } );
                for ( (cell, (_, bind_pattern) ), [_, dataum_id]  ) 
                    in cells.iter_mut().zip(task.channels).zip(&mut dataum_indexes)
                {
                    let joined_consumer = JoinedConsumer {
                        bind_pattern : bind_pattern,
                        last_dataum_id : *dataum_id,
                        share : share.clone(),
                    };
                    cell.joined_consumers.borrow_mut().push(joined_consumer);
                }
            }
        }
        else {

            let mut matched = true;
            for ( (cell, (_, bind_pattern) ), [dataum_index, dataum_id]  ) 
                    in cells.iter_mut().zip(&task.channels).zip(&mut dataum_indexes)
            {
                matched = TuplespaceChannel::find_first_dataum_position( &cell.dataums.borrow(), bind_pattern, dataum_index, dataum_id);
                if !matched {
                    break;
                }
            }// for

            // for temporary consumer, send the dataum immediately if matches; otherwise store the continuation
            if matched {
                let data_list = cells
                    .iter_mut()
                    .zip(&dataum_indexes)
                    .map( |(cell, [idx, _]) | {
                        cell.dataums.borrow_mut().remove(*idx).data
                    })
                    .collect();

                let reply = Some(smallvec![(task.continuation, data_list)]);
                if let Err(_) = task.replier.send(reply) {
                    error!("task.replier.send(reply) failed");
                }
            } else {

                if cells.len() == 1 {
                    assert_eq!(task.channels.len(), 1);
                    cells[0].consumers.push(IndependentConsumer {
                        bind_pattern : task.channels.into_iter().next().unwrap().1,
                        continuation : task.continuation,
                    });
                } else {
                    // store the joined consumer
                    let share = Arc::new(SharedJoinedConsumer{
                        channels : task.channels.iter().enumerate().map(|(idx, (hash,_))| (*hash, idx)).collect(),
                        continuation : task.continuation,
                        persistent : false,
                    } );
                    for ( (cell, (_, bind_pattern) ), [_, dataum_id]  ) 
                        in cells.iter_mut().zip(task.channels).zip(&mut dataum_indexes)
                    {
                        let joined_consumer = JoinedConsumer {
                            bind_pattern : bind_pattern,
                            last_dataum_id : *dataum_id,
                            share : share.clone(),
                        };
                        cell.joined_consumers.borrow_mut().push(joined_consumer);
                    }
                }
                if let Err(_) = task.replier.send(None) {
                    error!("task.replier.send(Reply::None) failed");
                }

            }

        }
    }


    // try to find the position of first match dataum since start_index
    
    // start_index 
    //      - input : the index of the dataum to scan from
    //      - output : the index of the last scanned dataum, if it is matched, that is the matched one
    // dataum_id 
    //      - output : id of last scanned but unmatched dataum
    #[inline]
    fn find_first_dataum_position(dataums : &ShortVector<Dataum>, bind_pattern : &BindPattern, start_index : &mut usize, dataum_id : &mut usize) -> bool {
        while *start_index < dataums.len() {
            let current_id = dataums[*start_index].id;
            if *dataum_id < current_id { // this dataum was not scanned
                if TuplespaceChannel::is_matched(&dataums[*start_index].data, bind_pattern) {
                    return true;
                }
                *dataum_id = current_id;
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


    pub(super) fn join( mut cells : ShortVector<RefMut<Box<TuplespaceChannel>>>, join_task : JoinChannelTask){

        let match_result = TuplespaceChannel::match_one( &mut cells, &join_task);
        match match_result {
            MatchResult::NoMatch => {
                if let Err(_) = join_task.replier.send(None) {
                    error!("task.replier.send(Reply::None) failed");
                }
            },
            MatchResult::Matched(shared_consumer,index_pairs) => {
                // the returned order of elements in index_pairs is the same as the order of cells
                // but it might be different from the order of the continuation
                // hence need resort it.
                let mut data_list: ShortVector<(usize, ListParWithRandom)> = cells
                    .iter_mut()
                    .zip(&index_pairs)
                    .map( |(cell, [ consumer_idx, dataum_idx]) | { 
                        if !shared_consumer.persistent {
                            cell.joined_consumers.borrow_mut().remove(*consumer_idx);
                        }
                        (
                            *shared_consumer.channels.get(&cell.hash).unwrap(),
                            cell.dataums.borrow_mut().remove(*dataum_idx).data
                        )
                    })
                    .collect();

                data_list.sort_by(|x, y| x.0.cmp(&y.0));

                let sorted_data_list = data_list.into_iter().map(|(_,p)| p).collect();
                let reply = Some(smallvec![(shared_consumer.continuation.clone(), sorted_data_list)]);
                if let Err(_) = join_task.replier.send(reply) {
                    error!("task.replier.send(reply) failed");
                }
            }
        };
        

        

    }

    fn match_one( cells : &mut ShortVector<RefMut<Box<TuplespaceChannel>>>, join_task : &JoinChannelTask) -> MatchResult {
        // the value records the index of joined_consumer and dataum
        let mut full_joined_consumers : FxHashMap<*const SharedJoinedConsumer, ShortVector<[usize;2]>> = FxHashMap::default();
        for cell in cells 
        {
            if cell.dataums.borrow().is_empty() {
                
                return MatchResult::NoMatch;
            }

            for (consumer_idx, JoinedConsumer { last_dataum_id, bind_pattern, share }) 
                in cell.joined_consumers.borrow_mut().iter_mut().enumerate()
            {
                let mut dataum_idx = 0; // TODO : should be more smart to determine the start index by id
                if TuplespaceChannel::find_first_dataum_position(&cell.dataums.borrow_mut(), bind_pattern, &mut dataum_idx, last_dataum_id) {
                    if share.channels.len() == join_task.consumer.channels.len() &&
                       share.channels.iter().all(|(h, _)| join_task.consumer.channels.contains_key(h) ) {
                        // all cells are joined
                        match full_joined_consumers.entry(Arc::as_ptr(share) ) {
                            Entry::Occupied(o) => {
                                let borrow = o.into_mut();
                                borrow.push([consumer_idx, dataum_idx]); // record the indexes
                                if borrow.len() >= join_task.consumer.channels.len() {

                                    // all channels are ready
                                    return MatchResult::Matched(
                                        share.clone(),
                                        std::mem::replace(borrow, ShortVector::default())
                                    );
                                }
                            },
                            Entry::Vacant(v) => {
                                v.insert(smallvec![[consumer_idx, dataum_idx]]);
                            },
                        };
                    } else {
                        // not all cell are joined, but current one is qualified
                        unimplemented!("partial join are not implemented yet");
                    }
                } else {
                    println!("No match");
                }
            }

        }

        return MatchResult::NoMatch;
    }
}


enum MatchResult {
    NoMatch,
    Matched(Arc<SharedJoinedConsumer>, ShortVector<[usize;2]>), // the indexes of marched dataums of all channels in given order
}




