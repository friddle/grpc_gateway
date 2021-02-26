use std::pin::Pin;
use std::result;

use bytes::Bytes;
use futures::{Future, future, Stream, stream};
use futures::future::{FutureExt, TryFutureExt};
use futures::stream::{StreamExt, TryStreamExt};
use futures::task::{Context, Poll};
use httpbis::{DataOrTrailers, EndStream, Headers, HttpStreamAfterHeaders, Response, SenderState};

use crate::{Error, NodeConf, Result, GrpcStatus};
use crate::gateway::gateway_struct::struct_types::{Http2DataType, IHttp2Node, IHttp2Stream};
use crate::gateway::httpbis_impl::httpbis_handler::{HttpBisHandlerReceiver, HttpBisHandlerSend};
use crate::gateway::httpbis_impl::httpbis_struct_income::HttpBisIncomeStream;
use crate::common::headers::{trailers, headers_500, headers_500_from_error};

pub struct HttpBisOutcomeNode {
    pub req: httpbis::ClientRequest
}

impl IHttp2Node for HttpBisOutcomeNode {
    fn send_data(&mut self, data: Bytes, end_of_stream: bool) -> Result<()> {
        let result = if end_of_stream {
            self.req.send_data_end_of_stream(data)
        } else {
            self.req.send_data(data)
        };
        result.map_err(|e| crate::Error::from(e))
    }

    fn send_header(&mut self, header: Headers,end_of_stream:bool) -> Result<()> {
        Err(crate::Error::Panic(String::from("unsupport send header inside outcome/should in begin")))
    }

    fn send_trailers(&mut self, trailer: Headers) -> Result<()> {
        self.req.send_trailers(trailer).map_err(|e| crate::Error::from(e))
    }

    fn send_error(&mut self, err: crate::Error) -> Result<()> {
        self.req.send_trailers(headers_500_from_error(err)).map_err(|e| crate::Error::from(e))
    }

    fn _close(&mut self) -> Result<()> {
        self.req.close().map_err(|e| crate::Error::from(e))
    }
}

impl HttpBisOutcomeNode {}

pub type HttpFutureStreamSend<T> = Pin<Box<dyn Stream<Item=std::result::Result<T, httpbis::Error>> + Send>>;

pub struct HttpBisOutcomeStream {
    rsp_stream: HttpFutureStreamSend<Http2DataType>
}

impl HttpBisOutcomeStream {
    fn end_stream_to_bool(end_stream: EndStream) -> bool {
        return match end_stream {
            EndStream::No => false,
            EndStream::Yes => true
        };
    }

    pub fn new(rsp: httpbis::Response) -> Self {
        //需要转换成
        let stream = Box::pin(
            rsp.0.map_ok(|(headers, rem)| {
                // NOTE: flag may be wrong for first item
                let header = stream::once(future::ok(
                    Http2DataType::Headers(headers)
                ));
                let rem = rem.map_ok(|data_or_trailer: DataOrTrailers| {
                    match data_or_trailer {
                        DataOrTrailers::Data(bytes, end_stream) =>
                            Http2DataType::BytesWithEndStream((bytes, Self::end_stream_to_bool(end_stream))),
                        DataOrTrailers::Trailers(headers) => Http2DataType::Trailer(headers)
                    }
                });
                header.chain(rem)
            })
                .try_flatten_stream()
        );
        return HttpBisOutcomeStream {
            rsp_stream: stream
        };
    }
}

impl IHttp2Stream for HttpBisOutcomeStream {}

impl Stream for HttpBisOutcomeStream {
    type Item = Http2DataType;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        return match self.rsp_stream.poll_next_unpin(cx) {
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(data))) => Poll::Ready(Some(data)),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Http2DataType::Error(crate::Error::from(e)))),
            Poll::Pending => Poll::Pending
        };
    }
}
