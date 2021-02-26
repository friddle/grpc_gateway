use std::net::ToSocketAddrs;
use std::sync::Arc;

use httpbis::{AnySocketAddr, ServerBuilder, ServerHandlerContext, ServerRequest, ServerResponse, ServerTlsOption};
use tls_api::TlsAcceptor;

use crate::{Error, Metadata};
use crate::common::node_conf::ClientAddress::Unix;
use crate::gateway::dispatch::DispatchCore;
use crate::gateway::gateway_node::{GateWayNode, GateWayNodeFuture};
use crate::gateway::gateway_server::Server;
use crate::gateway::gateway_struct::gateway_context::GateWayContext;
use crate::gateway::gateway_struct::gateway_server_build::ServerConf;
use crate::gateway::gateway_struct::gateway_types::GateWayTypes;
use crate::gateway::gateway_struct::struct_types::{IHttp2Node, IHttp2Stream};
use crate::gateway::httpbis_impl::httpbis_gatenode::HttpBisNode;
use crate::gateway::httpbis_impl::httpbis_struct_income::{HttpBisIncomeNode, HttpBisIncomeStream};
use crate::gateway::httpbis_impl::httpbis_struct_types::HttpbisTypes;

use super::httpbis_handler;

#[derive(Debug)]
pub struct HttpBisServer {
    pub server: Option<httpbis::Server>,
    pub conf: ServerConf,
    pub core: Arc<DispatchCore<HttpbisTypes>>,
}

impl HttpBisServer {}

//这里就很奇妙了。。。
impl httpbis::ServerHandler for DispatchCore<HttpbisTypes> {
    fn start_request(
        &self,
        context: httpbis::ServerHandlerContext,
        req: httpbis::ServerRequest,
        mut resp: httpbis::ServerResponse,
    ) -> httpbis::Result<()> {
        let path = req.headers.path().to_owned();
        let metadata = Metadata::from_headers(req.headers.clone()).map_err(|e|
            httpbis::Error::InternalError(String::from("metadata error")))?;
        let income_stream = req.register_stream_handler(|serverIncreaseInWindow| {
            let (send, receiver) = httpbis_handler::create_handler(serverIncreaseInWindow);
            (send, HttpBisIncomeStream { stream: receiver })
        });
        let income_node = HttpBisIncomeNode { rsp: resp };
        let context = GateWayContext { handle: context.loop_remote() };
        //这里肯定有问题的啊
        let dispatch_result=self.dispatch(path, metadata, context, income_node, income_stream)
            .map_err(|e| map_httpbis_error(e));
        dispatch_result
    }
}

fn map_httpbis_error(e: crate::Error) -> httpbis::Error {
    return match e {
        Error::Http(http_err) => http_err,
        _ => httpbis::Error::InternalError(String::from("inter error")),
    };
}


impl Server for HttpBisServer {
    type InnerServer = httpbis::Server;
    type Types = HttpbisTypes;

    fn build(conf: ServerConf, core: Arc<DispatchCore<Self::Types>>) -> Self {
        return HttpBisServer {
            core,
            conf,
            server: None,
        };
    }

    fn start_tls_listen<Tls: TlsAcceptor>(&mut self, address: AnySocketAddr, ssl: Tls) -> Result<(), Error> where Tls: TlsAcceptor {
        let mut server_build:ServerBuilder<Tls>= ServerBuilder::new();
        server_build.set_tls(ssl);
        match address {
            AnySocketAddr::Inet(addr) => server_build.set_addr(addr),
            AnySocketAddr::Unix(addr) => server_build.set_unix_addr(addr)
        };
        server_build.service.set_service("/", self.core.clone());
        server_build.conf.thread_name = Some(String::from("gateway-thread"));
        let server = server_build.build().map_err(|e| Error::from(e))?;
        self.server = Some(server);
        Ok(())
    }

    fn start_plain_listen(&mut self, address: AnySocketAddr) -> Result<(), Error> {
        let mut server_build = ServerBuilder::new_plain();
        match address {
            AnySocketAddr::Inet(addr) => server_build.set_addr(addr)?,
            AnySocketAddr::Unix(addr) => server_build.set_unix_addr(addr)?
        };
        server_build.service.set_service("/", self.core.clone());
        server_build.conf.thread_name = Some(String::from("gateway-thread"));
        let server = server_build.build().map_err(|e| Error::from(e))?;
        self.server = Some(server);
        Ok(())
    }


    fn is_alive(&self) -> bool {
        return match &self.server {
            Some(server) => server.is_alive(),
            None => false
        };
    }

    fn get_address(&self) -> Option<AnySocketAddr> {
        return match &self.server {
            Some(server) => Some(server.local_addr().clone()),
            None => None
        };
    }

    fn inner_server() -> &'static httpbis::Server {
        unimplemented!()
    }
}

