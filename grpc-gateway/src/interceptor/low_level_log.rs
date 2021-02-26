use crate::interceptor::LogInterceptor;
use crate::{Metadata, ConnectInfo, Headers, Interceptor};
use bytes::Bytes;
use log::{info, trace, warn};
pub struct LowLevelLogInterceptor{

}

impl LowLevelLogInterceptor{
    pub fn new()->Self{
        env_logger::init();
        return LowLevelLogInterceptor{

        }
    }
}

impl Interceptor for LowLevelLogInterceptor{
    fn comming_req(&self,path:String,metadata:Metadata,data:ConnectInfo)->(String,Metadata){
        println!("receive req path:{} \n metadata:{:?} \n data:{:?}",path,metadata,data);
        return (path,metadata)
    }

    //
    fn comming_headers(&self,headers:Headers,data:ConnectInfo)->Headers{
        return headers;
    }

    fn comming_frame(&self, bytes: Bytes,end_or_stream:bool,frame_size: u32, data: ConnectInfo) -> Bytes {
        return bytes;
    }

    fn comming_error(&self,error:crate::Error)->crate::Error
    {
        println!("error :{:?}",error);
        return error;
    }


    fn comming_trailer(&self,trailer:Headers,data:ConnectInfo)->Headers{
        return trailer
    }

    fn outcome_headers(&self,headers:Headers,data:ConnectInfo)->Headers{
        return headers;
    }

    fn outcome_frame(&self,bytes:Bytes,end_of_stream:bool,frame_size:u32,data:ConnectInfo)->Bytes{
        return bytes;
    }

    fn outcome_trailer(&self,trailer:Headers,data:ConnectInfo)->Headers{
        return trailer;
    }

}