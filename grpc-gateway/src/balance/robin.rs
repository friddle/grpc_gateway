use crate::{Interceptor, Metadata};
use crate::balance::ILoadBalance;
use rand::Rng;
use std::hash::Hash;
use std::sync::Mutex;
use std::collections::HashMap;

//因为不会修改上层。需要自己保存可变性
//记得定时清0
pub struct RobinLoadBalance
{
    pub robinData:Mutex<HashMap<String,u32>>
}
impl Interceptor for RobinLoadBalance {}

impl RobinLoadBalance {
    pub fn new() -> RobinLoadBalance {
        return RobinLoadBalance {
            robinData:Default::default()
        };
    }
}

impl ILoadBalance for RobinLoadBalance{
    fn pick_next<'a>(&self, path: &str, meta: &Metadata, client_ids: &'a [&String]) -> &'a String {
        //pick max
        unimplemented!();
    }
}



