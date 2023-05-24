
use cosmwasm_std::{ entry_point,Binary, Deps, DepsMut, Env, MessageInfo, Response,  StdResult, to_binary};
use crate::error::ContractError;
use crate::handler::{change_owner, set_whitelist, swap_denom, update_pair_config, update_pair_max_spread, update_pair_status};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::querier::{query_config, query_is_swap_whitelist, query_pair_config, query_swap_info};
use crate::state::{Config, store_config};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    store_config(
        deps.storage,
        &Config {
            owner: info.sender.clone(),
        },
    )?;

    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdatePairConfig { asset_infos, pair_address, max_spread, to }
        => {
            // Validate input parameters before processing
            if asset_infos.len() != 2 {
                return Err(ContractError::InvalidParameter);
            }
            update_pair_config(deps, info, asset_infos, pair_address, max_spread, to)
        }
        ExecuteMsg::ChangeOwner { new_owner } => change_owner(deps, info, new_owner),
        ExecuteMsg::UpdatePairStatus { asset_infos, is_disabled } => {
            if asset_infos.len() != 2 {
                return Err(ContractError::InvalidParameter);
            }
            update_pair_status(deps, info, asset_infos, is_disabled)
        }
        ExecuteMsg::UpdatePairMaxSpread { asset_infos, max_spread } => {
            if asset_infos.len() != 2 {
                return Err(ContractError::InvalidParameter);
            }
            update_pair_max_spread(deps, info, asset_infos, max_spread)
        },
        ExecuteMsg::SetWhitelist { caller, is_whitelist } => set_whitelist(deps, info, caller, is_whitelist),
        ExecuteMsg::SwapDenom { from_coin, target_denom } => swap_denom(deps, env, info, from_coin, target_denom),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::QuerySwapInfo {asset_infos} => to_binary(&query_swap_info(deps,asset_infos)?),
        QueryMsg::QueryIsSwapWhitelist { caller } => to_binary(&query_is_swap_whitelist(deps, caller)?),
        QueryMsg::QueryPairConfig { asset_infos } => {
            to_binary(&query_pair_config(deps, asset_infos)?)
        },
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn proper_initialization() {}
}