use std::sync::Arc;

use tls_api::{TlsAcceptor, TlsConnector};

pub enum ServerTlsOption<A: TlsAcceptor> {
    Plain,
    Tls(A),
}
