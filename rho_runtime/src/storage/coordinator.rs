use std::collections::hash_map::Entry;
use std::cell::{ RefCell };
use std::rc::Rc;
use std::sync::Arc;
use tokio::task::{self, JoinHandle};
use tokio::sync::Notify;
use futures::future::Either;

use rustc_hash::{ FxHashMap };

use super::*;
use crossbeam::queue::ArrayQueue;
use blake3::Hash;

enum PendingTask{
    Install(InstallTask),
    Uninstall,
    Produce(ProduceTask),
    Consume(ConsumeTask),
    Join(JoinChannelTask),
}





pub(super) struct InstallTask {
    pub(super) channel : (Hash, BindPattern),
    pub(super) callback : RustCallbacFunction,
}


pub(super) struct ProduceTask {
    pub(super) replier : oneshot::Sender<Reply>,
    pub(super) channel : (Hash, Par),
    pub(super) data : ListParWithRandom,
    pub(super) persistent : bool,
}





pub(super) struct ConsumeTask {
    pub(super) replier : oneshot::Sender<Reply>,
    pub(super) channels : ShortVector<(Hash, BindPattern)>,
    pub(super) continuation : TaggedContinuation,
    pub(super) persistent : bool,
    pub(super) peek : bool,
}

pub(super) struct JoinChannelTask {
    pub(super) replier : oneshot::Sender<Reply>,
    pub(super) consumer : Arc<super::transit::SharedJoinedConsumer>,
}

struct ThreadSafeShare {
    notify : Notify,
    queue : ArrayQueue<PendingTask>,
}

pub struct Coordinator {
    share : Arc<ThreadSafeShare>,
    transit_port_map : FxHashMap<Hash, Rc<RefCell<TransitPort>>>
}


#[derive(Default)]
struct TransitPort {
    completed_signal : Option<CompletionSignal>,
}

#[derive(Clone)]
pub struct  AsyncStore {
    share : Arc<ThreadSafeShare>,
}

#[async_trait]
impl Storage for AsyncStore {

    fn install(&self, channel : Par, bind_pattern : BindPattern, func : RustCallbacFunction) -> Reply {
        let install_task = InstallTask{
            channel : (channel.blake3_hash(), bind_pattern),
            callback : func,
        };

        if let Err(_) = self.share.queue.push(PendingTask::Install(install_task)) {
            panic!("Coordinator queue is full!");
        }
        self.share.notify.notify_one();
        None
    }

    fn uninstall(&self) -> Reply {
        if let Err(_) = self.share.queue.push(PendingTask::Uninstall) {
            panic!("Coordinator queue is full!");
        }
        self.share.notify.notify_one();
        None
    }


    
    async fn produce(&self, channel : Par, data : ListParWithRandom, persistent : bool) -> Reply {
        let (tx, rx) = oneshot::channel();
        let produce_task = ProduceTask{
            replier : tx,
            channel : (channel.blake3_hash(), channel),
            data : data,
            persistent : persistent,
        };
        if let Err(_) = self.share.queue.push(PendingTask::Produce(produce_task)) {
            panic!("Coordinator queue is full!");
        }
        self.share.notify.notify_one();
        match rx.await {
            Err(e) => {
                warn!("Unable to send. {}.", e);
                None
            },
            Ok(reply) => reply,
        }
    }

    async fn consume(&self, binds : Vec<(BindPattern, Par)>, body : ParWithRandom, persistent : bool, peek : bool) -> Reply {
        let (tx, rx) = oneshot::channel();
        let consume_task = ConsumeTask{
            replier : tx,
            channels : binds.into_iter().map(|tuple| (tuple.1.blake3_hash(), tuple.0)).collect(),
            continuation : TaggedContinuation::ParBody(body),
            persistent : persistent,
            peek : peek,
        };
        if let Err(_) = self.share.queue.push(PendingTask::Consume(consume_task)) {
            panic!("Coordinator queue is full!");
        }
        self.share.notify.notify_one();
        match rx.await {
            Err(e) => {
                debug!("Unable to receive. Reason {}.", e);
                None
            },
            Ok(reply) => reply,
        }
    }
}


impl<'a> Coordinator {

    pub fn create() -> (AsyncStore, Self) {
        let share = Arc::new(ThreadSafeShare {
            notify : Notify::new(),
            queue : ArrayQueue::new(1000000), 
        });
        
        let coordinator = Self { 
            share : share.clone(),
            transit_port_map : FxHashMap::with_capacity_and_hasher( 100000, Default::default()) 
        };
        let hot_store = AsyncStore { share : share };
        (hot_store, coordinator)
    }

    pub async fn run(&mut self) {
        loop {
            while let Some(pending_task) = self.share.queue.pop() {
                match  pending_task {
                    PendingTask::Install(install) => {
                        self.handle_install(install)
                    },
                    PendingTask::Produce(produce) => {
                        self.handle_produce(produce)
                    },
                    PendingTask::Consume(consume) => {
                        match consume.channels.len() {
                            1 => self.handle_consume_single(consume),
                            2 => self.handle_consume_double(consume),
                            _ => self.handle_consume(consume),
                        } 
                       
                    },
                    PendingTask::Join(join_task) => {
                        match join_task.consumer.channels.len() {
                            2 => self.handle_join_double(join_task),
                            _ => self.handle_join(join_task),
                        } 
                    }
                    PendingTask::Uninstall => {
                        return;
                    }
                }
            }
            self.share.notify.notified().await;
        }
        
    }

    fn get_or_create_transit_port(&mut self, channel_hash : Hash) -> Rc<RefCell<TransitPort>> {
        match self.transit_port_map.entry(channel_hash) {
            Entry::Occupied(o) => o.into_mut().clone(),
            Entry::Vacant(v) => {
                v.insert(Rc::new(RefCell::new(TransitPort::default()))).clone()
            },
        }
    }


    fn handle_install(&mut self, install : InstallTask) {

        // get the transit port of this channel
        let transit_port = self.get_or_create_transit_port(install.channel.0);
        // create a pair of sender + receiver
        let (tx, rx) = oneshot::channel();
        
        // replace receiver which will be signaled when current coroutine completes
        let prev_signal = match transit_port.borrow_mut().completed_signal.replace(rx.into()) {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(Transit::default()) {
                    warn!("prev_tx.send(Transit::default()) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };
        
        task::spawn( async move {
            // first ensure previous coroutines are completed
            let mut transit = match prev_signal.await {
                Err(e) => {
                    error!("Error in oneshot::Receiver<Transit>. {} - {:?}", &e, &e);
                    return;
                },
                Ok(t) => t
            };

            // now handle it
            Transit::install(&mut transit, install);

            // now send the signal
            if let Err(_) = tx.send(transit) {
                error!("tx.send(transit) failed but shouldn't!");
            }
        });

    }


    fn handle_produce(&mut self, produce : ProduceTask) {

        // get the transit port of this channel
        let transit_port = self.get_or_create_transit_port(produce.channel.0);
        
        // replace receiver which will be signaled when current coroutine completes
        let prev_signal = match transit_port.borrow_mut().completed_signal.take() {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(Transit::default()) {
                    warn!("prev_tx.send(Transit::default()) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };

        let cloned_share = self.share.clone();
        
        let current_signal = task::spawn(async move {
            // first ensure previous coroutines are completed
            let mut transit = match prev_signal.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };

            // now handle it
            if let Some(joined_consumer) = Transit::produce(&mut transit, produce) {
                // join is required
                
                if let Err(_) = cloned_share.queue.push(PendingTask::Join(joined_consumer)) {
                    error!("Coordinator queue is full!");
                }
                cloned_share.notify.notify_one();
            }

            transit
        });
        transit_port.borrow_mut().completed_signal = Some(current_signal.into());
    }


    fn handle_consume_single(&mut self, consume_task : ConsumeTask) {

        assert_eq!(consume_task.channels.len(), 1);
        
        let (hash, _) = &consume_task.channels[0];

        // get the transit port of this channel
        let transit_port0 = self.get_or_create_transit_port(*hash);
        let prev_rx0 = match transit_port0.borrow_mut().completed_signal.take() {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(Transit::default()) {
                    warn!("prev_tx.send(Transit::default()) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };


        
        let current_signal = task::spawn( async move {

            let mut transit0 = match prev_rx0.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };

            // now handle it
            {
                Transit::consume_single(&mut transit0, consume_task);
            };

            transit0
        });// task::spawn()
        transit_port0.borrow_mut().completed_signal = Some(current_signal.into());
                
    }// handle_consume_double()


    fn handle_consume_double(&mut self, consume_task : ConsumeTask) {

        assert_eq!(consume_task.channels.len(), 2);
        
        let (hash, _) = &consume_task.channels[0];

        // get the transit port of this channel
        let transit_port0 = self.get_or_create_transit_port(*hash);
        let prev_rx0 = match transit_port0.borrow_mut().completed_signal.take() {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(Transit::default()) {
                    warn!("prev_tx.send(Transit::default()) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };


        let (hash, _) = &consume_task.channels[1];

        // get the transit port of this channel
        let transit_port1 = self.get_or_create_transit_port(*hash);
        let (tx1, rx1) = oneshot::channel();
        let prev_rx1 = match transit_port1.borrow_mut().completed_signal.replace(rx1.into()) {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(Transit::default()) {
                    warn!("prev_tx.send(Transit::default()) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };


        
        let current_signal = task::spawn( async move {

            let mut transit0 = match prev_rx0.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };
            let mut transit1 = match prev_rx1.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };

            // now handle it
            {
                let transits = smallvec![&mut transit0, &mut transit1];
                Transit::consume_multiple(transits, consume_task);
            };

    
            if let Err(_) = tx1.send(transit1) {
                warn!("tx1.send(transit1) failed but it should not!");
            }
            transit0
        });// task::spawn()
        transit_port0.borrow_mut().completed_signal = Some(current_signal.into());
                
    }// handle_consume_double()

    fn handle_consume(&mut self, consume_task : ConsumeTask) {

        let mut rx_tx_pairs = ShortVector::with_capacity(consume_task.channels.len());

        // get all signals
        for (hash, _) in &consume_task.channels {
            // get the transit port of this channel
            let transit_port = self.get_or_create_transit_port(*hash);
            // create a pair of sender + receiver
            let (tx, rx) = oneshot::channel();

            rx_tx_pairs.push(
                // replace receiver which will be signaled when current coroutine completes
                match transit_port.borrow_mut().completed_signal.replace(rx.into()) {
                    Some(signal) => {
                        (signal, tx)
                    },
                    None => {
                        // no previous Receiver, this is a fresh new channel
                        // simulate one
                        let (prev_tx, prev_rx) = oneshot::channel();
                        if let Err(_) = prev_tx.send(Transit::default()) {
                            warn!("prev_tx.send(Transit::default()) must not fail");
                        }
                        (prev_rx.into(), tx)
                    }
                }
            );
        }

        
        task::spawn( async move {


            let mut pairs : ShortVector<(Transit, oneshot::Sender<Transit>)> = ShortVector::with_capacity(rx_tx_pairs.len());
            
    
            for (rx, tx) in rx_tx_pairs {
                match rx.await {
                    Err(e) => {
                        warn!("Error in oneshot::Receiver<Transit>. {} - {:?}", &e, &e);
                        return;
                    },
                    Ok(transit) => {
                        pairs.push((transit, tx));
                    }
                }
            }

            // now handle it
            {
                let transits = pairs.iter_mut().map(|pair| &mut pair.0).collect();
                Transit::consume_multiple(transits, consume_task);
            };

    
            // now send the signals
            for (transit, tx) in pairs {
                if let Err(_) = tx.send(transit) {
                    warn!("tx.send(transit) failed but it should not!");
                }
            }

        });// task::spawn()
                
    }// handle_consume()



    fn handle_join_double(&mut self, join_task : JoinChannelTask) {

        assert_eq!( join_task.consumer.channels.len(), 2);

        let iter = &mut join_task.consumer.channels.iter();
        let hash0 = *(iter.next().unwrap().0);

        let transit_port0 = self.get_or_create_transit_port(hash0);
        let prev_rx0 = match transit_port0.borrow_mut().completed_signal.take() {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(Transit::default()) {
                    warn!("prev_tx.send(Transit::default()) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };

        let hash1 = *iter.next().unwrap().0;

        // get the transit port of this channel
        let transit_port1 = self.get_or_create_transit_port(hash1);
        let (tx1, rx1) = oneshot::channel();
        let prev_rx1 = match transit_port1.borrow_mut().completed_signal.replace(rx1.into()) {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(Transit::default()) {
                    warn!("prev_tx.send(Transit::default()) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };
        drop(iter);

        let current_signal = task::spawn( async move {

            let mut transit0 = match prev_rx0.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };
            let mut transit1 = match prev_rx1.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };

            // now handle it
            {
                let transits : ShortVector<_> = smallvec![(hash0, &mut transit0), (hash1, &mut transit1)];
                Transit::join(transits, join_task);
            };

    
            // now send the signals
            if let Err(_) = tx1.send(transit1) {
                warn!("tx.send(transit) failed but it should not!");
            }
            
            transit0
        });// task::spawn()
        transit_port0.borrow_mut().completed_signal = Some(current_signal.into());
    }


    fn handle_join(&mut self, join_task : JoinChannelTask) {

        let mut tuples = ShortVector::with_capacity(join_task.consumer.channels.len());

        // get all signals
        for (hash, _) in &join_task.consumer.channels {
            // get the transit port of this channel
            let transit_port = self.get_or_create_transit_port(*hash);
            // create a pair of sender + receiver
            let (tx, rx) = oneshot::channel();

            tuples.push(
                // replace receiver which will be signaled when current coroutine completes
                match transit_port.borrow_mut().completed_signal.replace(rx.into()) {
                    Some(signal) => {
                        (*hash, signal, tx)
                    },
                    None => {
                        // no previous Receiver, this is a fresh new channel
                        // simulate one
                        let (prev_tx, prev_rx) = oneshot::channel();
                        if let Err(_) = prev_tx.send(Transit::default()) {
                            warn!("prev_tx.send(Transit::default()) must not fail");
                        }
                        (*hash, prev_rx.into(), tx)
                    }
                }
            );
        }


        task::spawn( async move {

            let mut vector : ShortVector<(Hash, Transit, oneshot::Sender<Transit>)> = ShortVector::with_capacity(tuples.len());
                
        
            for (hash, rx, tx) in tuples {
                match rx.await {
                    Err(e) => {
                        warn!("Error in oneshot::Receiver<Transit>. {} - {:?}", &e, &e);
                        return;
                    },
                    Ok(transit) => {
                        vector.push((hash, transit, tx));
                    }
                }
            }

            // we dont need sort the order of pairs/channels because they are the same
            // but later when storing the consumer, they must be stored
            /*
            // sort pairs & channels to make them in the same order
            pairs.sort_by( |(_,_,left), (_, _, right)| {
                left.as_bytes().cmp(right.as_bytes())
            });
            channels.sort_by( |(_,left), (_, right)| {
                left.as_bytes().cmp(right.as_bytes())
            });
            */

            // now handle it
            {
                let transits : ShortVector<_> = vector.iter_mut().map(|pair| (pair.0, &mut pair.1)).collect();
                Transit::join(transits, join_task);
            };

    
            // now send the signals
            for (_, transit, tx) in vector {
                if let Err(_) = tx.send(transit) {
                    warn!("tx.send(transit) failed but it should not!");
                }
            }
            

        });// task::spawn()

    }

}

