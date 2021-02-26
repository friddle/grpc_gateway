use std::pin::Pin;
use std::sync::Arc;

use futures::future::TryFutureExt;
use httpbis::{AnySocketAddr, Client, ClientBuilder, ClientConf, HttpScheme, HttpStreamAfterHeaders, ServerResponse, ClientTlsOption};
use tokio::runtime::Handle;
use tls_api_native_tls::*;

use crate::{Error, Headers, Metadata, NodeConf};
use crate::common::headers::init_headers;
use crate::common::node_conf::{ClientAddress, get_client_address, Tls};
use crate::gateway::gateway_node::{GateWayNode, GateWayNodeFuture};
use crate::gateway::gateway_struct::struct_types::{IHttp2Node, IHttp2Stream};
use crate::gateway::httpbis_impl::httpbis_struct_income::HttpBisIncomeNode;
use crate::gateway::httpbis_impl::httpbis_struct_outcome::{HttpBisOutcomeNode, HttpBisOutcomeStream};

pub struct HttpBisNode {
    client: Arc<httpbis::Client>,
    conf: NodeConf,
    httpbis_conf: httpbis::ClientConf,
    handle: Option<Handle>,
}


impl HttpBisNode {
    //假如创建失败怎么办?DeadClient?
    fn new_plain_client(node_conf:&NodeConf,handle:&Option<Handle>)->Client
    {
        let mut client_builder=ClientBuilder::new_plain();
        client_builder.set_addr((&node_conf.host.clone()[..], node_conf.port.unwrap_or(5000)));
        match handle {
            Some(_handle) => client_builder.event_loop = Some(_handle.clone()),
            _ => ()
        }
        return client_builder.build().expect("should run");
    }

    fn new_tls_client<T:tls_api::TlsConnector>(node_conf:&NodeConf,tls:&Arc<T>,handle:&Option<Handle>)->Client{
        let mut client_builder=ClientBuilder::<T>::new();
        client_builder.tls=ClientTlsOption::Tls(node_conf.authorized.clone().unwrap(),tls.clone());
        client_builder.set_addr((&node_conf.host.clone()[..], node_conf.port.unwrap_or(5000)));
        match handle {
            Some(_handle) => client_builder.event_loop = Some(_handle.clone()),
            _ => ()
        }
        return client_builder.build().expect("should build success");
    }

}

impl GateWayNode for HttpBisNode {
    type Stream = HttpBisOutcomeStream;
    type Node = HttpBisOutcomeNode;

    fn new(node_conf: &NodeConf, handle: Option<Handle>) -> Self {
        let client=match &node_conf.tls{
            Tls::Native(tls)=>{
                Self::new_tls_client::<tls_api_native_tls::TlsConnector>(&node_conf,tls,&handle)
            },
            Tls::None=>{
                Self::new_plain_client(&node_conf,&handle)
            },
            _=>{
                panic!("unsupport tls type")
            }
        };
        return HttpBisNode {
            client: Arc::new(client),
            conf: node_conf.clone(),
            handle,
            httpbis_conf: ClientConf::new(),
        };
    }

    fn poll_fn(&self, path: String, metadata: Metadata) -> GateWayNodeFuture<HttpBisOutcomeStream, HttpBisOutcomeNode>
    {
        let req_headers = init_headers(path, metadata, self.conf.host.clone(), HttpScheme::Http);
        let http_future = self.client.start_request(
            req_headers, None, None, false,
        );
        let http_future = http_future.map_err(crate::Error::from);
        Box::pin(http_future.map_ok(
            move |(req, stream_rsp)| {
                let node = HttpBisOutcomeNode { req };
                let stream = HttpBisOutcomeStream::new(stream_rsp);
                (stream, node)
            }
        ))
    }

    fn client_id(&self) -> String {
        return self.conf.id.clone();
    }

    fn get_address(&self) -> (String, u16)
    {
        return (self.conf.host.clone(), self.conf.port.unwrap_or(5000));
    }
}

impl Clone for HttpBisNode {
    fn clone(&self) -> Self {
        unimplemented!("")
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!("");
    }
}

