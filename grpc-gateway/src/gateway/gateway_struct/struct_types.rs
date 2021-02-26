use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::sync::{Arc, RwLock};

use bytes::Bytes;
use futures::{Future, Stream};
use httpbis::ErrorCode;
use crate::{Chars, Error, Header, Headers};
use crate::gateway::grpc_frames::grpc_frame_parser::GrpcFrameParser;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Http2DataType {
    BytesWithEndStream((Bytes, bool)),
    Headers(Headers),
    Trailer(Headers),
    RST(ErrorCode),
    Error(Error),
    //error
}

pub trait IHttp2Stream: Stream<Item=Http2DataType> + Unpin + Send {
    fn frame_processed(&mut self, frame_size: u32) -> crate::Result<()> { Ok(()) }
    fn frame_above(&mut self, frame_size: u32) -> crate::Result<()> { Ok(()) }
}


//Headers
pub trait IHttp2Node: Unpin + Send {
    fn send_data(&mut self, data: Bytes, end_of_stream: bool) -> crate::Result<()>;
    fn send_header(&mut self, header: Headers,end_of_stream:bool) -> crate::Result<()>;
    fn send_trailers(&mut self, trailer: Headers) -> crate::Result<()>;
    fn send_error(&mut self,error:crate::Error)->crate::Result<()>;
    fn _close(&mut self) -> crate::Result<()>;
}

pub type ConnectInfo = Arc<RwLock<HashMap<String, Option<Chars>>>>;
pub type Http2StreamBox = Box<dyn IHttp2Stream>;
pub type Http2NodeBox = Box<dyn IHttp2Node>;


pub struct GrpcParser {
    pub buf: GrpcFrameParser,
    pub parsed_frames: VecDeque<Bytes>,
}


impl GrpcParser {
    pub fn new() -> Self {
        GrpcParser {
            buf: GrpcFrameParser::default(),
            parsed_frames: VecDeque::new(),
        }
    }
}

#[macro_export]
macro_rules! println_bytes {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $(match arg{
           Bytes=>println!("bytes:"),
           _=>println!($arg)
        };)*
    })
}
