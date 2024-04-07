#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Auction, AUCTION, Bids, BIDS};

const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,   // removed _
    msg: InstantiateMsg, // removed _
) -> Result<Response, ContractError> {

    let auction = Auction {
        starting_bid: 200,
        current_bid: msg.starting_bid,
    };

    AUCTION.save(deps.storage, &auction)?;
    Ok(Response::new())
        
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut, // removed _ as needed later
    env: Env, // removed _ as needed later
    info: MessageInfo, // removed _ as needed later
    msg: ExecuteMsg, // remove _ as used now
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateAuction {
            starting_bid,
            current_bid,
        } => unimplemented!(),
        ExecuteMsg::Bid { amount } => unimplemented!()
        }
    }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

mod execute {
    use super::*;
    
    pub fn execute_create_auction(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        starting_bid: u128,
        current_bid: u128,
    ) -> Result<Response, ContractError> {
        let msg = ExecuteMsg::CreateAuction {
            starting_bid,
            current_bid,
        };
        execute(deps, env, info, msg)
    }
}

#[cfg(test)]
mod tests {}


