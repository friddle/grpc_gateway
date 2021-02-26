use bytes::Bytes;
use httpbis::{BytesDeque, HttpScheme};
use httpbis::Header;
use httpbis::Headers;

use crate::{GrpcStatus, Metadata,Error};

pub(crate) static HEADER_GRPC_STATUS: &'static str = "grpc-status";
pub(crate) static HEADER_GRPC_MESSAGE: &'static str = "grpc-message";

pub(crate) fn headers_500(grpc_status: GrpcStatus, message: String) -> Headers {
    Headers::from_vec(vec![
        Header::new(":status", "500"),
        Header::new("content-type", "application/grpc"),
        Header::new(HEADER_GRPC_STATUS, format!("{}", grpc_status as i32)),
        Header::new(HEADER_GRPC_MESSAGE, message),
    ])
}


pub(crate) fn headers_500_from_error(err:Error) -> Headers {
    Headers::from_vec(vec![
        Header::new(":status", "500"),
        Header::new("content-type", "application/grpc"),
        Header::new(HEADER_GRPC_STATUS, format!("{}", GrpcStatus::Internal as i32)),
        Header::new(HEADER_GRPC_MESSAGE, format!("error {:?}",err)),
    ])
}



pub(crate) fn headers_200(metadata: Metadata) -> Headers {
    let mut headers = Headers::from_vec(vec![
        // TODO: do not allocate
        Header::new(":status", "200"),
        Header::new("content-type", "application/grpc"),
        Header::new(HEADER_GRPC_STATUS, "0"),
    ]);
    headers.extend(metadata.into_headers());
    headers
}

/// Create HTTP response for gRPC error
pub(crate) fn grpc_error_message(message: &str) -> httpbis::SimpleHttpMessage {
    let headers = Headers::from_vec(vec![
        Header::new(":status", "200"),
        // TODO: alloc
        Header::new(
            HEADER_GRPC_STATUS,
            format!("{}", GrpcStatus::Internal.code()),
        ),
        Header::new(HEADER_GRPC_MESSAGE, message.to_owned()),
    ]);
    httpbis::SimpleHttpMessage {
        headers,
        body: BytesDeque::new(),
    }
}

pub(crate) fn init_headers(method: String, metadata: Metadata, authority: String, http_scheme: HttpScheme) -> Headers {
    let mut headers = Headers::from_vec(vec![
        Header::new(Bytes::from_static(b":method"), Bytes::from_static(b"POST")),
        // TODO: do not allocate static
        Header::new(Bytes::from_static(b":path"), method),
        Header::new(Bytes::from_static(b":authority"), authority),
        Header::new(
            Bytes::from_static(b":scheme"),
            Bytes::from_static(http_scheme.as_bytes()),
        ),
        Header::new(
            Bytes::from_static(b"content-type"),
            Bytes::from_static(b"application/grpc"),
        ),
        Header::new(Bytes::from_static(b"te"), Bytes::from_static(b"trailers")),
    ]);
    headers.extend(metadata.into_headers());
    return headers;
}


// Trailers -> Status [Status-Message] *Custom-Metadata
pub(crate) fn trailers(
    grpc_status: GrpcStatus,
    message: Option<String>,
    metadata: Metadata,
) -> Headers {
    let mut headers = Headers::from_vec(vec![Header::new(
        HEADER_GRPC_STATUS,
        format!("{}", grpc_status as i32),
    )]);
    if let Some(message) = message {
        headers.add_header(Header::new(HEADER_GRPC_MESSAGE, message));
    }
    headers.extend(metadata.into_headers());
    headers
}
