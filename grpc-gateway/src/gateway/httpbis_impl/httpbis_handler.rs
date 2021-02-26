use bytes::Bytes;
use futures::channel::mpsc;
use futures::future::err;
use httpbis::{Error, ErrorCode, Headers, ServerIncreaseInWindow};

use crate::gateway::gateway_struct::struct_types::Http2DataType;

//into_stream->

pub struct HttpBisHandlerSend {
    pub sender: mpsc::UnboundedSender<Http2DataType>,
}

pub struct HttpBisHandlerReceiver {
    pub receiver: mpsc::UnboundedReceiver<Http2DataType>,
    pub increase_in_window: ServerIncreaseInWindow,
}


impl HttpBisHandlerSend {
    pub fn new(sender: mpsc::UnboundedSender<Http2DataType>) -> HttpBisHandlerSend {
        return HttpBisHandlerSend {
            sender
        };
    }
    fn send(&mut self, message: Http2DataType) -> crate::Result<()> {
        Ok(
            self.sender.unbounded_send(message)
                .map_err(|_| crate::Error::Other("_error"))?
        )
    }
}

//送Bytes->
impl httpbis::ServerRequestStreamHandler for HttpBisHandlerSend {
    fn data_frame(&mut self, data: Bytes, end_stream: bool) -> Result<(), Error> {

        self.send(Http2DataType::BytesWithEndStream((data, end_stream)));
        Ok(())
    }

    fn trailers(&mut self, trailers: Headers) -> Result<(), Error> {

        self.send(Http2DataType::Trailer(trailers));
        Ok(())
    }

    fn rst(&mut self, error_code: ErrorCode) -> Result<(), Error> {

        self.send(Http2DataType::RST(error_code));
        Ok(())
    }

    fn error(&mut self, error: Error) -> Result<(), Error> {

        self.send(Http2DataType::Error(crate::Error::from(error)));
        Ok(())
    }
}

pub fn create_handler(increase_in_window: ServerIncreaseInWindow) -> (HttpBisHandlerSend, HttpBisHandlerReceiver) {
    let (tx, rx) = mpsc::unbounded();
    (
        HttpBisHandlerSend::new(tx),
        HttpBisHandlerReceiver {
            receiver: rx,
            increase_in_window,
        }
    )
}