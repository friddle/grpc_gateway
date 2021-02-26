use bytes::Bytes;
use crate::{ConnectInfo, Error, Metadata, Result,Headers};

pub mod log;
pub mod low_level_log;


//TODO:等下搞個Receiver版本
pub trait Interceptor: Sync + Send + 'static
{

    fn comming_req(&self,path:String,metadata:Metadata,data:ConnectInfo)->(String,Metadata){
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

pub type InterceptorBox = Box<dyn Interceptor>;

pub trait LogInterceptor: Sync + Send + 'static {}
