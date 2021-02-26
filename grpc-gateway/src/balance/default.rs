use crate::{Interceptor, Metadata};
use crate::balance::ILoadBalance;

pub struct DefaultLoadBalance
{}

impl DefaultLoadBalance {
    pub fn new() -> DefaultLoadBalance {
        return DefaultLoadBalance {};
    }
}

impl Interceptor for DefaultLoadBalance {}

impl ILoadBalance for DefaultLoadBalance {
    fn pick_next<'a>(&self, path: &str, meta: &Metadata, client_ids: &'a [&String]) -> &'a String {
        return client_ids.get(0).expect("pick client must not empty");
    }
}






