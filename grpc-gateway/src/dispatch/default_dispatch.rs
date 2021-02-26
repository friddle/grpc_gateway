use std::sync::Arc;


use crate::balance::LoadBalanceType;
use crate::common::metadata::Metadata;
use crate::common::node_conf::{NodeConf, NodeConfs};
use crate::dispatch::GrpcDispatchHandler;
use crate::Interceptor;
use regex::Regex;

//这里是实现
pub struct DefaultGrpcDispatchHandler
{
    path: String,
    balance_type: LoadBalanceType,
    confs: NodeConfs,
}

impl DefaultGrpcDispatchHandler
{
    pub fn new(path: String, confs: NodeConfs) -> DefaultGrpcDispatchHandler
    {
        //这个可以有
        DefaultGrpcDispatchHandler {
            path,
            balance_type: LoadBalanceType::Default,
            confs,
        }
    }
}

impl GrpcDispatchHandler for DefaultGrpcDispatchHandler
{
    fn is_match(&self, path: &str, metadata: &Metadata) -> bool {
        let regex=Regex::new(&self.path).unwrap();
        regex.is_match(path)
    }

    fn get_balance_type(&self) -> &LoadBalanceType {
        return &self.balance_type;
    }

    fn get_clients(&self) -> NodeConfs {
        return self.confs.clone();
    }

    fn get_weight(&self) -> i32 {
        return 0;
    }
}

impl Interceptor for DefaultGrpcDispatchHandler {}