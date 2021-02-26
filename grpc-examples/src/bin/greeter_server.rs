use std::env;
use std::thread;

use grpc::ServerHandlerContext;
use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;

use grpc_examples::{DEFAULT_LISTEN_GREET_PORT, isServerTls};
use grpc_examples::proto_gen::helloworld::{HelloReply, HelloRequest};
use grpc_examples::proto_gen::helloworld_grpc::Greeter;
use grpc_examples::proto_gen::helloworld_grpc::GreeterServer;

use tls_api::TlsAcceptorBuilder;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<HelloRequest>,
        resp: ServerResponseUnarySink<HelloReply>,
    ) -> grpc::Result<()> {

        let mut r = HelloReply::new();
        let name = if req.message.get_name().is_empty() {
            "world"
        } else {
            req.message.get_name()
        };
        println!("greeting request from {}", name);
        r.set_message(format!("Hello {}", name));
        resp.finish(r)
    }
}

fn test_tls_acceptor() -> tls_api_native_tls::TlsAcceptor {
    let pkcs12 = include_bytes!("../foobar.com.p12");
    let builder = tls_api_native_tls::TlsAcceptorBuilder::from_pkcs12(pkcs12, "mypassmypass").unwrap();
    builder.build().unwrap()
}

fn is_tls() -> bool
{
    println!("envaruments {:?}",env::args());
    env::args().any(|a| a == "tls")
}

fn main() {
    //let tls = is_tls();
    let tls=isServerTls;

    let port = DEFAULT_LISTEN_GREET_PORT;

    println!("is_tls {} port {}",tls,port);
    let mut server = grpc::ServerBuilder::new();
    server.http.set_port(port);
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    //http2_common.http.set_cpu_pool_threads(4);
    if tls {
        server.http.set_tls(test_tls_acceptor());
    }
    let _server = server.build().expect("sinks");

    println!(
        "greeter sinks started on port {} {}",
        port,
        if tls { "with tls" } else { "without tls" }
    );

    loop {
        thread::park();
    }
}
