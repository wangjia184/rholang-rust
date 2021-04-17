use super::*;
use crossbeam::sync::ShardedLock;
use tokio::sync::mpsc::{ self, Sender, Receiver };


pub(super) enum PendingTask{
    Produce(ProduceTask),
}

pub(super) struct ProduceTask {
    pub(super) channel_hash : blake3::Hash,
    pub(super) channel : Par,
    pub(super) persist : bool,
}

pub struct Dispatcher {
    tx: Sender<PendingTask>,
    rx : Receiver<PendingTask>,
}

pub struct  HotStore {
    tx : ShardedLock<Sender<PendingTask>>,
}

#[async_trait]
impl Storage for HotStore {
    
    async fn produce(&self, channel : Par, data : ListParWithRandom, persist : bool){
        let produce_task = ProduceTask{
            channel_hash : channel.blake3_hash(),
            channel : channel,
            persist : persist,
        };
        let sender = self.tx.read().unwrap().clone();
        if let Err(err) = sender.send(PendingTask::Produce(produce_task)).await {
            panic!("HotStore::produce failed. {}", &err);
        }
    }
}


impl Dispatcher {

    pub fn create() -> (HotStore, Self) {
        let (tx, rx) : (Sender<PendingTask>, Receiver<PendingTask>) = mpsc::channel(1000);
        let hot_store = HotStore { tx : ShardedLock::new(tx.clone()) };
        let dispatcher = Self { rx : rx, tx : tx };
        (hot_store, dispatcher)
    }

    pub async fn run(&mut self) {
        while let Some(pending_task) = self.rx.recv().await {
            match  pending_task {
                PendingTask::Produce(produce) => self.handle_produce(produce).await,
                _ => unreachable!("Bug, this branch must not reach"),
            }
        }
    }


    async fn handle_produce(&mut self, produce : ProduceTask) {
        println!("handle_produce {}", &produce.channel_hash.to_hex());
    }


}

