
use std::{collections::hash_map::Entry};
use std::cell::{ RefCell };
use std::rc::Rc;
use std::sync::Arc;
use std::cmp;
use num_cpus;
use tokio::sync::Notify;
use crossbeam::queue::ArrayQueue;
use tokio::sync::mpsc::UnboundedSender;

use rustc_hash::{ FxHashMap };

use super::*;
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

// This queue allows other coroutines to send messages to Coordinator
pub(super) struct ThreadSafeQueue {
    notify : Notify,
    inner : ArrayQueue<PendingTask>,
}

impl  ThreadSafeQueue {
    #[inline]
    pub(super) fn push(&self, pending_task : PendingTask) {
        if let Err(_) = self.inner.push(pending_task) {
            panic!("Coordinator queue is full!");
        }
        self.notify.notify_one();
    }
}

pub struct Coordinator {
    queue : Arc<ThreadSafeQueue>,
    runner_index : usize,
    runner_senders : Vec<UnboundedSender<JoinGroup>>,
    cell_slot_map : FxHashMap<Hash, Rc<RefCell<CellSlot>>>
}

#[derive(Default)]
pub struct CellSlot {
    pub(super) completed_signal : Option<oneshot::Receiver<TupleCell>>,
}



impl<'a> Coordinator {

    #[inline]
    fn send_to_runner(&mut self, join_group : JoinGroup) {
        // round robin
        self.runner_index += 1;
        let idx = self.runner_index % self.runner_senders.len();
        if let Some(tx) = self.runner_senders.get_mut(idx) {
            if let Err(e) = tx.send(join_group) {
                warn!("Runner exited. {}", e);
                return;
            }
        } else {
            unreachable!("runner_senders must not be empty");
        }
        
    }

    pub fn create() -> (AsyncStore, Self) {
        
        let queue = Arc::new(ThreadSafeQueue {
            notify : Notify::new(),
            inner : ArrayQueue::new(1000000), 
        });

        let mut runner_senders = Vec::with_capacity(cmp::max( num_cpus::get() / 1 /* to be tuned */, 1));
        for _ in 0..runner_senders.capacity() {
            runner_senders.push(Runner::start(queue.clone()));
        }
        
        let coordinator = Self { 
            runner_index : 0,
            runner_senders : runner_senders, 
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
                        self.handle_consume(consume);
                    },
                    PendingTask::Join(join_task) => {
                        self.handle_join(join_task);
                    }
                    PendingTask::Uninstall => {
                        return;
                    }
                }
            }
            self.queue.notify.notified().await;
        }
        
    }

    fn get_or_create_cell_slot(&mut self, channel_hash : Hash) -> Rc<RefCell<CellSlot>> {
        match self.cell_slot_map.entry(channel_hash) {
            Entry::Occupied(o) => o.into_mut().clone(),
            Entry::Vacant(v) => {
                v.insert(Rc::new(RefCell::new(CellSlot::default()))).clone()
            },
        }
    }


    fn handle_install(&mut self, install : InstallTask) {

        // get the cell port of this channel
        let cell_slot = self.get_or_create_cell_slot(install.channel.0);
        // create a pair of sender + receiver
        let (tx, rx) = oneshot::channel();
        
        // replace receiver which will be signaled when current coroutine completes
        let prev_rx = match cell_slot.borrow_mut().completed_signal.replace(rx) {
            Some(prev_rx) => {
                prev_rx
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TupleCell::new(install.channel.0)) {
                    warn!("prev_tx.send(TupleCell::new(install.channel.0)) failed but shouldn't!");
                }
                prev_rx
            }
        };

        let join_group = JoinGroup::new_single(prev_rx, tx, PendingTask::Install(install));
        self.send_to_runner(join_group);
    }


    fn handle_produce(&mut self, produce : ProduceTask) {

        // get the cell_slot of this channel
        let cell_slot = self.get_or_create_cell_slot(produce.channel.0);
        // create a pair of sender + receiver
        let (tx, rx) = oneshot::channel();
        
        // replace receiver which will be signaled when current coroutine completes
        let prev_signal = match cell_slot.borrow_mut().completed_signal.replace(rx) {
            Some(signal) => {
                signal
            },
            None => {
                // no previous Receiver, this is a fresh new channel
                // simulate one
                let (prev_tx, prev_rx) = oneshot::channel();
                if let Err(_) = prev_tx.send(TupleCell::new(produce.channel.0)) {
                    warn!("prev_tx.send(TupleCell::new(produce.channel.0)) failed but shouldn't!");
                }
                prev_rx
            }
        };

        let join_group = JoinGroup::new_single(prev_signal, tx, PendingTask::Produce(produce));
        self.send_to_runner(join_group);
 
    }

    fn handle_consume(&mut self, consume : ConsumeTask) {

        let len = consume.channels.len();
        let mut receivers : ShortVector<oneshot::Receiver<TupleCell>> = ShortVector::with_capacity(len);
        let mut senders = ShortVector::with_capacity(len);

        // get all signals
        for (hash, _) in &consume.channels {
            // get the cell slot of this channel
            let cell_slot = self.get_or_create_cell_slot(*hash);
            // create a pair of sender + receiver
            let (tx, rx) = oneshot::channel();

            // replace receiver which will be signaled when current coroutine completes
            match cell_slot.borrow_mut().completed_signal.replace(rx) {
                Some(prev_rx) => {
                    receivers.push(prev_rx);
                    senders.push(tx);
                },
                None => {
                    // no previous Receiver, this is a fresh new channel
                    // simulate one
                    let (prev_tx, prev_rx) = oneshot::channel();
                    if let Err(_) = prev_tx.send(TupleCell::new(*hash)) {
                        warn!("prev_tx.send(TupleCell::new(*hash)) must not fail");
                    }
                    receivers.push(prev_rx);
                    senders.push(tx);
                }
            };
        }

        let join_group = JoinGroup::new_multiple(receivers, senders, PendingTask::Consume(consume));
        self.send_to_runner(join_group);
                
    }// handle_consume()

    fn handle_join(&mut self, join_task : JoinChannelTask) {

        let len = join_task.consumer.channels.len();
        let mut receivers : ShortVector<oneshot::Receiver<TupleCell>> = ShortVector::with_capacity(len);
        let mut senders = ShortVector::with_capacity(len);

        // get all signals
        for (hash, _) in &join_task.consumer.channels {
            // get the cell_slot of this channel
            let cell_slot = self.get_or_create_cell_slot(*hash);
            // create a pair of sender + receiver
            let (tx, rx) = oneshot::channel();

            // replace receiver which will be signaled when current coroutine completes
            match cell_slot.borrow_mut().completed_signal.replace(rx) {
                Some(prev_rx) => {
                    receivers.push(prev_rx);
                    senders.push(tx);
                },
                None => {
                    // no previous Receiver, this is a fresh new channel
                    // simulate one
                    let (prev_tx, prev_rx) = oneshot::channel();
                    if let Err(_) = prev_tx.send(TupleCell::new(*hash)) {
                        warn!("prev_tx.send(TupleCell::new(*hash)) must not fail");
                    }
                    receivers.push(prev_rx);
                    senders.push(tx);
                }
            };
        }

        let join_group = JoinGroup::new_multiple(receivers, senders, PendingTask::Join(join_task));
        self.send_to_runner(join_group);

    }

}

