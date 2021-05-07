
use std::sync::Arc;

use super::*;



#[derive(Clone)]
pub struct  AsyncStore {
    queue : Arc<ThreadSafeQueue>,
}

impl  AsyncStore {
    pub(super) fn new(queue : Arc<ThreadSafeQueue>) -> Self {
        Self {
            queue : queue
        }
    }
}

impl Storage for AsyncStore {

    

    fn install(&self, channel : Par, bind_pattern : BindPattern, func : RustCallbacFunction) -> Reply {
        let install_task = InstallTask{
            channel : (channel.blake3_hash(), bind_pattern),
            callback : func,
        };

        self.queue.push(PendingTask::Install(install_task));
        None
    }


    fn uninstall(&self) -> Reply {
        self.queue.push(PendingTask::Uninstall);
        None
    }


    #[inline]
    fn produce(&self, channel : Par, data : ListParWithRandom, persistent : bool) -> oneshot::Receiver<Reply> {
        let (tx, rx) = oneshot::channel();
        let produce_task = ProduceTask{
            replier : tx,
            channel : (channel.blake3_hash(), channel),
            data : data,
            persistent : persistent,
        };
        self.queue.push(PendingTask::Produce(produce_task));
        rx
    }

    #[inline]
    fn consume(&self, binds : Vec<(BindPattern, Par)>, body : ParWithRandom, persistent : bool, peek : bool) -> oneshot::Receiver<Reply> {
        let (tx, rx) = oneshot::channel();
        let consume_task = ConsumeTask{
            replier : tx,
            channels : binds.into_iter().map(|tuple| (tuple.1.blake3_hash(), tuple.0)).collect(),
            continuation : TaggedContinuation::ParBody(body),
            persistent : persistent,
            peek : peek,
        };
        self.queue.push(PendingTask::Consume(consume_task));
        rx
    }
}