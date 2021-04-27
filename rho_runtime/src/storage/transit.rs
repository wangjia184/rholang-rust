

use super::*;

// The following code is just for prototype test, they need be refactored for production!!!

struct IndependentConsumer {
    pub(self) bind_pattern : BindPattern,
    pub(self) continuation : TaggedContinuation,
}

pub(super) struct  Transit {
    id_base : u64,
    dataums : ShortVector<Dataum>,

    consumers : ShortVector<IndependentConsumer>,
    persistented_consumers : ShortVector<IndependentConsumer>,
}

#[derive(Debug)]
pub (super) struct  Dataum {
    id : u64,
    data : ListParWithRandom,
    persistent : bool,
}

impl Default for Transit {
    fn default() -> Self {
        Self {
            id_base : 0,
            dataums : ShortVector::default(),
            consumers : ShortVector::default(),
            persistented_consumers : ShortVector::default(),
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
    pub(super) fn produce(transit : &mut Transit, task : ProduceTask) {

       
        //println!("Produce : data : {:?}, channel : {:?}", &task.data, &transit.dataums);

        // first try to search in temp consumers
        match transit.consumers.iter().position( |consumer| {
            transit.is_matched(&task.data, &consumer.bind_pattern)
        }) 
        {
            Some(idx) => {
                // a match is found
                let consumer = transit.consumers.remove(idx);

                let reply = Some(smallvec![(consumer.continuation, smallvec![task.data])]);
                if let Err(_) = task.replier.send(reply) {
                    error!("task.replier.send(reply) failed");
                }
            },
            None => {
                // then search in persistented consumers
                match transit.persistented_consumers.iter().find( |persistented_consumer| {
                    transit.is_matched(&task.data, &persistented_consumer.bind_pattern)
                }) 
                {
                    Some(persistented_consumer) => {
                        // a match is found
                        let reply = Some(smallvec![(persistented_consumer.continuation.clone(), smallvec![task.data])]);
                        if let Err(_) = task.replier.send(reply) {
                            error!("task.replier.send(reply) failed");
                        }
                    },
                    None => {
                        // store it for later match
                        transit.id_base += 1;
                        let dataum = Dataum{
                            id : transit.id_base,
                            data : task.data,
                            persistent : task.persistent,
                        };
                        transit.dataums.push(dataum);

                        // TODO : check joined consumers

                        if let Err(_) = task.replier.send(None) {
                            error!("task.replier.send(Reply::None) failed");
                        }
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
        // indexes records the last matched positions
        let mut dataum_indexes: SmallVec<[usize; 5]> = smallvec![0; transits.len()];

        

        if task.persistent {

            let mut vector = ShortVector::new();

            let mut matched = true;
            while matched {
                for ( (transit, (_, bind_pattern) ), dataum_index  ) 
                    in transits.iter_mut().zip(&task.channels).zip(&mut dataum_indexes)
                {
                    matched = transit.find_first_dataum_position( bind_pattern, dataum_index);
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
                    .map( |(transit, idx) | {
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
                todo!("Store the joined consumer, be careful with the order of patterns");
            }
        }
        else {

            let mut matched = true;
            for ( (transit, (_, bind_pattern) ), dataum_index  ) 
                    in transits.iter_mut().zip(&task.channels).zip(&mut dataum_indexes)
            {
                matched = transit.find_first_dataum_position( bind_pattern, dataum_index);
                if !matched {
                    break;
                }
            }// for

            // for temporary consumer, send the dataum immediately if matches; otherwise store the continuation
            if matched {
                let data_list = transits
                    .iter_mut()
                    .zip(&dataum_indexes)
                    .map( |(transit, idx) | {
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
                    todo!("Store the joined consumer, be careful with the order");
                }
                if let Err(_) = task.replier.send(None) {
                    error!("task.replier.send(Reply::None) failed");
                }

            }

        }
    }


    // try to find the position of first match dataum since start_index
    #[inline]
    fn find_first_dataum_position(&self, bind_pattern : &BindPattern, start_index : &mut usize) -> bool {
        while *start_index < self.dataums.len() {
            if self.is_matched(&self.dataums[*start_index].data, bind_pattern) {
                return true;
            }
            *start_index = *start_index + 1;
        }
        false
    }

    #[inline]
    fn is_matched(&self, list_par_with_random : &ListParWithRandom, bind_pattern : &BindPattern) -> bool {
        // only match length for now
        bind_pattern.patterns.len() == list_par_with_random.pars.len()
    }
}





