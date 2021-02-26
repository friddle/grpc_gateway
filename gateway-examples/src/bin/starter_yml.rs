use std::collections::HashMap;
use std::{env, fs};
use std::sync::Arc;
use std::thread;
use grpc_gateway::{DispatchYamlStarter, HttpBisServer};
use grpc_gateway::gateway::gateway_server::Server;


fn main()
{    
    let file_location = env::args()
        .filter(|a| a != "--file=")
        .nth(1)
        .map(|s| s.to_owned())
        .unwrap_or_else(|| "./src/bin/demo.yml".to_owned());
    let file_location=String::from(file_location);
    let ymlStarter=DispatchYamlStarter::<HttpBisServer>::parse_from_file(&file_location).run();
    let server=ymlStarter.server.as_ref().expect("");
    println!("listen address:{:?}",server.get_address().unwrap());
    loop {
        thread::park();
    }
}