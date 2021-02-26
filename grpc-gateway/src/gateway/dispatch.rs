/*
* 这里应该是具体实现...
*  实现各种基础逻辑
*/

use core::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;

use futures::{future, StreamExt, TryFutureExt};
use futures::channel::mpsc;
use futures::channel::oneshot;
use tokio::task::JoinHandle;

use crate::{GrpcDispatchBox, InterceptorBox, LoadBalanceType, Metadata, NodeConf, NodeConfs, Server, GrpcStatus};
use crate::balance::{ILoadBalance, LoadBalanceBox};
use crate::gateway::gateway_node::GateWayNode;
use crate::gateway::gateway_struct::gate_node_pools::GateNodePools;
use crate::gateway::gateway_struct::gateway_close_message::GateWayEndMessage;
use crate::gateway::gateway_struct::gateway_context::GateWayContext;
use crate::gateway::gateway_struct::gateway_req::RouterIncome;
use crate::gateway::gateway_struct::gateway_rsp::RouterOutcome;
use crate::gateway::gateway_struct::gateway_types::GateWayTypes;
use crate::gateway::gateway_struct::struct_types::{ConnectInfo, IHttp2Node, IHttp2Stream};
use crate::gateway::gateway_utils::get_all_load_balances;
use crate::common::headers::headers_500;

//另一个项目的多态是在定义的时候生成的。不管


pub struct DispatchCore<T: GateWayTypes>
{
    //分发(到是没错。动态的类型分发)
    dispatches: Vec<Arc<GrpcDispatchBox>>,
    //拦截(也是)
    interceptors: Vec<Arc<InterceptorBox>>,
    load_balances: HashMap<LoadBalanceType, LoadBalanceBox>,
    gate_node_pool: GateNodePools<T::GateWayNode>,
}

impl<T: GateWayTypes> fmt::Debug for DispatchCore<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Core").finish()
    }
}


impl<T: GateWayTypes> DispatchCore<T>
{
    pub fn transform_type<U: GateWayTypes>(self) -> DispatchCore<U> {
        return DispatchCore {
            dispatches: self.dispatches,
            interceptors: self.interceptors,
            load_balances: get_all_load_balances(),
            gate_node_pool: GateNodePools::new(),
        };
    }


    pub fn new(mut dispatches: Vec<Arc<GrpcDispatchBox>>,
               interceptors: Vec<Arc<InterceptorBox>>) -> DispatchCore<T> {
        dispatches.sort_by_key(|dispatch| dispatch.get_weight());
        DispatchCore {
            dispatches,
            interceptors,
            load_balances: get_all_load_balances(),
            gate_node_pool: GateNodePools::new(),
        }
    }

    pub fn pick_dispatch(&self, path: &str, meta_data: &Metadata) -> crate::Result<Arc<GrpcDispatchBox>> {
        let dispatch: Vec<Arc<GrpcDispatchBox>> =
            self.dispatches.iter()
                .filter(|dispatch| dispatch.is_match(path, meta_data))
                .map(|b| b.clone())
                .collect();
        if dispatch.len() == 0
        {
            return Err(crate::Error::GateWayError(String::from(format!("dispatch fit empty {}", path))));
        }
        return Ok(dispatch.first().expect("dispatch should not empty").clone());
    }

    //--->每次pick->
    pub fn _pick_node(&self,
                      path: &str,
                      metadata: &Metadata,
                      context: &GateWayContext,
                      dispatch: &GrpcDispatchBox) -> crate::Result<Arc<T::GateWayNode>> {
        let load_balance_box: LoadBalanceBox = self.load_balances.get(dispatch.get_balance_type()).expect("need pick load balance").clone();
        let node_confs: NodeConfs = dispatch.get_clients();
        let node_confs_map: HashMap<String, &NodeConf> = node_confs.iter().map(|conf| (conf.id.clone(), conf)).collect();
        let node_conf_ids: Vec<&String> = node_confs_map.iter().map(|(it, _)| it).collect();
        let pick_node_id = load_balance_box.pick_next(path, metadata, node_conf_ids.as_slice());
        let handle = context.loop_remote();
        let node: crate::Result<Arc<T::GateWayNode>> =
            self.gate_node_pool.take_client(pick_node_id,
                                            node_confs_map.get(pick_node_id).expect("must should"),
                                            handle,
            );
        return node;
    }

    pub fn pick_node(&self,
                      path: &str,
                      metadata: &Metadata,
                      context: &GateWayContext) -> crate::Result<Arc<T::GateWayNode>> {
        let dispatch = self.pick_dispatch(&path, &metadata)?;
        let node = self._pick_node(&path, &metadata, &context, &dispatch)?;
        return Ok(node)
    }

    pub fn interceptor_path_metadata(interceptors: &Vec<Arc<InterceptorBox>>, mut path: String, mut metadata: Metadata,data:ConnectInfo) -> (String, Metadata)
    {
        for interceptor in interceptors{
            let (mut _path,mut _metadata)=interceptor.comming_req(path,metadata,data.clone());
            path=_path;
            metadata=_metadata
        }
        (path, metadata)
    }

    fn get_interceptors(interceptors: &Vec<Arc<InterceptorBox>>) -> Vec<Arc<InterceptorBox>>
    {
        interceptors
            .iter()
            .map(|it| it.clone()).
            collect()
    }


    pub fn dispatch(&self,
                    path: String,
                    metadata: Metadata,
                    context: GateWayContext,
                    mut income_node: T::GateWayIncomeNode,
                    mut income_stream: T::GateWayIncomeStream,
    ) -> crate::Result<()>
    {
        let nodeResult=self.pick_node(&path,&metadata,&context);
        let node=match nodeResult{
            Err(error)=>{
                income_node.send_error(error);
                return Ok(());
            }
            Ok(node)=>node
        };
        let result = context.
            loop_remote().spawn(
            Self::execute_box(path, metadata, node, income_stream, income_node, Self::get_interceptors(&self.interceptors))
                .map_err(|e| crate::Error::from(e))
                .map_ok(|r| ())
        );
        return Ok(());
    }

    //TODO:这里还是解决不了核心问题->Arc是不能mut的。。。
    // 然而发送接收消息需要Mut->所以感觉需要再加一个event||好像可以event_poll的时候poll下。等下改造
    // 好像用once可以解决第一个生成.await问题。---->
    pub async fn execute_box(path: String,
                             metadata: Metadata,
                             node: Arc<T::GateWayNode>,
                             income_stream: T::GateWayIncomeStream,
                             income_node: T::GateWayIncomeNode,
                             interceptors: Vec<Arc<InterceptorBox>>,
    ) -> crate::Result<()>
    {
        //TODO->interceptors->try->
        let pub_value = ConnectInfo::default();
        let (path, metadata) = Self::interceptor_path_metadata(&interceptors, path, metadata,pub_value.clone());
        let client_result = node.poll_fn(path, metadata).await;
        let (outcome_stream, outcome_node) = client_result?;
        let req =
            RouterIncome::new(
                outcome_node,
                income_stream,
                Self::get_interceptors(&interceptors),
                pub_value.clone(),
            );

        let rsp = RouterOutcome::new(
            income_node,
            outcome_stream,
            Self::get_interceptors(&interceptors),
            pub_value.clone(),
        );

        //可以每一个加上isClose的参数||也可以尝试到时候close的时候问下底层是否Close了。。。第二个靠谱些..
        future::join(StreamExt::into_future(req),
                     StreamExt::into_future(rsp)).await;
        return Result::Ok(());
    }
}


#[cfg(test)]
mod test {
    use super::*;
}

