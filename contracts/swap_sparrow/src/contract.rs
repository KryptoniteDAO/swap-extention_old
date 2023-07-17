use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use crate::error::ContractError;
use crate::handler::{change_owner, set_whitelist, swap_denom, update_pair_config, update_pair_max_spread, update_pair_status};
use crate::helper::pair_key;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_config, query_cumulative_prices, query_is_swap_whitelist, query_pair_config, query_reverse_simulation, query_simulation, query_swap_info};
use crate::state::{Config, read_pair_config, store_config};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    store_config(
        deps.storage,
        &Config {
            owner: msg.owner,
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
        }
        ExecuteMsg::SetWhitelist { caller, is_whitelist } => set_whitelist(deps, info, caller, is_whitelist),
        ExecuteMsg::SwapDenom { from_coin, target_denom, to_address } => swap_denom(deps, env, info, from_coin, target_denom, to_address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::QuerySwapInfo { asset_infos } => to_binary(&query_swap_info(deps, asset_infos)?),
        QueryMsg::QueryIsSwapWhitelist { caller } => to_binary(&query_is_swap_whitelist(deps, caller)?),
        QueryMsg::QueryPairConfig { asset_infos } => {
            to_binary(&query_pair_config(deps, asset_infos)?)
        }
        QueryMsg::QuerySimulation { asset_infos, offer_asset } => {
            let pair_key = pair_key(&asset_infos);
            let pair_config = read_pair_config(deps.storage, &pair_key)?;
            let contract_addr = pair_config.pair_address.clone().to_string();
            to_binary(&query_simulation(&deps.querier, contract_addr, offer_asset)?)
        }
        QueryMsg::QueryReverseSimulation { asset_infos, ask_asset } => {
            let pair_key = pair_key(&asset_infos);
            let pair_config = read_pair_config(deps.storage, &pair_key)?;
            let contract_addr = pair_config.pair_address.clone().to_string();
            to_binary(&query_reverse_simulation(&deps.querier, contract_addr, ask_asset)?)
        }
        QueryMsg::QueryCumulativePrices { asset_infos } => {
            let pair_key = pair_key(&asset_infos);
            let pair_config = read_pair_config(deps.storage, &pair_key)?;
            let contract_addr = pair_config.pair_address.clone().to_string();
            to_binary(&query_cumulative_prices(&deps.querier, contract_addr)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    #[test]
    fn proper_initialization() {}
}