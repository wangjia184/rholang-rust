
use std::cell::RefMut;
use blake3::Hash;

use super::*;

// The following code is just for prototype test, they need be refactored for production!!!

struct IndependentConsumer {
    pub(self) bind_pattern : BindPattern,
    pub(self) continuation : TaggedContinuation,
    pub(self) persistent : bool,
    pub(self) peek : bool,
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



pub(super) type ConsumingChannel<'a> = (RefMut<'a, Transit>, BindPattern, Hash);


// only allow to update the passed-in transit(s)
// coordinator ensured there is no others are working on them when we are called here
impl Transit {

    pub(super) fn install(mut wrapper : TransitWrapper, task : InstallTask) -> TransitWrapper{

        let transit = &mut wrapper.transit;

        let independent_consumer = IndependentConsumer {
            bind_pattern : task.channel.1,
            continuation : TaggedContinuation::Callback(task.callback),
            persistent : true,
            peek : false,
        };
        transit.persistented_consumers.push(independent_consumer);

        wrapper
    }

    // check all the existing consumers, if no match, save it
    pub(super) fn produce(mut wrapper : TransitWrapper, mut task : ProduceTask) -> TransitWrapper {

        
        let transit = &mut wrapper.transit;

        
        //println!("Produce : data : {:?}, channel : {:?}", &task.data, &transit.dataums);
        

        // first try to search in temp consumers
        match transit.consumers.iter().position( |consumer| {
            // only match length for now
            consumer.bind_pattern.patterns.len() == task.data.pars.len()
        }) 
        {
            Some(idx) => {
                // a match is found
                let consumer = transit.consumers.remove(idx);

                let reply = Some((consumer.continuation, smallvec![task.data]));
                if let Err(_) = task.replier.send(reply) {
                    error!("task.replier.send(reply) failed");
                }
            },
            None => {

                match transit.persistented_consumers.iter().find( |persistented_consumer| {
                    // only match length for now
                    persistented_consumer.bind_pattern.patterns.len() == task.data.pars.len()
                }) 
                {
                    Some(persistented_consumer) => {
                        // a match is found
                        let reply = Some((persistented_consumer.continuation.clone(), smallvec![task.data]));
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

                        if let Err(_) = task.replier.send(Reply::None) {
                            error!("task.replier.send(Reply::None) failed");
                        }
                    }
                }
                
                
            }
        }

        

        wrapper
    }

    // check all the existing dataums, if no match, save it
    pub(super) fn consume( mut task : ConsumeTask<ConsumingChannel>){


        // record the position of matched dataums in each channel
        let mut tuples = ShortVector::new();
        let mut matched = true;

        for (transit, ref bind_pattern, ref hash) in &mut task.channels {

            let mut idx = 0;
            if matched {
                if let Some(i) = transit.dataums.iter()
                    .position( |dataum| {
                        // only match length for now
                        bind_pattern.patterns.len() == dataum.data.pars.len()
                    }) 
                {
                        idx = i;
                }
                else {
                    matched = false;
                }
            }
            
            tuples.push( (transit, idx, bind_pattern, hash) );
            
        }// for

        // all binds matched
        if matched {

            let data_list = tuples
                .into_iter()
                .map( |(transit, idx, _, _)| {
                    transit.dataums.remove(idx).data
                })
                .collect();

            let reply = Some((task.continuation, data_list));
            if let Err(_) = task.replier.send(reply) {
                error!("task.replier.send(reply) failed");
            }
        } else {
            
            if tuples.len() == 1 {
                let (transit, _, bind_pattern, hash) = tuples.pop().unwrap();
                let independent_consumer = IndependentConsumer {
                    bind_pattern : bind_pattern.clone(),
                    continuation : task.continuation,
                    persistent : task.persistent,
                    peek : task.peek,
                };
                transit.consumers.push(independent_consumer);
            } else {
                todo!("Store the joined consumer");
            }
            if let Err(_) = task.replier.send(Reply::None) {
                error!("task.replier.send(Reply::None) failed");
            }
            
            drop(tuples);
        }

    }
}





