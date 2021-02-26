use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use futures::*;
use futures::task::{Context, Poll};

use crate::{GrpcDispatchBox, GrpcStatus, Headers, InterceptorBox, Metadata};
use crate::common::bytesx::bytes_debug_output;
use crate::common::headers::{headers_200, headers_500, trailers};
use crate::gateway::gateway_struct::gateway_close_message::GateWayEndMessage;
use crate::gateway::gateway_struct::struct_types::{ConnectInfo, GrpcParser, Http2DataType, IHttp2Node, IHttp2Stream};
use crate::gateway::grpc_frames::grpc_frame::write_grpc_frame_bytes_v2;
use httpbis::BytesDeque;

pub struct RouterOutcome<OutcomeStream, IncomeNode>
    where OutcomeStream: IHttp2Stream, IncomeNode: IHttp2Node
{
    pub income_node: IncomeNode,
    pub outcome_stream: OutcomeStream,
    pub interceptors: Vec<Arc<InterceptorBox>>,
    pub pub_values: ConnectInfo,
}

impl<OutcomeStream, IncomeNode> RouterOutcome<OutcomeStream, IncomeNode>
    where OutcomeStream: IHttp2Stream, IncomeNode: IHttp2Node
{
    pub fn new(rsp: IncomeNode,
               stream: OutcomeStream,
               interceptors: Vec<Arc<InterceptorBox>>,
               pub_values: ConnectInfo,
    ) -> Self
    {
        return RouterOutcome {
            income_node: rsp,
            outcome_stream: stream,
            interceptors,
            pub_values,
        };
    }

    //TODO:
    fn send_gateway_error(&mut self, mut error: crate::Error) -> crate::Result<()> {
        for interceptor in &self.interceptors{
            error=interceptor.comming_error(error)
        }
        self.income_node.send_error(error);
        self.income_node._close();
        Ok(())
    }

    fn send_income_headers(&mut self, mut header: Headers,end_of_stream:bool) -> crate::Result<()> {
        for interceptor in &self.interceptors{
            header=interceptor.outcome_headers(header,self.pub_values.clone())
        }
        self.income_node.send_header(header,end_of_stream);
        Ok(())
    }

    fn send_income_data(&mut self, mut bytes: Bytes, frame_size: usize, end_stream: bool) -> crate::Result<()> {
        for interceptor in &self.interceptors{
            bytes=interceptor.outcome_frame(bytes,end_stream,frame_size as u32,self.pub_values.clone())
        }
        self.income_node.send_data(bytes, end_stream);
        Ok(())
    }

    fn close_all_with_trailer(&mut self,mut header: Headers) -> crate::Result<()> {
        //todo->trailer finish
        for interceptor in &self.interceptors{
            header=interceptor.outcome_trailer(header,self.pub_values.clone())
        }
        self.income_node.send_trailers(header);
        //self.income_node._close();
        Ok(())
    }
}


impl<OutcomeStream, IncomeNode> Stream for RouterOutcome<OutcomeStream, IncomeNode>
    where OutcomeStream: IHttp2Stream, IncomeNode: IHttp2Node
{
    type Item = crate::Result<()>;

    //先实验下。不行估计还是要解析。毕竟压缩要实现
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            let poll: Poll<Option<Http2DataType>> = unsafe {
                Pin::new_unchecked(&mut self.outcome_stream).poll_next(cx)
            };

            //TODO:这里应该是Poll::Pending
            let data = match poll {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(data)) => data,
                Poll::Ready(None) => {
                    //正常情况下应该是不存在这个状态的

                    self.send_gateway_error(crate::Error::Panic(String::from("failed")));
                    return Poll::Ready(None);
                }
            };
            match data {
                Http2DataType::Headers(header) => {

                    self.send_income_headers(header,false);
                }
                Http2DataType::BytesWithEndStream((bytes, is_end_stream)) => {
                    let frame_size=bytes.len();
                    self.send_income_data(bytes,frame_size,is_end_stream);
                    self.outcome_stream.frame_above(frame_size as u32);
                }
                Http2DataType::RST(e) => {

                    self.send_gateway_error(crate::Error::from(e));
                    return Poll::Ready(None);
                }
                Http2DataType::Trailer(header) => {
                    //好像关键没收到

                    self.close_all_with_trailer(header);
                    return Poll::Ready(None);
                }
                Http2DataType::Error(error) => {
                    self.send_gateway_error(error);
                    return Poll::Ready(None);
                }
            };

        }
    }
}

//这边也是rsp.into_future().await;



