use futures::{future, Future, Stream, stream};
use futures::task::{Context, Poll};
use tokio::runtime::Handle;

use crate::Metadata;

pub struct GateWayContext {
    pub handle: Handle,
}

impl GateWayContext {
    pub fn loop_remote(&self) -> Handle {
        self.handle.clone()
    }

    /// Spawn a future, ignore result.
    pub fn spawn<F>(&self, f: F)
        where
            F: Future<Output=crate::Result<()>> + Send + 'static,
    {
        self.handle.spawn(async {
            if let Err(e) = f.await {
                println!("spaned future returned error: {:?}", e);
            }
        });
    }

    /// Spawn a poll_fn future. Function error is ignored.
    pub fn spawn_poll_fn<F>(&self, f: F)
        where
            F: FnMut(&mut Context<'_>) -> Poll<crate::Result<()>> + Send + 'static,
    {
        self.spawn(futures::future::poll_fn(f))
    }
}