use std::borrow::BorrowMut;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use futures::*;
use futures::Stream;
use futures::task::{Context, Poll};
use httpbis::Headers;

use crate::{GrpcDispatchBox, GrpcStatus, InterceptorBox, Metadata};
use crate::common::headers::{headers_200, headers_500, trailers};
use crate::gateway::gateway_struct::struct_types::{ConnectInfo, GrpcParser, Http2DataType, IHttp2Node, IHttp2Stream};
use crate::gateway::grpc_frames::grpc_frame::{write_grpc_frame_bytes_v2};

use super::gateway_close_message::GateWayEndMessage;
use crate::common::bytesx::bytes_debug_output;

pub struct RouterIncome<OutcomeNode, IncomeStream>
    where OutcomeNode: IHttp2Node, IncomeStream: IHttp2Stream
{
    pub outcome_node: OutcomeNode,
    pub income_stream: IncomeStream,
    pub interceptors: Vec<Arc<InterceptorBox>>,
    pub pub_values: ConnectInfo,
}


impl<OutcomeNode, IncomingStream> RouterIncome<OutcomeNode, IncomingStream>
    where OutcomeNode: IHttp2Node, IncomingStream: IHttp2Stream
{
    pub fn new(req: OutcomeNode,
               stream: IncomingStream,
               interceptors: Vec<Arc<InterceptorBox>>,
               pub_values: ConnectInfo,
    ) -> Self
    {
        return RouterIncome {
            outcome_node: req,
            income_stream: stream,
            interceptors,
            pub_values,
        };
    }

    fn send_outcome_data(&mut self, mut bytes: Bytes, mut frame_size: usize, mut end_stream: bool) -> crate::Result<()> {
        //这里可以定能优化的。有时间搞定;
        for interceptor in &self.interceptors {
            bytes= interceptor.comming_frame(bytes, end_stream,frame_size as u32,self.pub_values.clone());
        }
        self.outcome_node.send_data(bytes, end_stream);
        Ok(())
    }

    fn send_outcome_headers(&mut self, mut header: Headers,end_of_stream:bool) -> crate::Result<()> {
        for interceptor in &self.interceptors{
            header=interceptor.comming_headers(header,self.pub_values.clone());
        }
        self.outcome_node.send_header(header,end_of_stream);
        Ok(())
    }

    fn close_all_with_trailer(&mut self, mut header: Headers) -> crate::Result<()> {
        for interceptor in &self.interceptors{
            header=interceptor.comming_trailer(header,self.pub_values.clone());
        }
        self.outcome_node.send_trailers(header);
        Ok(())
    }

    fn send_grpc_error(&mut self, mut error: crate::Error) -> crate::Result<()>
    {
        for interceptor in &self.interceptors{
            error=interceptor.comming_error(error);
        }
        self.outcome_node.send_error(error);
        Ok(())
    }

    pub fn _close_all(&mut self) {
        //self.end.send_close_message();
        //self.outcome_node._close();
    }
}

impl<OutcomeNode, IncomingStream> Stream for RouterIncome<OutcomeNode, IncomingStream>
    where OutcomeNode: IHttp2Node, IncomingStream: IHttp2Stream
{
    type Item = crate::Result<()>;

    //每次都不需要返回。每次只需要搞就行->
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            let poll: Poll<Option<Http2DataType>> = unsafe {
                Pin::new_unchecked(&mut self.income_stream).poll_next(cx)
            };
            let data = match poll {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(data)) => data,
                Poll::Ready(None) => {
                    //Unary
                    self._close_all();
                    return Poll::Ready(None);
                }
            };
            match data {
                Http2DataType::Headers(header) => {
                    self.send_outcome_headers(header,false);
                    return Poll::Ready(Some(Ok(())));
                }
                Http2DataType::BytesWithEndStream((bytes, is_end_stream)) => {
                    let frame_size = bytes.len();
                    //这里要debug下运行机制->total_consumed->
                    self.send_outcome_data(bytes,frame_size,is_end_stream);
                    self.income_stream.frame_above(frame_size as u32);
                }
                Http2DataType::RST(code) => {

                    self.send_grpc_error(crate::Error::from(code));
                    return Poll::Ready(None);
                }
                Http2DataType::Trailer(header) => {
                    self.close_all_with_trailer(header);
                    return Poll::Ready(None);
                }
                Http2DataType::Error(error) => {

                    self.send_grpc_error(error);
                    return Poll::Ready(None);
                }
            };
        }
    }
}






