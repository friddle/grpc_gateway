use std::net::ToSocketAddrs;
use std::sync::Arc;

use httpbis::AnySocketAddr;
use tls_api::TlsAcceptor;

use crate::gateway::dispatch::DispatchCore;
use crate::gateway::gateway_node::GateWayNode;
use crate::gateway::gateway_struct::gateway_types::GateWayTypes;
use crate::gateway::gateway_struct::struct_types::{IHttp2Node, IHttp2Stream};
use crate::gateway::httpbis_impl::httpbis_struct_income::HttpBisIncomeNode;
use crate::ServerConf;

/// Running http2_common.
/// running_tls
pub trait Server {
    type InnerServer;
    type Types: GateWayTypes;

    fn build(conf: ServerConf, core: Arc<DispatchCore<Self::Types>>) -> Self;

    fn start_tls_listen<Tls: TlsAcceptor>(&mut self, address: AnySocketAddr, ssl: Tls)
        -> crate::Result<()> where Tls: TlsAcceptor;

    fn start_plain_listen(&mut self, address: AnySocketAddr) -> crate::Result<()>;

    fn is_alive(&self) -> bool;

    fn get_address(&self) -> Option<AnySocketAddr>;

    fn inner_server() -> &'static Self::InnerServer;
}


