#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Addr;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, IsDelegateResponse, QueryMsg};
use crate::state::{DELEGATIONS};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pandapass";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
       )
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DelegateAll { delegate_address } => {
            execute::delegate_all(deps, info, delegate_address)
        }
    }
}

pub mod execute {

    use super::*;

    pub fn delegate_all(
        deps: DepsMut,
        info: MessageInfo,
        delegate_address: Addr,
    ) -> Result<Response, ContractError> {

        DELEGATIONS.update(
            deps.storage,
            &info.sender,
            |mut v: Option<Vec<Addr>>| -> StdResult<Vec<Addr>> {
                match v.as_mut() {
                    Some(vec) => {
                        vec.push(delegate_address);
                        Ok(vec.clone())
                    }
                    None => Ok(vec![delegate_address]),
                }
            },
        )?;
        Ok(Response::new()
            .add_attribute("action", "delegate_add"))
    }

    pub fn revoke_delegation(
        deps: DepsMut,
        info: MessageInfo,
        delegate_address: Addr,
    ) -> Result<Response, ContractError> {

        DELEGATIONS.update(
            deps.storage,
            &info.sender,
            |mut v: Option<Vec<Addr>>| -> StdResult<Vec<Addr>> {
                match v.as_mut() {
                    Some(vec) => {
                        if let Some(index) = vec.iter().position(|x| x == delegate_address) {
                            vec.remove(index);
                        }
                        Ok(vec.clone())
                    }
                    None => Ok(vec![]),
                }
            },
        )?;

        Ok(Response::new()
            .add_attribute("action", "revoke_delegation"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsDelegate {
            delegator,
            delegate,
        } => to_binary(&query::is_delegate(deps, delegator, delegate)?),
    }
}

pub mod query {

    use super::*;

    pub fn is_delegate(
        deps: Deps,
        delegator: Addr,
        delegate: Addr,
    ) -> StdResult<IsDelegateResponse> {
        let delegation_array = DELEGATIONS.may_load(deps.storage, &delegator)?;
        let is_delegate = match delegation_array {
            Some(vec) => vec.contains(&delegate),
            None => false,
        };
        Ok(IsDelegateResponse { is_delegate })
    }
}
