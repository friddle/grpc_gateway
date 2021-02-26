use std::clone::Clone;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::{Interceptor, NodeConfs};
use crate::balance::LoadBalanceType;
use crate::common::metadata::Metadata;

pub trait GrpcDispatchHandler: Send
{
    fn is_match(&self, path: &str, metadata: &Metadata) -> bool;
    fn get_balance_type(&self) -> &LoadBalanceType;
    fn get_clients(&self) -> NodeConfs;
    fn get_weight(&self) -> i32
    {
        return 0;
    }
}

pub type GrpcDispatchBox = Box<dyn GrpcDispatchHandler + Send + Sync>;

pub mod default_dispatch;
pub mod yml_dispatch;
