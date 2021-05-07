use std::sync::Arc;
use core::pin::Pin;
use core::task::{Context, Poll};
use tokio::sync::{mpsc::{self, UnboundedReceiver, UnboundedSender}, oneshot::{self, error::RecvError} };
use futures::{Future, StreamExt, stream::futures_unordered::FuturesUnordered };

use super::*;
use super::tuplecell::TupleCell;



pub(super) struct Runner {
    rx : UnboundedReceiver<JoinGroup>,
    futures : FuturesUnordered<JoinGroup>,
    queue : Arc<ThreadSafeQueue>,
}


pub(super) struct JoinGroup {
    receivers : ShortVector<oneshot::Receiver<TupleCell>>,
    inner : Option<JoinedGroup>,
}

pub(super) struct JoinedGroup {
    senders : ShortVector<oneshot::Sender<TupleCell>>,
    cells : ShortVector<TupleCell>,
    pending_task : Option<PendingTask>,
}

impl JoinGroup {
    #[inline]
    pub(super) fn new_single(receiver: oneshot::Receiver<TupleCell>, sender: oneshot::Sender<TupleCell>, pending_task : PendingTask) -> Self {
        Self {
            receivers : smallvec![receiver],
            inner : Some(JoinedGroup {
                senders : smallvec![sender],
                cells : ShortVector::with_capacity(1),
                pending_task : Some(pending_task),
            })
        }
    }

    #[inline]
    pub(super) fn new_multiple(receivers : ShortVector<oneshot::Receiver<TupleCell>>, senders : ShortVector<oneshot::Sender<TupleCell>>, pending_task : PendingTask) -> Self {
        let cells = ShortVector::with_capacity(senders.len());
        Self {
            receivers : receivers,
            inner : Some(JoinedGroup {
                senders : senders,
                cells : cells,
                pending_task : Some(pending_task),
            })
        }
    }
}


impl Future for JoinGroup
{
    type Output = Result<JoinedGroup, RecvError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        debug_assert!(self.inner.is_some());

        let mut len = self.receivers.len();
        while len > 0 {

            let pinned_receiver : Pin<_>;
            // get_mut is better than [index] on performance
            if let Some(receiver) = self.receivers.get_mut(len - 1) {
                pinned_receiver = Pin::new(receiver);
            } else {
                unreachable!("receiver cannot be found");
            }

            let poll_result = pinned_receiver.poll(cx);
            match poll_result {
                Poll::Ready(Ok(cell)) => {
                    // removing from the front of a vec is an O(n) operation, whereas pop/push is O(1).
                    self.receivers.pop(); // consider reuse the receiver
                    if let Some(inner) = &mut self.inner {
                        inner.cells.push(cell);
                    } else {
                        unreachable!("JoinGroup::inner is None!");
                    }
                    len -= 1;
                },
                Poll::Ready(Err(e)) => {
                    return Poll::Ready(Err(e));
                },
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }

        // we need reverse the order 
        if let Some(mut inner) = self.inner.take() {
            inner.cells.reverse();
            return Poll::Ready(Ok(inner));
        } else {
            unreachable!("JoinGroup::inner is None!");
        }
    }
}

impl Runner {

    pub(super) fn start(queue : Arc<ThreadSafeQueue>) -> UnboundedSender<JoinGroup> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        tokio::task::spawn( async move {
            let instance = Self {
                rx : rx,
                futures : FuturesUnordered::default(),
                queue : queue,
            };
            instance.run().await
        });

        tx
    }

    async fn run(mut self) {
        loop {
            tokio::select! {
                Some(result) = self.futures.next() => {
                    match result {
                        Err(e) => {
                            error!("RecvError should not happen! {}", e);
                        },
                        Ok(joined_group) => { 
                            self.handle(joined_group);
                        }
                    }
                },
                result = self.rx.recv() => {
                    if let Some(join_group) = result {
                        self.futures.push(join_group);
                    } else {
                        return; // sender dropped
                    }
                }
            }
        }
    }

    fn handle(&self, mut joined_group : JoinedGroup) {
        debug_assert_eq!( joined_group.senders.len(), joined_group.cells.len());

        match  joined_group.pending_task.take() {
            Some(PendingTask::Install(install)) => {
                debug_assert_eq!( joined_group.cells.len(), 1);
                
                if let Some(cell) = joined_group.cells.get_mut(0) {
                    TupleCell::install( cell, install);
                } else {
                    unreachable!("Produce must conntain only one target");
                }

            },
            Some(PendingTask::Produce(produce)) => {
                debug_assert_eq!( joined_group.cells.len(), 1);

                if let Some(cell) = joined_group.cells.get_mut(0) {
                    if let Some(joined_consumer) = TupleCell::produce( cell, produce) {
                        // join is required
                        self.queue.push(PendingTask::Join(joined_consumer));
                    }
                } else {
                    unreachable!("Produce must conntain only one target");
                }
            },
            Some(PendingTask::Consume(consume)) => {

                let cells = joined_group.cells.iter_mut().collect();
                TupleCell::consume_multiple(cells, consume);
            },
            Some(PendingTask::Join(join_task)) => {
                let cells : ShortVector<_> = joined_group.cells.iter_mut().collect();
                TupleCell::join(cells, join_task);
            }
            Some(PendingTask::Uninstall) => {
                unreachable!("PendingTask::Uninstall")
            },
            None => {
                unreachable!("JoinedGroup::pending_task is None.")
            }
        }


        while !joined_group.senders.is_empty() {
            let sender = joined_group.senders.pop().unwrap();
            let cell = joined_group.cells.pop().unwrap();
            if let Err(_) = sender.send(cell) {
                error!("Sending failed. This is usually the receiver dropped.");
            }
        }
    }
}