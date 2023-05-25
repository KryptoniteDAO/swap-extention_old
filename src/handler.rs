use cosmwasm_std::{Addr, Coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, StdError, SubMsg, to_binary, WasmMsg};
use crate::error::ContractError;
use crate::helper::{Asset, AssetInfo, pair_key};
use crate::msg::{SwapMsg};
use crate::querier::query_simulation;
use crate::state::{Config, is_address_in_whitelist, PairConfig, read_config, read_pair_config, read_swap_info_default_zero, store_config, store_pair_configs, store_swap_infos, store_swap_whitelist};


/**
 * Update the config of the contract
 */
#[allow(clippy::too_many_arguments)]
pub fn update_pair_config(deps: DepsMut, info: MessageInfo,
                          asset_infos: [AssetInfo; 2],
                          pair_address: Addr, max_spread: Option<Decimal>,
                          to: Option<Addr>) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    let mut pair_config = PairConfig {
        pair_address: pair_address.clone(),
        is_disabled: false,
        max_spread: None,
        to: None,
    };

    if let Some(max_spread) = max_spread {
        pair_config.max_spread = Some(max_spread);
    }
    if let Some(to) = to {
        pair_config.to = Some(to);
    }

    let pair_key = pair_key(&asset_infos);
    store_pair_configs(deps.storage, &pair_key, &pair_config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "update_pair_config"),
        ("pair_address", pair_address.as_str()),
        ("max_spread", max_spread.unwrap_or_default().to_string().as_str()), ]))
}


/**
 * Change the owner of the contract
 */
pub fn change_owner(deps: DepsMut, info: MessageInfo, new_owner: Addr) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.owner = new_owner.clone();
    store_config(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "change_owner"),
        ("new_owner", new_owner.as_str()),
    ]))
}

pub fn update_pair_status(deps: DepsMut, info: MessageInfo,
                          asset_infos: [AssetInfo; 2], is_disabled: bool) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    let pair_key = pair_key(&asset_infos);
    let mut pair_config = read_pair_config(deps.storage, &pair_key)?;
    pair_config.is_disabled = is_disabled;

    store_pair_configs(deps.storage, &pair_key, &pair_config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "update_pair_status"),
        ("pair_address", pair_config.pair_address.as_str()),
        ("is_disabled", pair_config.is_disabled.to_string().as_str()),
    ]))
}

pub fn update_pair_max_spread(deps: DepsMut, info: MessageInfo,
                              asset_infos: [AssetInfo; 2], max_spread: Decimal) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    let pair_key = pair_key(&asset_infos);
    let mut pair_config = read_pair_config(deps.storage, &pair_key)?;
    pair_config.max_spread = Some(max_spread);

    store_pair_configs(deps.storage, &pair_key, &pair_config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "update_pair_max_spread"),
        ("pair_address", pair_config.pair_address.as_str()),
        ("max_spread", pair_config.max_spread.unwrap_or_default().to_string().as_str()),
    ]))
}

pub fn set_whitelist(deps: DepsMut, info: MessageInfo, caller: Addr, is_whitelist: bool) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    store_swap_whitelist(deps.storage, caller, is_whitelist)?;

    Ok(Response::default())
}

/**
* U => A=>B=>SWAP=>B=>A
 * Swap the coin
 */
pub fn swap_denom(deps: DepsMut, _env: Env, info: MessageInfo, from_coin: Coin, target_denom: String) -> Result<Response, ContractError> {
    let sender = info.sender.clone();
    // check wihitelist
    if !is_address_in_whitelist(deps.storage, sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }
    if from_coin.denom == target_denom {
        return Err(ContractError::InvalidDenom {});
    }
    if from_coin.amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }

    let payment = info
        .funds
        .iter()
        .find(|x| x.denom == from_coin.denom && x.amount == from_coin.amount)
        .ok_or_else(|| {
            StdError::generic_err(format!("No {} assets are provided to swap.", from_coin.denom))
        })?;


    let asset_infos = [
        AssetInfo::NativeToken {
            denom: from_coin.denom.clone(),
        },
        AssetInfo::NativeToken {
            denom: target_denom.clone(),
        },
    ];
    let pair_key = pair_key(&asset_infos);

    let mut swap_info = read_swap_info_default_zero(deps.storage, &pair_key)?;


    let pair_config = read_pair_config(deps.storage, &pair_key)?;
    if pair_config.is_disabled {
        return Err(ContractError::PairNotFound {});
    }


    // swap
    let asset = Asset {
        amount: payment.amount,
        info: asset_infos[0].clone(),
    };
    let pair_address = pair_config.pair_address.clone();
    let offer_asset = asset.clone();
    let simulation_response = query_simulation(&deps.querier,pair_address.clone().to_string(), offer_asset.clone())?;

    swap_info.total_amount_out += simulation_response.return_amount;
    swap_info.total_amount_in += payment.amount;

    let mut _max_spread = None;
    if pair_config.max_spread.is_some() {
        _max_spread = Some(pair_config.max_spread.unwrap());
    }
    let swap = SwapMsg::Swap {
        offer_asset: asset,
        belief_price: None,
        max_spread: _max_spread,
        to: Some(pair_config.to.unwrap_or(sender.clone()).to_string()),
    };

    let sub_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: pair_address.clone().to_string(),
        msg: to_binary(&swap)?,
        funds: vec![Coin {
            denom: payment.denom.to_string(),
            amount: payment.amount,
        }],
    }));


    let res = Response::new()
        .add_submessage(sub_msg)
        .add_attributes(vec![
            ("action", "swap"),
            ("from_coin", from_coin.to_string().as_str()),
            ("target_denom", target_denom.as_str()),
        ]);

    store_swap_infos(deps.storage, &pair_key, &swap_info)?;

    Ok(res)
}



