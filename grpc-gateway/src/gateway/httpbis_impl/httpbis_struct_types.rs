use crate::gateway::gateway_struct::gateway_types::GateWayTypes;
use crate::gateway::httpbis_impl::httpbis_gatenode::HttpBisNode;
use crate::gateway::httpbis_impl::httpbis_struct_income::{HttpBisIncomeNode, HttpBisIncomeStream};
use crate::gateway::httpbis_impl::httpbis_struct_outcome::{HttpBisOutcomeNode, HttpBisOutcomeStream};
use crate::HttpBisServer;

pub struct HttpbisTypes;

impl GateWayTypes for HttpbisTypes {
    type GateWayNode = HttpBisNode;
    type GateWayIncomeNode = HttpBisIncomeNode;
    type GateWayIncomeStream = HttpBisIncomeStream;
    type GateWayOutcomeNode = HttpBisOutcomeNode;
    type GateWayOutcomeStream = HttpBisOutcomeStream;
}