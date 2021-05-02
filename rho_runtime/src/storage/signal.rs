
use super::*;
use thiserror::Error;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::Future;
use tokio::task::{JoinError, JoinHandle};
use tokio::sync::oneshot::error::RecvError;

// The result after coroutine execution
pub(super) enum CompletionSignal {
    // if coroutine only gains one Transit, its return value is the Transit
    JoinHandle(JoinHandle<Transit>),
    // when there are multiple Transits joined in a coroutine execution
    OneshotReceiver(oneshot::Receiver<Transit>),
}

impl From<oneshot::Receiver<Transit>> for CompletionSignal {
    #[inline]
    fn from(receiver : oneshot::Receiver<Transit>) -> Self {
        Self::OneshotReceiver(receiver)
    }
}

impl From<JoinHandle<Transit>> for CompletionSignal {
    #[inline]
    fn from(join_handle : JoinHandle<Transit>) -> Self {
        Self::JoinHandle(join_handle)
    }
}

#[derive(Error, Debug)]
pub(super) enum SignalError {
    #[error("Fault to retrieve Transit from oneshot::Receiver. {0}")]
    OneshotRecvError(RecvError),

    #[error("Fault to retrieve Transit from JoinHandle. {0}")]
    JoinError(JoinError),
}



impl Future for CompletionSignal
{
    type Output = Result<Transit, SignalError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            match self.get_unchecked_mut() {
                CompletionSignal::OneshotReceiver(receiver) => {
                    let pinned = Pin::new_unchecked(receiver);
                    pinned.poll(cx).map_err( |e| SignalError::OneshotRecvError(e) )
                },
                CompletionSignal::JoinHandle(join_handle) => {
                    let pinned = Pin::new_unchecked(join_handle);
                    pinned.poll(cx).map_err( |e| SignalError::JoinError(e) )
                },
            }
        }
    }
}