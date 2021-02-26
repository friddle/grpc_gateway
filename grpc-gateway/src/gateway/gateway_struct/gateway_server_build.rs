use std::net::{ToSocketAddrs};
use std::sync::Arc;

use httpbis::AnySocketAddr;

use crate::{Error, GrpcDispatchBox, InterceptorBox, ServerTlsOption};
use crate::gateway::dispatch::DispatchCore;
use crate::gateway::gateway_node::GateWayNode;
use crate::gateway::gateway_server::Server;
use crate::gateway::gateway_struct::gateway_types::GateWayTypes;
use crate::gateway::httpbis_impl::httpbis_struct_types::HttpbisTypes;
use crate::interceptor::low_level_log::*;
use crate::interceptor::LogInterceptor;

/// gRPC http2_common configuration.
#[derive(Default, Debug, Clone)]
pub struct ServerConf {}

impl ServerConf {
    /// Default configuration.
    pub fn new() -> ServerConf {
        Default::default()
    }
}

//还要多几个构建---->.比如http部分就里面去
pub struct ServerBuilder<A: tls_api::TlsAcceptor = tls_api_stub::TlsAcceptor>
{
    conf: ServerConf,
    interceptors: Vec<Arc<InterceptorBox>>,
    dispatches: Vec<Arc<GrpcDispatchBox>>,
    tls: ServerTlsOption<A>,
    address: Option<AnySocketAddr>,
}

impl ServerBuilder<tls_api_stub::TlsAcceptor> {
    /// New builder for no-TLS HTTP/2.
    pub fn new_plain() -> ServerBuilder<tls_api_stub::TlsAcceptor> {
        ServerBuilder::new()
    }
}

impl<tls: tls_api::TlsAcceptor> ServerBuilder<tls> {
    /// New builder for given TLS acceptor.
    pub fn new() -> ServerBuilder<tls> {
        ServerBuilder {
            conf: ServerConf::new(),
            interceptors:Self::interceptor_init(),
            dispatches: Vec::new(),
            tls: ServerTlsOption::Plain,
            address: None,
        }
    }

    fn interceptor_init()->Vec<Arc<InterceptorBox>>{
        let logInterceptor:Arc<InterceptorBox>=Arc::new(Box::new(LowLevelLogInterceptor::new()));
        let mut interceptors:Vec<Arc<InterceptorBox>>=Vec::new();
        interceptors.push(logInterceptor);
        return interceptors;
    }


    pub fn set_addr<T: ToSocketAddrs>(mut self, addr: T) -> Self {
        let addrs: Vec<_> = addr.to_socket_addrs().expect("should socket address support type").collect();
        if addrs.is_empty() {
            panic!("addr is empty")
        } else if addrs.len() > 1 {
            panic!("addr too much")
        }
        self.address = Some(AnySocketAddr::Inet(addrs.into_iter().next().unwrap()));
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.set_addr(format!("0.0.0.0:{}", port))
    }

    pub fn set_tls(mut self, acceptor: tls) -> Self {
        self.tls = ServerTlsOption::Tls(acceptor);
        self
    }

    //肯定需要加的是Arc啦->因为人家在外面也要用
    pub fn add_dispatch(mut self, dispatch: Arc<GrpcDispatchBox>) -> Self
    {
        self.dispatches.push(dispatch);
        self
    }

    pub fn set_conf(mut self, conf: ServerConf) -> Self {
        self.conf = conf;
        self
    }

    pub fn add_interceptor(
        mut self, interceptor: Arc<InterceptorBox>,
    ) -> Self {
        self.interceptors.push(interceptor);
        self
    }

    pub fn build<S: Server>(self) -> Result<S, Error> {
        if self.dispatches.is_empty() {
            return Err(Error::Panic(String::from("dispatch should not be empty")));
        }
        let core: Arc<DispatchCore<S::Types>> = Arc::new(
            DispatchCore::new(self.dispatches, self.interceptors));
        let addr =
            self.address.expect("address is not empty");
        let mut serve: S = S::build(self.conf, core);
        match self.tls {
            ServerTlsOption::Plain => {
                serve.start_plain_listen(addr);
            }
            ServerTlsOption::Tls(acceptor) => {
                serve.start_tls_listen(addr, acceptor);
            }
        }
        return Ok(serve);
    }
}
