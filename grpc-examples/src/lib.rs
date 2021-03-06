extern crate futures;
extern crate grpc;
extern crate grpc_protobuf;
extern crate protobuf;
extern crate tls_api;

use proto_gen::helloworld::HelloReply;
use proto_gen::helloworld::HelloRequest;
use proto_gen::helloworld_grpc::Greeter;
use proto_gen::helloworld_grpc::GreeterClient;
use proto_gen::helloworld_grpc::GreeterServer;

//pub mod route;
pub mod proto_gen;
pub mod route;

pub const isClientTls: bool = true;
pub const isServerTls: bool = true;
pub const DEFAULT_CALL_PORT: u16 = 5001;
pub const DEFAULT_LISTEN_GREET_PORT: u16 = 5002;
pub const DEFAULT_LISTEN_ROUTE_PORT: u16 = 5003;
pub const ROUTE_GUIDE_DB_PATH: &str = "testdata/route_guide_db.json";
