use std::pin::Pin;
use std::sync::Mutex;

use futures::*;
use futures::channel::oneshot;
use futures::channel::oneshot::{Receiver, Sender};
use futures::sink::SinkExt;
use futures::task::{Context, Poll};

use crate::Result;

pub struct GateWayEndMessage {
    pub sender: Mutex<Option<Sender<bool>>>,
    pub receiver: Receiver<bool>,
}

impl GateWayEndMessage {
    pub fn new() -> (GateWayEndMessage, GateWayEndMessage) {
        let (tx1, rx1) = oneshot::channel::<bool>();
        let (tx2, rx2) = oneshot::channel::<bool>();
        return (
            GateWayEndMessage {
                sender: Mutex::new(Some(tx1)),
                receiver: rx2,
            },
            GateWayEndMessage {
                sender: Mutex::new(Some(tx2)),
                receiver: rx1,
            }
        );
    }

    pub fn send_close_message(&mut self) -> crate::Result<()>
    {
        let mut sender = self.sender.lock().unwrap();
        sender.take().unwrap().send(true);
        Ok(())
    }
}

impl Future for GateWayEndMessage {
    type Output = bool;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let result = self.receiver.poll_unpin(cx);
        match result {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(result)) => Poll::Ready(result),
            Poll::Ready(Err(cancel)) => Poll::Ready(false),
            _ => Poll::Ready(false)
        }
    }
}
