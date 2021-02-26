use crate::gateway::gateway_node;
use crate::gateway::gateway_struct::struct_types::{IHttp2Node, IHttp2Stream};
use crate::Server;

pub trait GateWayTypes: 'static {
    type GateWayNode: gateway_node::GateWayNode;
    type GateWayIncomeNode: IHttp2Node;
    type GateWayIncomeStream: IHttp2Stream;
    type GateWayOutcomeNode: IHttp2Node;
    type GateWayOutcomeStream: IHttp2Stream;
}