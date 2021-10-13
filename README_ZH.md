Grpc-GateWay-Zh
========

本项目是为了进行grpc的请求转发。既通过相应的method和参数进行转发。
并且提供相应的功能  
抄的是这个[grpc-rust](https://github.com/stepancheg/grpc-rust)库。
因为代码有太多提交和瑕疵。所以会清理掉提交。从完整版开始  
因为本人技术有效。有非常多的不正确的rust写法。请见谅

## 当前状态 
Https基本不可用。是上游代码问题。准备有时间去帮忙弄下上游代码   
LoadBalance还没有实现   
基本可以。暂时只有在测试环境上使用。准备下阶段在生产状态使用。   
还有一些特殊状态没有测试和处理。    

## 具体概念
|名字      |含义                    | 
|:--------|:-----------------------|
|NodeConf|后端节点配置.用来配置相应节点的|
|Dispatch|进行分发节点配置|
|Interceptor|基础拦截器。可以相应的拦截并修改数据|
|LogInterceptor|日志拦截器|

## 具体用法为

### 基础用法  

  
请参考[gateway-examples/src/bin/start_sample](/gateway-examples/src/bin/start_sample)
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


### 使用yml_text启动
请参考[gateway-examples/src/bin/start_yml](/gateway-examples/src/bin/start_yml)


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
启动方式是 `starter_yml --file=./demo.yml`


## 常见结构定义
TODO  

## TodoList:
*.   完善上游Https代码。并添加多种Https实现。并通过测试    
*.   完成LogInterceptor实现  
*.   修改Bytes从0.5 到1.0的迁移  
*.   添加压力测试代码    
*.   完成多种LoadBalance实现    
*.   添加下游基于Redis/Web的管理系统。并实现动态刷新
