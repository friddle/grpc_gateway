Grpc-GateWay
========
[中文文档](./README_ZH.md)
Grpc Gateway impelement for Rust.choose backend node by method and metadata
And provider some function for gateway

fork project [grpc-rust](https://github.com/stepancheg/grpc-rust)。

Because my code has too many Unnecessary commits.So will clean up the commits. Start with the full version  
Because I am not good at rust. There are very much incorrect rust usage. if find and fix.

## Current Status
*.  Https is unavailable. It's an upstream code problem. Ready to help  with upstream code sometime  
*.  LoadBalance has not been implemented yet  
*.  using it in my test server for now. Ready to use it in production next phase.   
*.  There are still some special states not tested and handled.

## Concept
|Name      |Meaning                    | 
|:--------|:-----------------------|
|NodeConf|Back-end node configuration|
|Dispatch|routing configuration|
|Interceptor|basic interceptor|
|LogInterceptor|log interceptor|

## Usage

### Basic Usage  

  
guide code:[gateway-examples/src/bin/starter_sample](/gateway-examples/src/bin/starter_sample.rs)
```
fn main()
{
    let node_conf = NodeConf::new_plain("localhost", 50051);
    let dispatch =
        Box::new(DefaultGrpcDispatchHandler::new("/".to_owned(), Vec::from([node_conf])));
    let server_build = ServerBuilder::new_plain().set_port(50052).add_dispatch(Arc::new(dispatch));
    let server: HttpBisServer = server_build.build().expect("run");
    println!("alive:{}", server.is_alive());
    println!("server address:{:?}", server.get_address());
    if server.is_alive(){
        loop {
            thread::park()
        }
    }
}
```


### Use yml to start server
Guide [gateway-examples/src/bin/start_yml](/gateway-examples/src/bin/starter_yml.rs)


```
 listen:
       port: 50051
       auth: none
 proxys:
     -
       name: helloworld
       methods:
         - /helloword*
       nodes:
         -
           host: localhost
           port: 50051
           auth: none
     -
       name: round_chat
       methods:
         - /round*
       nodes:
         -
           host: localhost
           port: 50053
           auth: none
```
start by `starter_yml --file=./demo.yml`


## Common Struct 
TODO

## TodoList:
*.   Fix Https  
*.   Add LogInterceptor implement
*.   Fix Bytes from 0.5 to 1.0
*.   Add pressure test code
*.   Add baklance implemention  
*.   Add Redis/Web System to Manage
