use std::collections::hash_map::Entry;
use std::cell::{ RefCell };
use std::rc::Rc;
use std::sync::Arc;
use tokio::task;
use tokio::sync::Notify;

use rustc_hash::{ FxHashMap };

use super::*;
use crossbeam::queue::ArrayQueue;
use blake3::Hash;

enum PendingTask{
    Install(InstallTask),
    Uninstall,
    Produce(ProduceTask),
    Consume(ConsumeTask),
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

struct ThreadSafeShare {
    notify : Notify,
    queue : ArrayQueue<PendingTask>,
}

pub struct Coordinator {
    share : Arc<ThreadSafeShare>,
    transit_port_map : FxHashMap<Hash, Rc<RefCell<TransitPort>>>
}

#[derive(Default)]
pub struct TransitPort {
    pub(super) completed_signal : Option<oneshot::Receiver<Transit>>,
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
                warn!("Unable to receive. Reason {}.", e);
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
                        self.handle_consume(consume);
                    },
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
        let prev_signal = match transit_port.borrow_mut().completed_signal.replace(rx) {
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
                prev_rx
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
        // create a pair of sender + receiver
        let (tx, rx) = oneshot::channel();
        
        // replace receiver which will be signaled when current coroutine completes
        let prev_signal = match transit_port.borrow_mut().completed_signal.replace(rx) {
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
                prev_rx
            }
        };
        
        task::spawn( async move {
            // first ensure previous coroutines are completed
            let mut transit = match prev_signal.await {
                Err(e) => {
                    warn!("Error in oneshot::Receiver<Transit>. {} - {:?}", &e, &e);
                    return;
                },
                Ok(t) => t
            };

            // now handle it
            Transit::produce(&mut transit, produce);

            // now send the signal
            if let Err(_) = tx.send(transit) {
                warn!("tx.send(transit) failed but shouldn't!");
            }
        });

    }

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
                match transit_port.borrow_mut().completed_signal.replace(rx) {
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
                        (prev_rx, tx)
                    }
                }
            );
        }

        
        task::spawn( async move {

            if rx_tx_pairs.len() == 1 { // optimize for frequent execution path

                let (rx, tx) = rx_tx_pairs.pop().unwrap();
                let mut transit = match rx.await {
                    Err(e) => {
                        warn!("Error in oneshot::Receiver<Transit>. {} - {:?}", &e, &e);
                        return;
                    },
                    Ok(t) => t
                };

                Transit::consume_single(&mut transit, consume_task);

                
                if let Err(_) = tx.send(transit) {
                    warn!("tx.send(transit) failed but it should not!");
                }

            } 
            else {
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
                    let transits = pairs.iter_mut().map(|pair| &mut pair.0).collect();
                    Transit::consume_multiple(transits, consume_task);
                };

        
                // now send the signals
                for (transit, tx) in pairs {
                    if let Err(_) = tx.send(transit) {
                        warn!("tx.send(transit) failed but it should not!");
                    }
                }
            }// if_else
            

        });// task::spawn()
                
    }// handle_consume()


}

