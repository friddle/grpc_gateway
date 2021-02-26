use core::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::runtime::Handle;

use crate::gateway::gateway_node::GateWayNode;
use crate::NodeConf;

//client_pool实现内部可变||这里加锁就好了。本来就应该
pub struct GateNodePools<T: GateWayNode> {
    pub _client_pool: RwLock<HashMap<String, Arc<T>>>
}

impl<T: GateWayNode> GateNodePools<T> {
    pub fn new() -> GateNodePools<T> {
        return GateNodePools {
            _client_pool: RwLock::new(HashMap::new())
        };
    }

    pub fn take_client(&self, client_id: &str, node_conf: &NodeConf, handle: Handle) -> crate::Result<Arc<T>> {
        //基本上应该不会有错误。有就write
        //按道理不应该出现
        let nodeOption = match self._client_pool.read().unwrap().get(client_id) {
            Some(node) => {
                Some(node.clone())
            }
            None => {
                None
            }
        };
        //尝试在另一个时间段看释放了没
        let node = match nodeOption {
            Some(node) => node,
            None => {
                let node = Arc::new(T::new(node_conf, Some(handle)));
                let cp_node = node.clone();
                self._client_pool.write().unwrap().insert(String::from(client_id), node);
                cp_node
            }
        };
        return Ok(node);
    }

    //这里拿出来后是否释放就搞不清楚了
    pub fn free_client(&mut self, client_id: &str) -> crate::Result<()> {
        let client = self._client_pool.write().unwrap().remove(&client_id.to_string());
        match client {
            Some(client) => Ok(()),
            None => Err(crate::Error::GateWayError(String::from("delete node failed")))
        }
    }
}

