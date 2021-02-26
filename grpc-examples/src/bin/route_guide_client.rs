use futures::executor;
use grpc::ClientConf;
use grpc::prelude::*;
use grpc::ClientStub;
use grpc::ClientStubExt;

use grpc_examples::{DEFAULT_CALL_PORT, DEFAULT_LISTEN_ROUTE_PORT, isClientTls};
use grpc_examples::route::client::run_client;
use grpc_examples::proto_gen::route_guide_grpc::*;
use grpc_examples::proto_gen::route_guide::*;
use tls_api::TlsConnectorBuilder;
use tls_api::TlsConnector;
use std::sync::Arc;

fn test_tls_connector() -> tls_api_native_tls::TlsConnector {
    let root_ca = include_bytes!("../root-ca.der");
    let root_ca = tls_api::Certificate::from_der(root_ca.to_vec());

    let mut builder = tls_api_native_tls::TlsConnector::builder().unwrap();
    builder
        .add_root_certificate(root_ca)
        .expect("add_root_certificate");
    builder.build().unwrap()
}

fn main() {
    env_logger::init();
    let port = DEFAULT_CALL_PORT;

    let tls=isClientTls;
    let client =match tls{
        true=>{
           RouteGuideClient::new_plain("127.0.0.1", port, ClientConf::new()).expect("client")
        },
        false=>{
            let tls_option =
                httpbis::ClientTlsOption::Tls("foobar.com".to_owned(), Arc::new(test_tls_connector()));
            let grpc_client = Arc::new(
                grpc::ClientBuilder::new("127.0.0.1", port)
                    .explicit_tls(tls_option)
                    .build()
                    .unwrap(),
            );
            RouteGuideClient::with_client(grpc_client)
        }
    };
    println!("port {} tls {}",port,tls);
    executor::block_on(async { run_client(&client).await });
}
