use std::marker;
use std::pin::Pin;

use bytes::Bytes;
use futures::channel::mpsc;
use futures::Stream;
use futures::stream::StreamExt;
use futures::task::{Context, Poll};
use httpbis::{Headers, SenderState};

use crate::{Error, NodeConf};
use crate::gateway::gateway_struct::struct_types::{Http2DataType, IHttp2Node, IHttp2Stream};
use crate::gateway::httpbis_impl::httpbis_handler::{HttpBisHandlerReceiver, HttpBisHandlerSend};
use crate::gateway::httpbis_impl::httpbis_struct_types::HttpbisTypes;
use crate::common::headers::headers_500_from_error;

pub struct HttpBisIncomeNode {
    pub rsp: httpbis::ServerResponse,
    //pub phantom:PhantomData<&'a>
}

impl IHttp2Node for HttpBisIncomeNode {
    fn send_data(&mut self, data: Bytes, end_of_stream: bool) -> Result<(), Error> {
        let result = if end_of_stream {
            self.rsp.send_data_end_of_stream(data)
        } else {
            self.rsp.send_data(data)
        };
        result.map_err(|e| crate::Error::from(e))
    }

    fn send_header(&mut self, header: Headers,end_of_stream:bool) -> Result<(), Error> {
        if end_of_stream {
           self.rsp.send_headers_end_of_stream(header).map_err(|e| crate::Error::from(e))
        }
        else{
            self.rsp.send_headers(header).map_err(|e| crate::Error::from(e))
        }
    }

    fn send_trailers(&mut self, trailer: Headers) -> Result<(), Error> {
        self.rsp.send_trailers(trailer).map_err(|e| crate::Error::from(e))
    }

    fn send_error(&mut self, err: crate::Error) -> Result<(),Error> {
        let trailer=headers_500_from_error(err);
        if self.rsp.state()==SenderState::ExpectingHeaders{
            return self.rsp.send_headers_end_of_stream(trailer).map_err(|e| crate::Error::from(e));
        }else{
            return self.rsp.send_trailers(trailer).map_err(|e| crate::Error::from(e))
        };

    }

    fn _close(&mut self) -> Result<(), Error> {
        self.rsp.close().map_err(|e| crate::Error::from(e))
    }
}

pub struct HttpBisIncomeStream {
    pub stream: HttpBisHandlerReceiver,
}

impl IHttp2Stream for HttpBisIncomeStream {
    fn frame_processed(&mut self, frame_size: u32) -> crate::Result<()> {
        self.stream.increase_in_window.data_frame_processed(frame_size);
        self.stream.increase_in_window.increase_window_auto()?;
        Ok(())
    }

    fn frame_above(&mut self, frame_size: u32) -> crate::Result<()> {
        if frame_size != 0 {
            self.stream.increase_in_window.increase_window(frame_size);
        }
        Ok(())
    }
}

impl Stream for HttpBisIncomeStream {
    type Item = Http2DataType;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        return self.stream.receiver.poll_next_unpin(cx);
    }
}


