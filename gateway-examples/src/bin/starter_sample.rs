use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::thread;

use grpc_gateway::{DefaultGrpcDispatchHandler, HttpBisServer, NodeConf, ServerBuilder, GrpcDispatchBox};
use grpc_gateway::gateway::gateway_server::Server;
use gateway_examples::{DEFAULT_HOST, DEFAULT_LISTEN_GREET_PORT, DEFAULT_LISTEN_ROUTE_PORT, DEFAULT_LISTEN_GATEWAY_PORT};


fn greet_dispatch()->GrpcDispatchBox
{
    let node_conf = NodeConf::new_plain(DEFAULT_HOST, DEFAULT_LISTEN_GREET_PORT);
    let dispatch =
        Box::new(DefaultGrpcDispatchHandler::new("/helloword*".to_owned(),
                                                 Vec::from([node_conf])));
    return dispatch;
}

fn chat_dispatch()->GrpcDispatchBox
{
    let node_conf = NodeConf::new_plain(DEFAULT_HOST, DEFAULT_LISTEN_ROUTE_PORT);
    let dispatch =
        Box::new(DefaultGrpcDispatchHandler::new("/routeguide*".to_owned(),
                                                 Vec::from([node_conf])));
    return dispatch;
}


fn main()
{
    let server_build =
        ServerBuilder::new_plain().set_port(DEFAULT_LISTEN_GATEWAY_PORT)
            .add_dispatch(Arc::new(greet_dispatch()))
            .add_dispatch(Arc::new(chat_dispatch()));
    let server: HttpBisServer = server_build.build().expect("run");
    println!("alive:{}", server.is_alive());
    println!("server address:{:?}", server.get_address());
    if server.is_alive(){
        loop {
            thread::park()
        }
    }
}