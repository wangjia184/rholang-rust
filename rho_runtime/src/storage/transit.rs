

use blake3::Hash;

use super::*;

struct IndependentConsumer {
    pub(self) bind_pattern : BindPattern,
    pub(self) body : ParWithRandom,
    pub(self) persistent : bool,
    pub(self) peek : bool,
}

pub(super) struct  Transit {
    id_base : u64,
    dataums : ShortVector<Dataum>,

    independent_consumers : ShortVector<IndependentConsumer>,
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
            independent_consumers : ShortVector::default(),
        }
    }
}



pub(super) type ProducingChannel = TransitWrapper;
pub(super) type ConsumingChannel = (TransitWrapper, BindPattern, Hash);


// we are only allowed to update the passed-in transit(s)
// coordinator ensures there is no others are working on them when we are called here
impl Transit {
    // check all the existing consumers, if no match, save it
    pub(super) fn produce( mut task : ProduceTask<ProducingChannel>) -> TransitWrapper {

        let transit = &task.channel.transit;
        

        

        // first try to search in independent consumers
        match transit.independent_consumers.iter().position( |consumer| {
            // only match length for now
            consumer.bind_pattern.patterns.len() == task.data.pars.len()
        }) 
        {
            Some(idx) => {
                // a match is found
                let transit = &mut task.channel.transit;
                let consumer = transit.independent_consumers.remove(idx);

                if let Err(_) = task.replier.send(Reply::ParWithBody( consumer.body, smallvec![task.data])) {
                    error!("task.replier.send(Reply::ParWithBody( consumer.body, smallvec![task.data])) failed");
                }
            },
            None => {
                let transit = &mut task.channel.transit;
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

        

        task.channel
    }

    // check all the existing dataums, if no match, save it
    pub(super) fn consume( mut task : ConsumeTask<ConsumingChannel>) -> ShortVector<TransitWrapper>{

        // record the position of matched dataums in each channel
        let mut tuples = ShortVector::new();
        let mut matched = true;

        for (ref mut wrapper, bind_pattern, hash) in &mut task.channels {

            let transit = &mut wrapper.transit;

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

            if let Err(_) = task.replier.send(Reply::ParWithBody( task.body, data_list)) {
                error!("task.replier.send(Reply::ParWithBody( task.body, data_list)) failed");
            }
        } else {
            
            if tuples.len() == 1 {
                let (transit, _, bind_pattern, hash) = tuples.pop().unwrap();
                let independent_consumer = IndependentConsumer {
                    bind_pattern : bind_pattern.clone(),
                    body : task.body,
                    persistent : task.persistent,
                    peek : task.peek,
                };
                transit.independent_consumers.push(independent_consumer);
            } else {
                todo!("Store the joined consumer");
            }
            if let Err(_) = task.replier.send(Reply::None) {
                error!("task.replier.send(Reply::None) failed");
            }
            
            drop(tuples);
        }

        task.channels.into_iter().map( |c| c.0).collect()
    }
}





