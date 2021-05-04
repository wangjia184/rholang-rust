use tokio::sync::oneshot::{Receiver, error::RecvError};
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{Future, StreamExt, stream::futures_unordered::FuturesUnordered };

use super::*;
use super::transit::Transit;



pub(super) struct Congregator {

    futures : FuturesUnordered<JoinGroup>,
}

impl Default for Congregator {
    fn default() -> Self {
        Congregator {
            futures : FuturesUnordered::default(),
        }
    }
}



pub(super) struct JoinGroup {
    receivers : ShortVector<Receiver<Transit>>,
    transits : ShortVector<Transit>,
}


impl Future for JoinGroup
{
    type Output = Result<ShortVector<Transit>, RecvError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let mut len = self.receivers.len();
        while len > 0 {
            let receiver = Pin::new(&mut self.receivers[len - 1]);
            let poll_result = receiver.poll(cx);
            match poll_result {
                Poll::Ready(Ok(transit)) => {
                    self.receivers.pop(); // consider recycle the receiver for reuse
                    self.transits.push(transit);
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
        
        let mut transits = ShortVector::with_capacity(self.transits.len());
        transits.append(&mut self.transits);

        return Poll::Ready(Ok(transits));
    }
}

impl Congregator {
    async fn run(&mut self) {
        loop {
            while let Some(result) = self.futures.next().await {
                match result {
                    Err(e) => {
                        error!("RecvError should not happen! {}", e);
                    },
                    Ok(transits) => {

                    }
                }
            }
        }
    }
}