use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum PseudoHeaderName {
    // 8.1.2.3 Request Pseudo-Header Fields
    /// `:method`
    Method = 0,
    /// `:scheme`
    Scheme = 1,
    /// `:authority`
    Authority = 2,
    /// `:path`
    Path = 3,

    // 8.1.2.4 Response Pseudo-Header Fields
    /// `:status`
    Status = 4,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct HeaderName(HeaderNameEnum);

enum HeaderNameEnum {
    Pseudo(PseudoHeaderName),
    Regular(RegularHeaderName),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Header {
    name: HeaderName,
    /// Header value.
    pub value: HeaderValue,
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("name", &self.name.name())
            .field("value", &self.value)
            .finish()
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Headers {
    // Pseudo-headers stored before regular headers
    headers: Vec<Header>,
    pseudo_count: usize,
}
