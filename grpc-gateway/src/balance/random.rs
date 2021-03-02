/*
 随机分配
**/

use crate::{Interceptor, Metadata};
use crate::balance::ILoadBalance;
use rand::Rng;

pub struct RandomLoadBalance
{

}
impl Interceptor for RandomLoadBalance {}

impl RandomLoadBalance {
    pub fn new() -> RandomLoadBalance {
        return RandomLoadBalance {};
    }
}

impl ILoadBalance for RandomLoadBalance{
    fn pick_next<'a>(&self, path: &str, meta: &Metadata, client_ids: &'a [&String]) -> &'a String {
        let mut rng=rand::thread_rng();
        let mut index:u32=rng.gen_range((0 .. client_ids.len() as u32));
        let mut client_id=client_ids[index as usize];
        return client_id
    }
}



