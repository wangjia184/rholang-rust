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

pub(super) enum PendingTask{
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
    pub(super) consumer : Arc<super::tuplecell::SharedJoinedConsumer>,
}

pub(super) struct ThreadSafeQueue {
    notifier : Notify,
    inner : ArrayQueue<PendingTask>,
}

impl Default for ThreadSafeQueue {
    #[inline]
    fn default() -> Self {
        Self {
            notifier : Notify::new(),
            inner : ArrayQueue::new(1000000), 
        }
    }
}

impl ThreadSafeQueue {
    #[inline]
    pub(super) fn push(&self, msg : PendingTask) {
        if let Err(_) = self.inner.push(msg) {
            panic!("Coordinator queue is full!");
        }
        self.notifier.notify_one();
    }
}


pub struct Coordinator {
    queue : Arc<ThreadSafeQueue>,
    cell_slot_map : FxHashMap<Hash, Rc<RefCell<TupleCellSlot>>>
}


#[derive(Default)]
struct TupleCellSlot {
    completed_signal : Option<CompletionSignal>,
}



impl<'a> Coordinator {

    pub fn create() -> (AsyncStore, Self) {
        let queue = Arc::new(ThreadSafeQueue::default());
        
        let coordinator = Self { 
            queue : queue.clone(),
            cell_slot_map : FxHashMap::with_capacity_and_hasher( 100000, Default::default()) 
        };
        let hot_store = AsyncStore::new(queue);
        (hot_store, coordinator)
    }

    pub async fn run(&mut self) {
        loop {
            while let Some(pending_task) = self.queue.inner.pop() {
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
            self.queue.notifier.notified().await;
        }
        
    }

    #[inline]
    fn get_or_create_cell_slot(&mut self, channel_hash : Hash) -> Rc<RefCell<TupleCellSlot>> {
        match self.cell_slot_map.entry(channel_hash) {
            Entry::Occupied(o) => o.into_mut().clone(),
            Entry::Vacant(v) => {
                v.insert(Rc::new(RefCell::new(TupleCellSlot::default()))).clone()
            },
        }
    }


    fn handle_install(&mut self, install : InstallTask) {

        // get the cell_slot of this channel
        let cell_slot = self.get_or_create_cell_slot(install.channel.0);
        // create a pair of sender + receiver
        let (tx, rx) = oneshot::channel();
        
        // replace receiver which will be signaled when current coroutine completes
        let prev_signal = match cell_slot.borrow_mut().completed_signal.replace(rx.into()) {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TuplespaceChannel::new(install.channel.0)) {
                    warn!("prev_tx.send(TupleCell::new(install.channel.0)) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };
        
        task::spawn( async move {
            // first ensure previous coroutines are completed
            let mut cell = match prev_signal.await {
                Err(e) => {
                    error!("Error in oneshot::Receiver<TupleCell>. {} - {:?}", &e, &e);
                    return;
                },
                Ok(t) => t
            };

            // now handle it
            TuplespaceChannel::install(cell.borrow_mut(), install);

            // now send the signal
            if let Err(_) = tx.send(cell) {
                error!("tx.send(cell) failed but shouldn't!");
            }
        });

    }


    fn handle_produce(&mut self, produce : ProduceTask) {

        // get the cell_slot of this channel
        let cell_slot = self.get_or_create_cell_slot(produce.channel.0);
        
        // replace receiver which will be signaled when current coroutine completes
        let prev_signal = match cell_slot.borrow_mut().completed_signal.take() {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TuplespaceChannel::new(produce.channel.0)) {
                    warn!("prev_tx.send(TupleCell::new(produce.channel.0)) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };

        let cloned_share = self.queue.clone();
        
        let current_signal = task::spawn(async move {
            // first ensure previous coroutines are completed
            let mut cell = match prev_signal.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };

            // now handle it
            if let Some(joined_consumer) = TuplespaceChannel::produce(cell.borrow_mut(), produce) {
                // join is required
                
                if let Err(_) = cloned_share.inner.push(PendingTask::Join(joined_consumer)) {
                    error!("Coordinator queue is full!");
                }
                cloned_share.notifier.notify_one();
            }

            cell
        });
        cell_slot.borrow_mut().completed_signal = Some(current_signal.into());
    }


    fn handle_consume_single(&mut self, consume_task : ConsumeTask) {

        assert_eq!(consume_task.channels.len(), 1);
        
        let (hash, _) = &consume_task.channels[0];

        // get the cell_slot0 of this channel
        let cell_slot0 = self.get_or_create_cell_slot(*hash);
        let prev_rx0 = match cell_slot0.borrow_mut().completed_signal.take() {
            Some(prev_rx) => {
                prev_rx
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TuplespaceChannel::new(*hash)) {
                    warn!("prev_tx.send(TupleCell::new(*hash)) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };


        
        let current_signal = task::spawn( async move {

            let mut cell0 = match prev_rx0.await {
                Err(e) => {
                    panic!("{} - {:?}", &e, &e);
                },
                Ok(t) => t
            };

            // now handle it
            {
                TuplespaceChannel::consume_single(cell0.borrow_mut(), consume_task);
            };

            cell0
        });// task::spawn()
        cell_slot0.borrow_mut().completed_signal = Some(current_signal.into());
                
    }// handle_consume_double()


    fn handle_consume_double(&mut self, consume_task : ConsumeTask) {

        assert_eq!(consume_task.channels.len(), 2);
        
        let (hash, _) = &consume_task.channels[0];

        // get the cell slot of this channel
        let cell_slot0 = self.get_or_create_cell_slot(*hash);
        let prev_rx0 = match cell_slot0.borrow_mut().completed_signal.take() {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TuplespaceChannel::new(*hash)) {
                    warn!("prev_tx.send(TupleCell::new(*hash))  failed but shouldn't!");
                }
                prev_rx.into()
            }
        };


        let (hash, _) = &consume_task.channels[1];

        // get the cell slot of this channel
        let cell_slot1 = self.get_or_create_cell_slot(*hash);
        let (tx1, rx1) = oneshot::channel();
        let prev_rx1 = match cell_slot1.borrow_mut().completed_signal.replace(rx1.into()) {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TuplespaceChannel::new(*hash)) {
                    warn!("prev_tx.send(TupleCell::new(*hash)) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };


        
        let current_signal = task::spawn( async move {

            let (result0, result1) = tokio::join!( prev_rx0, prev_rx1);

            let mut cell0 = result0.unwrap();
            let mut cell1 = result1.unwrap();

            // now handle it
            {
                let cells = smallvec![cell0.borrow_mut(), cell1.borrow_mut()];
                TuplespaceChannel::consume_multiple(cells, consume_task);
            };

    
            if let Err(_) = tx1.send(cell1) {
                warn!("tx1.send(cell1) failed but it should not!");
            }
            cell0
        });// task::spawn()
        cell_slot0.borrow_mut().completed_signal = Some(current_signal.into());
                
    }// handle_consume_double()

    fn handle_consume(&mut self, consume_task : ConsumeTask) {

        let mut rx_tx_pairs = ShortVector::with_capacity(consume_task.channels.len());

        // get all signals
        for (hash, _) in &consume_task.channels {
            // get the cell_slot of this channel
            let cell_slot = self.get_or_create_cell_slot(*hash);
            // create a pair of sender + receiver
            let (tx, rx) = oneshot::channel();

            rx_tx_pairs.push(
                // replace receiver which will be signaled when current coroutine completes
                match cell_slot.borrow_mut().completed_signal.replace(rx.into()) {
                    Some(signal) => {
                        (signal, tx)
                    },
                    None => {
                        // no previous Receiver, this is a fresh new channel
                        // simulate one
                        let (prev_tx, prev_rx) = oneshot::channel();
                        if let Err(_) = prev_tx.send(TuplespaceChannel::new(*hash)) {
                            warn!("prev_tx.send(TupleCell::new(*hash)) must not fail");
                        }
                        (prev_rx.into(), tx)
                    }
                }
            );
        }

        
        task::spawn( async move {


            let mut pairs : ShortVector<(TupleCell, oneshot::Sender<TupleCell>)> = ShortVector::with_capacity(rx_tx_pairs.len());
            
    
            for (rx, tx) in rx_tx_pairs {
                match rx.await {
                    Err(e) => {
                        warn!("Error in oneshot::Receiver<TupleCell>. {} - {:?}", &e, &e);
                        return;
                    },
                    Ok(cell) => {
                        pairs.push((cell, tx));
                    }
                }
            }

            // now handle it
            {
                let cells = pairs.iter_mut().map(|pair| pair.0.borrow_mut()).collect();
                TuplespaceChannel::consume_multiple(cells, consume_task);
            };

    
            // now send the signals
            for (cell, tx) in pairs {
                if let Err(_) = tx.send(cell) {
                    warn!("tx.send(cell) failed but it should not!");
                }
            }

        });// task::spawn()
                
    }// handle_consume()



    fn handle_join_double(&mut self, join_task : JoinChannelTask) {

        assert_eq!( join_task.consumer.channels.len(), 2);

        let iter = &mut join_task.consumer.channels.iter();
        let hash0 = *(iter.next().unwrap().0);

        let cell_slot0 = self.get_or_create_cell_slot(hash0);
        let prev_rx0 = match cell_slot0.borrow_mut().completed_signal.take() {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TuplespaceChannel::new(hash0)) {
                    warn!("prev_tx.send(TupleCell::new(hash0)) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };

        let hash1 = *iter.next().unwrap().0;

        // get the cell_slot of this channel
        let cell_slot1 = self.get_or_create_cell_slot(hash1);
        let (tx1, rx1) = oneshot::channel();
        let prev_rx1 = match cell_slot1.borrow_mut().completed_signal.replace(rx1.into()) {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TuplespaceChannel::new(hash1)) {
                    warn!("prev_tx.send(TupleCell::new(hash1)) failed but shouldn't!");
                }
                prev_rx.into()
            }
        };
        drop(iter);

        let current_signal = task::spawn( async move {

            let (result0, result1) = tokio::join!( prev_rx0, prev_rx1);

            let mut cell0 = result0.unwrap();
            let mut cell1 = result1.unwrap();

            // now handle it
            {
                let cells : ShortVector<_> = smallvec![cell0.borrow_mut(), cell1.borrow_mut()];
                TuplespaceChannel::join(cells, join_task);
            };

    
            // now send the signals
            if let Err(_) = tx1.send(cell1) {
                warn!("tx1.send(cell1) failed but it should not!");
            }
            
            cell0
        });// task::spawn()
        cell_slot0.borrow_mut().completed_signal = Some(current_signal.into());
    }


    fn handle_join(&mut self, join_task : JoinChannelTask) {

        let mut tuples = ShortVector::with_capacity(join_task.consumer.channels.len());

        // get all signals
        for (hash, _) in &join_task.consumer.channels {
            // get the cell slot of this channel
            let cell_slot = self.get_or_create_cell_slot(*hash);
            // create a pair of sender + receiver
            let (tx, rx) = oneshot::channel();

            tuples.push(
                // replace receiver which will be signaled when current coroutine completes
                match cell_slot.borrow_mut().completed_signal.replace(rx.into()) {
                    Some(signal) => {
                        (signal, tx)
                    },
                    None => {
                        // no previous Receiver, this is a fresh new channel
                        // simulate one
                        let (prev_tx, prev_rx) = oneshot::channel();
                        if let Err(_) = prev_tx.send(TuplespaceChannel::new(*hash)) {
                            warn!("prev_tx.send(TupleCell::new(*hash)) must not fail");
                        }
                        (prev_rx.into(), tx)
                    }
                }
            );
        }


        task::spawn( async move {

            let mut vector : ShortVector<(TupleCell, oneshot::Sender<TupleCell>)> = ShortVector::with_capacity(tuples.len());
                
        
            for (rx, tx) in tuples {
                match rx.await {
                    Err(e) => {
                        warn!("Error in oneshot::Receiver<TupleCell>. {} - {:?}", &e, &e);
                        return;
                    },
                    Ok(cell) => {
                        vector.push((cell, tx));
                    }
                }
            }



            // now handle it
            {
                let cells : ShortVector<_> = vector.iter_mut().map(|pair| pair.0.borrow_mut()).collect();
                TuplespaceChannel::join(cells, join_task);
            };

    
            // now send the signals
            for (cell, tx) in vector {
                if let Err(_) = tx.send(cell) {
                    warn!("tx.send(cell) failed but it should not!");
                }
            }
            

        });// task::spawn()

    }

}

