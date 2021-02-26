use bytes::Bytes;
use futures::channel::mpsc;
use crate::{Headers,Header,ConnectInfo};

//TODO:所有请求变成两个请求。且不要update
//TODO:执行Log并且执行
pub trait ILogInterceptor{
    fn request_listener(
        &mut self,
        method:String,
        header:Headers,
        reqBytes:Vec<Bytes>,
        trailer:Headers,
        begin_time:u32
    );
    fn response_listener(
        &mut self,
        header:Headers,
        rspBytes:Vec<Bytes>,
        trailers:Headers,
        end_time:u32,
        connect_info:ConnectInfo
    );
}

pub struct LogConnectData{
   
}

//多个interceptor复用一个connectValue
//多个Bytes合成一个
//let intceptor=LogInterceptorSend.addLogInterceptor(A).addLogInterceptor(A).build()
//前面没行收集Bytes
//最后一步是new thread.run

pub struct LogInterceptorImpl{
    pub connectInfo:LogConnectData
}




