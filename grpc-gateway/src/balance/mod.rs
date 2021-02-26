use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::{Headers, Interceptor, Metadata};
pub mod random;
pub mod default;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalanceType
{
    Default,
    Round,
    Robin,
    Weighted,
    RoundRobin,
    Random,
    Hash,
    Lc,
    WLC,
    LRT,
}

pub trait ILoadBalance: Interceptor
{
    //这里实现的时候肯定本地记录(ClientId)->
    fn pick_next<'a>(&self, path: &str, metadata: &Metadata, client_ids: &'a [&String]) -> &'a String;
}

pub type LoadBalanceBox = Arc<Box<dyn ILoadBalance>>;


