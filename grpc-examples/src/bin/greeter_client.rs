use std::env;
use std::sync::Arc;

use futures::executor;
use grpc::ClientStub;
use grpc::ClientStubExt;
use tls_api::TlsConnector;

use grpc_examples::{DEFAULT_CALL_PORT, isClientTls};
use grpc_examples::proto_gen::helloworld::{HelloReply, HelloRequest};
use grpc_examples::proto_gen::helloworld_grpc::GreeterClient;
use tls_api::TlsConnectorBuilder;
use tls_api_native_tls::*;

fn test_tls_connector() -> tls_api_native_tls::TlsConnector {
    let root_ca = include_bytes!("../root-ca.der");
    let root_ca = tls_api::Certificate::from_der(root_ca.to_vec());

    let mut builder = tls_api_native_tls::TlsConnector::builder().unwrap();
    builder
        .add_root_certificate(root_ca)
        .expect("add_root_certificate");
    builder.build().unwrap()
}

fn is_tls() -> bool {
    println!("envaruments {:?}",env::args());
    env::args().any(|a| a == "tls")
}

fn main() {
    env_logger::init();

    //let tls = is_tls();
    let tls=isClientTls;

    let name = "word";

    let port = DEFAULT_CALL_PORT;

    let client_conf = Default::default();

    println!("is tls {} port {}",tls,port);

    let client = if tls {
        // This is a bit complicated, because we need to explicitly pass root CA here
        // because http2_common uses self-signed certificate.
        // TODO: simplify it
        let tls_option =
          httpbis::ClientTlsOption::Tls("foobar.com".to_owned(), Arc::new(test_tls_connector()));
        let grpc_client = Arc::new(
            grpc::ClientBuilder::new("127.0.0.1", port)
                .explicit_tls(tls_option)
                .build()
                .unwrap(),
        );
        GreeterClient::with_client(grpc_client)
    } else {
        GreeterClient::new_plain("127.0.0.1", port, client_conf).unwrap()
    };

    let mut req = HelloRequest::new();
    req.set_name(String::from(name));

    let resp = client
        .say_hello(grpc::RequestOptions::new(), req)
        .join_metadata_result();

    println!("befor call");

    let (meta_data, reply, trail_meta) = executor::block_on(resp).expect("ok");
    println!("{:?}", reply.message);
}
