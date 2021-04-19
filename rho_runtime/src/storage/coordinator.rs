use std::collections::hash_map::Entry;
use std::cell::{ RefCell };
use std::rc::Rc;
use tokio::task;
use tokio::sync::oneshot::{ self };
use rustc_hash::{ FxHashMap };

use super::*;
use crossbeam::sync::ShardedLock;
use tokio::sync::mpsc::{ self, Sender, Receiver };
use blake3::Hash;

enum PendingTask{
    Produce(ProduceTask),
    Consume(ConsumeTask),
}

pub(super) type ProduceChannel = (Hash, Par);

#[derive(Debug)]
pub(super) struct ProduceTask {
    pub(super) id : u64, // increment id
    pub(super) channel : ProduceChannel,
    pub(super) data : ListParWithRandom,
    pub(super) persistent : bool,
}

pub(super) type ConsumeChannel = (Hash, BindPattern, Par);

#[derive(Debug)]
pub(super) struct ConsumeTask {
    pub(super) id : u64, // increment id
    pub(super) channels : Vec<ConsumeChannel>,
    pub(super) body : ParWithRandom,
    pub(super) persistent : bool,
    pub(super) peek : bool,
}

pub struct Coordinator {
    _tx: Sender<PendingTask>, // keep an instance here so that it will not close if no other senders
    rx : Receiver<PendingTask>,
    transit_port_map : FxHashMap<Hash, Rc<RefCell<TransitPort>>>,
    id_base : u64,
}

pub struct  AsyncStore {
    tx : ShardedLock<Sender<PendingTask>>,
}

#[async_trait]
impl Storage for AsyncStore {
    
    async fn produce(&self, channel : Par, data : ListParWithRandom, persistent : bool){
        let produce_task = ProduceTask{
            id : 0, // to be updated on receiving
            channel : (channel.blake3_hash(), channel),
            data : data,
            persistent : persistent,
        };
        let sender = self.tx.read().unwrap().clone();
        if let Err(err) = sender.send(PendingTask::Produce(produce_task)).await {
            panic!("HotStore::produce failed. {}", &err);
        }
    }

    async fn consume(&self, binds : Vec<(BindPattern, Par)>, body : ParWithRandom, persistent : bool, peek : bool) {
        let consume_task = ConsumeTask{
            id : 0, // to be updated on receiving
            channels : binds.into_iter().map(|tuple| (tuple.1.blake3_hash(), tuple.0, tuple.1)).collect(),
            body : body,
            persistent : persistent,
            peek : peek,
        };
        let sender = self.tx.read().unwrap().clone();
        if let Err(err) = sender.send(PendingTask::Consume(consume_task)).await {
            panic!("HotStore::produce failed. {}", &err);
        }
    }
}


impl Coordinator {

    pub fn create() -> (AsyncStore, Self) {
        let (tx, rx) : (Sender<PendingTask>, Receiver<PendingTask>) = mpsc::channel(1000);
        let hot_store = AsyncStore { tx : ShardedLock::new(tx.clone()) };
        let coordinator = Self { rx : rx, _tx : tx, transit_port_map : FxHashMap::default(), id_base : 0 };
        (hot_store, coordinator)
    }

    pub async fn run(&mut self) {
        while let Some(pending_task) = self.rx.recv().await {
            self.id_base += 1; // assign an increment id to each received message
            match  pending_task {
                PendingTask::Produce(mut produce) => {
                    produce.id = self.id_base;
                    self.handle_produce(produce)
                },
                PendingTask::Consume(mut consume) => {
                    consume.id = self.id_base;
                    self.handle_consume(consume);
                },
                _ => unreachable!("Bug, this branch must not reach"),
            }
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


    fn handle_produce(&mut self, produce : ProduceTask) {
        //println!("handle_produce {:?}", &produce);
/* 
        let transit_port = self.get_or_create_transit_port(produce.channel.0);
        transit_port.unprocessed_produces.get_or_insert_with(|| ShortVector::new()).push(produce);

        match transit_port.cargo {
            None => { 
                // there is already a coroutine working on it
            },
            Some(ref cargo) => { // no corouting working on this one

            },
            
        }*/

    }

    fn handle_consume(&mut self, consume : ConsumeTask) {

        let mut signals = ShortVector::new();
        let mut hash_map = FxHashMap::default();

        // get all signals
        for tuple in consume.channels {
            // get the transit port of this channel
            let transit_port = self.get_or_create_transit_port(tuple.0);
            // create a pair of sender + receiver
            let (tx, rx) = oneshot::channel();
            // replace receiver which will be signaled when current coroutine completes
            if let Some(signal) = transit_port.borrow_mut().signal.replace(rx) {
                signals.push(signal); // save the previous Receiver
            }
            hash_map.insert(tuple.0, (tuple.1, tuple.2, tx));
        }
        
        task::spawn( async move {
            // first ensure no other coroutines are operating these channels
            for signal in signals {
                signal.await.unwrap(); // should never fail
            }
        });
        
        //println!("handle_consume {:?}", &join_group);

        
    }


}

