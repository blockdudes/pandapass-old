use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr};

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    DelegateAll {delegate_address: Addr},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IsDelegateResponse)]
    IsDelegate {delegator: Addr, delegate: Addr},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct IsDelegateResponse {
    pub is_delegate: bool,
}
