use std::thread;

use grpc_examples::{DEFAULT_LISTEN_GREET_PORT, DEFAULT_CALL_PORT, DEFAULT_LISTEN_ROUTE_PORT, isServerTls};
use grpc_examples::route::server::RouteGuideImpl;
use grpc_examples::proto_gen::route_guide_grpc::RouteGuideServer;
use grpc_examples::proto_gen::route_guide_grpc::RouteGuideClient;

use tls_api::TlsAcceptorBuilder;


fn test_tls_acceptor() -> tls_api_native_tls::TlsAcceptor {
    let pkcs12 = include_bytes!("../foobar.com.p12");
    let builder = tls_api_native_tls::TlsAcceptorBuilder::from_pkcs12(pkcs12, "mypassmypass").unwrap();
    builder.build().unwrap()
}

fn main() {
    let service_def = RouteGuideServer::new_service_def(RouteGuideImpl::new_and_load_db());

    let port = DEFAULT_LISTEN_ROUTE_PORT;
    let tls=isServerTls;
    let mut server_builder = grpc::ServerBuilder::new();
    server_builder.add_service(service_def);
    server_builder.http.set_port(port);
    if tls{
        server_builder.http.set_tls(test_tls_acceptor());
    }
    let server = server_builder.build().expect("build");

    println!("server stared on addr {} tls {}", server.local_addr(),tls);

    loop {
        thread::park();
    }
}
