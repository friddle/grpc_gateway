use std::collections::HashMap;
use std::{env, fmt};
use std::sync::Arc;
use std::thread;
use tls_api::{TlsConnector, TlsAcceptorBuilder};
use tls_api::TlsAcceptor;
use grpc_gateway::{DefaultGrpcDispatchHandler, HttpBisServer, NodeConf, ServerBuilder, GrpcDispatchBox};
use grpc_gateway::gateway::gateway_server::Server;
use grpc_gateway::common::node_conf::Tls;
use tls_api::TlsConnectorBuilder;
use strfmt::strfmt;
use std::io::{BufReader, Read};
use std::fs::File;
use gateway_examples::{DEFAULT_HOST, DEFAULT_LISTEN_GREET_PORT, DEFAULT_LISTEN_ROUTE_PORT, DEFAULT_LISTEN_GATEWAY_PORT, isServerTls, isClientTls, DEFAULT_AUTHORIZED};


const auth_pre_folder:&'static str="../grpc-examples/src/{auth_file}";

fn test_tls_connector() -> tls_api_native_tls::TlsConnector {
    let mut var:HashMap<String,String>=HashMap::new();
    var.insert("auth_file".to_string(),"root-ca.der".to_string());
    let file=strfmt(auth_pre_folder,&var).unwrap();
    let mut file=BufReader::new(File::open(&file)
        .expect(&format!("pem not exists in root-ca")));
    let mut root_ca =Vec::new();
    file.read_to_end(&mut root_ca);
    let root_ca = tls_api::Certificate::from_der(root_ca);
    let mut builder = tls_api_native_tls::TlsConnector::builder().unwrap();
    builder
        .add_root_certificate(root_ca)
        .expect("add_root_certificate");
    builder.build().unwrap()
}

fn test_tls_acceptor() -> tls_api_native_tls::TlsAcceptor {
    let mut var:HashMap<String,String>=HashMap::new();
    var.insert("auth_file".to_string(),"foobar.com.p12".to_string());
    let file=strfmt(auth_pre_folder,&var).unwrap();
    let mut file=BufReader::new(File::open(file)
        .expect(&format!("p12 not exists in root-ca")));
    let mut pkcs12 =Vec::new();
    file.read_to_end(&mut pkcs12);
    let builder = tls_api_native_tls::TlsAcceptorBuilder::from_pkcs12(&pkcs12, "mypassmypass").unwrap();
    builder.build().unwrap()
}


fn greet_dispatch()->GrpcDispatchBox
{
    let node_conf = if isClientTls{
        NodeConf::new_tls(DEFAULT_HOST, DEFAULT_LISTEN_GREET_PORT,
                          Tls::Native(Arc::new(test_tls_connector())),
                          DEFAULT_AUTHORIZED)
    }else{
        NodeConf::new_plain(DEFAULT_HOST, DEFAULT_LISTEN_GREET_PORT)
    };
    let dispatch =
        Box::new(DefaultGrpcDispatchHandler::new("/helloword*".to_owned(),
                                                 Vec::from([node_conf])));
    return dispatch;
}

fn chat_dispatch()->GrpcDispatchBox
{
    let node_conf = if isClientTls{
        NodeConf::new_tls(DEFAULT_HOST, DEFAULT_LISTEN_ROUTE_PORT,
                          Tls::Native(Arc::new(test_tls_connector())),
                          DEFAULT_AUTHORIZED)
    }else{
        NodeConf::new_plain(DEFAULT_HOST, DEFAULT_LISTEN_ROUTE_PORT)
    };
    let dispatch =
        Box::new(DefaultGrpcDispatchHandler::new("/routeguide*".to_owned(),
                                                 Vec::from([node_conf])));
    return dispatch;
}


fn main()
{
    let mut server_build =
        ServerBuilder::new().set_port(DEFAULT_LISTEN_GATEWAY_PORT)
            .add_dispatch(Arc::new(greet_dispatch()))
            .add_dispatch(Arc::new(chat_dispatch()));
    if isServerTls
    {
        server_build=server_build.set_tls(test_tls_acceptor());
    }
    let server: HttpBisServer = server_build.build().expect("run");
    println!("alive:{}", server.is_alive());
    println!("server address:{:?}", server.get_address());
    if server.is_alive(){
        loop {
            thread::park()
        }
    }
}
