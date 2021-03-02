use crate::{Interceptor, Metadata};
use crate::balance::ILoadBalance;
use rand::Rng;

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
        let mut rng=rand::thread_rng();
        let mut index:u32=rng.gen_range((0 .. client_ids.len() as u32));
        let mut client_id=client_ids[index as usize];
        return client_id
    }
}






