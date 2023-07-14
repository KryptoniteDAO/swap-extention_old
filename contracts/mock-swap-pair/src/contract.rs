use crate::error::ContractError;
use crate::helper::{Asset, AssetInfo};
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SimulationResponse};
use crate::state::{Config, CONFIG};
use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, QueryRequest, Response, StdResult, SubMsg, Uint128,
};
use std::ops::{Div, Mul};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    msg.asset_infos[0].check(deps.api)?;
    msg.asset_infos[1].check(deps.api)?;

    let config = Config {
        asset_infos: msg.asset_infos.clone(),
        swap_0_to_1_price: msg.swap_0_to_1_price.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
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
        ExecuteMsg::Swap {
            offer_asset,
            belief_price,
            max_spread,
            to,
        } => {
            offer_asset.info.check(deps.api)?;
            if !offer_asset.is_native_token() {
                return Err(ContractError::Cw20DirectSwap {});
            }

            let to_addr = if let Some(to_addr) = to {
                Some(deps.api.addr_validate(&to_addr)?)
            } else {
                None
            };
            swap(
                deps,
                env,
                info.clone(),
                info.sender,
                offer_asset,
                belief_price,
                max_spread,
                to_addr,
            )
        }
        ExecuteMsg::Update0To1Price { new_price } => update0_to1_price(deps, env, info, new_price),
    }
}

pub fn update0_to1_price(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    new_price: Uint128,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.swap_0_to_1_price = new_price;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "update_0_to_1_price")
        .add_attribute("new_price", new_price.to_string()))
}

#[allow(clippy::too_many_arguments)]
pub fn swap(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    sender: Addr,
    offer_asset: Asset,
    _belief_price: Option<Decimal>,
    _max_spread: Option<Decimal>,
    to: Option<Addr>,
) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address;
    let config = CONFIG.load(deps.storage)?;
    let asset_infos: [AssetInfo; 2] = config.asset_infos;

    let asset_info_0: AssetInfo = asset_infos[0].clone();
    let asset_info_1: AssetInfo = asset_infos[1].clone();
    let querier = deps.querier;
    let asset_info_0_balance = asset_info_0.query_pool(&querier, contract_addr.clone())?;
    let asset_info_1_balance = asset_info_0.query_pool(&querier, contract_addr.clone())?;
    if asset_info_0_balance == Uint128::zero() || asset_info_1_balance == Uint128::zero() {
        return Err(ContractError::InsufficientLiquidity {});
    };
    let mut return_amount = Uint128::new(0);
    let mut to_denom = String::new();
    if offer_asset.info.equal(&asset_info_0) {
        return_amount = config
            .swap_0_to_1_price
            .mul(offer_asset.amount)
            .div(Uint128::new(1_000_000));
        to_denom = asset_info_1.to_string();
    } else {
        return_amount =
            Uint128::new(1_000_000_000_000_000_000_000u128).div(config.swap_0_to_1_price);
        return_amount = return_amount
            .mul(offer_asset.amount)
            .div(Uint128::new(1_000_000_000_000_000u128));
        to_denom = asset_info_0.to_string();
    };
    let receiver = to.unwrap_or_else(|| sender.clone());
    let msg = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: receiver.clone().to_string(),
        amount: vec![Coin {
            denom: to_denom.clone(),
            amount: return_amount.clone(),
        }],
    }));
    let mut messages = vec![];
    messages.push(msg);
    Ok(Response::new()
        .add_submessages(messages)
        .add_attribute("action", "swap")
        .add_attribute("sender", sender.as_str())
        .add_attribute("receiver", receiver.as_str())
        .add_attribute("offer_asset", offer_asset.info.to_string())
        .add_attribute("ask_asset", "offer_asset_mock")
        .add_attribute("offer_amount", offer_asset.amount.to_string())
        .add_attribute("return_amount", return_amount.to_string())
        .add_attribute("tax_amount", "0")
        .add_attribute("spread_amount", "0")
        .add_attribute("commission_amount", "0")
        .add_attribute("maker_fee_amount", "0"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Simulation { offer_asset } => to_binary(&query_simulation(deps, offer_asset)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        asset_infos: config.asset_infos,
        swap_0_to_1_price: config.swap_0_to_1_price,
    })
}

/// ## Description
/// Returns information about a swap simulation in a [`SimulationResponse`] object.
/// ## Params
/// * **deps** is an object of type [`Deps`].
///
/// * **offer_asset** is an object of type [`Asset`]. This is the asset to swap as well as an amount of the said asset.
pub fn query_simulation(deps: Deps, offer_asset: Asset) -> StdResult<SimulationResponse> {
    let config = CONFIG.load(deps.storage)?;
    let asset_infos: [AssetInfo; 2] = config.asset_infos;

    let asset_info_0: AssetInfo = asset_infos[0].clone();
    let asset_info_1: AssetInfo = asset_infos[1].clone();

    let mut return_amount = Uint128::new(0);
    let mut to_denom = String::new();
    if offer_asset.info.equal(&asset_info_0) {
        return_amount = config
            .swap_0_to_1_price
            .mul(offer_asset.amount)
            .div(Uint128::new(1_000_000));
        to_denom = asset_info_1.to_string();
    } else {
        return_amount =
            Uint128::new(1_000_000_000_000_000_000_000u128).div(config.swap_0_to_1_price);
        return_amount = return_amount
            .mul(offer_asset.amount)
            .div(Uint128::new(1_000_000_000_000_000u128));
        to_denom = asset_info_0.to_string();
    };

    Ok(SimulationResponse {
        return_amount,
        spread_amount: Uint128::zero(),
        commission_amount: Uint128::zero(),
    })
}
