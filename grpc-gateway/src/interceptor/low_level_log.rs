use crate::interceptor::LogInterceptor;
use crate::{Metadata, ConnectInfo, Headers, Interceptor};
use bytes::Bytes;
use log::{info,debug,trace, warn,error};
use env_logger::init;
pub struct LowLevelLogInterceptor{

}

impl LowLevelLogInterceptor{
    pub fn new()->Self{
        return LowLevelLogInterceptor{

        }
    }
}

impl Interceptor for LowLevelLogInterceptor{
    fn comming_req(&self,path:String,metadata:Metadata,data:ConnectInfo)->(String,Metadata){
        info!("receive req path:{} \n metadata:{:?} \n data:{:?}",path,metadata,data);
        return (path,metadata)
    }

    //
    fn comming_headers(&self,headers:Headers,data:ConnectInfo)->Headers{
        debug!("receive comming header{:?}",headers);
        return headers;
    }

    fn comming_frame(&self, bytes: Bytes,end_of_stream:bool,frame_size: u32, data: ConnectInfo) -> Bytes {
        debug!("receive comming frames{:?} end_of_stream:{}",bytes,end_of_stream);
        return bytes;
    }

    fn comming_error(&self,error:crate::Error)->crate::Error
    {
        error!("comming error :{:?}",error);
        return error;
    }

    fn comming_trailer(&self,trailer:Headers,data:ConnectInfo)->Headers{
        debug!("comming trailer :{:?}",trailer);
        return trailer
    }

    fn outcome_headers(&self,headers:Headers,data:ConnectInfo)->Headers{
        debug!("outcome headers :{:?}",headers);
        return headers;
    }

    fn outcome_frame(&self,bytes:Bytes,end_of_stream:bool,frame_size:u32,data:ConnectInfo)->Bytes{
        debug!("outcome frames :{:?} end_of_stream: {}",bytes,end_of_stream);
        return bytes;
    }

    fn outcome_trailer(&self,trailer:Headers,data:ConnectInfo)->Headers{
        debug!("outcome trailers :{:?}",trailer);
        return trailer;
    }

}