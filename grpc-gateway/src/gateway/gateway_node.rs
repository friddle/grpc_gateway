use std::pin::Pin;

use futures::Future;
use httpbis::{AnySocketAddr, Headers};
use tokio::runtime::Handle;

use crate::{Error, Metadata, NodeConf};
use crate::common::node_conf::ClientAddress;
use crate::gateway::gateway_struct::struct_types::{IHttp2Node, IHttp2Stream};

pub type GateWayNodeFuture<Stream: IHttp2Stream, Node: IHttp2Node> = Pin<Box<dyn Future<Output=crate::Result<(Stream, Node)>> + Send>>;

#[allow(dead_code)]
pub trait GateWayNode: Clone + Sync + Send {
    type Stream: IHttp2Stream;
    type Node: IHttp2Node;

    fn new(node_conf: &NodeConf, handle: Option<Handle>) -> Self;
    //这里确实有可能是执行了才可能
    fn poll_fn(&self, path: String, metadata: Metadata) -> GateWayNodeFuture<Self::Stream, Self::Node>;

    fn client_id(&self) -> String;

    fn get_address(&self) -> (String, u16);
}



