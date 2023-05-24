use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, StdError, StdResult, Storage, Uint128};

use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairConfig {
    pub pair_address: Addr,
    pub is_disabled: bool,
    pub max_spread: Option<Decimal>,
    /// None. default sender
    pub to: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapInfo {
    pub total_amount_in: Uint128,
    pub total_amount_out: Uint128,
}

pub const PAIR_CONFIGS: Map<&[u8], PairConfig> = Map::new("pair_configs");

pub const CONFIG: Item<Config> = Item::new("config");

pub const SWAP_INFOS: Map<&[u8], SwapInfo> = Map::new("swap_infos");

pub const SWAP_WHITELIST: Map<Addr, bool> = Map::new("swap_whitelist");

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    CONFIG.save(storage, config)?;
    Ok(())
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

pub fn store_pair_configs(storage: &mut dyn Storage, pair_key: &[u8], pair_config: &PairConfig) -> Result<PairConfig, StdError> {
    PAIR_CONFIGS.update(storage, pair_key, |old| match old {
        Some(_) => Ok(pair_config.clone()),
        None => Ok(pair_config.clone()),
    })
}

pub fn read_pair_config(storage: &dyn Storage, pair_key: &[u8]) -> Result<PairConfig, StdError> {
    let pair_config = PAIR_CONFIGS
        .may_load(storage, pair_key)?
        .ok_or_else(|| StdError::generic_err("Pair config not found"));
    Ok(pair_config.unwrap())
}

pub fn store_swap_infos(storage: &mut dyn Storage, pair_key: &[u8], swap_info: &SwapInfo) -> StdResult<()> {
    SWAP_INFOS.save(storage, pair_key, swap_info);
    Ok(())
}

pub fn read_swap_info(storage: &dyn Storage, pair_key: &[u8]) -> Result<SwapInfo, StdError> {
    let swap_info = SWAP_INFOS.may_load(storage, pair_key)?
        .ok_or_else(|| StdError::generic_err("Swap not found"));
    Ok(swap_info.unwrap())
}

pub fn read_swap_info_default_zero(storage: &dyn Storage, pair_key: &[u8]) -> Result<SwapInfo, StdError> {
    let swap_info = SWAP_INFOS.may_load(storage, pair_key)?;
    Ok(swap_info.unwrap_or({
        SwapInfo {
            total_amount_in: Uint128::zero(),
            total_amount_out: Uint128::zero(),
        }
    }))
}

pub fn store_swap_whitelist(storage: &mut dyn Storage, caller: Addr, is_whitelist: bool) -> StdResult<()> {
    SWAP_WHITELIST.save(storage, caller, &is_whitelist);
    Ok(())
}

pub fn read_swap_whitelist(storage: &dyn Storage, caller: Addr) -> Result<bool, StdError> {
    let is_whitelist = SWAP_WHITELIST.may_load(storage, caller)?
        .ok_or_else(|| StdError::generic_err("Swap not found"));
    Ok(is_whitelist.unwrap())
}

pub fn is_address_in_whitelist(storage: &dyn Storage, addr: Addr) -> StdResult<bool> {
    let result = SWAP_WHITELIST.may_load(storage, addr)?;
    Ok(result.unwrap_or(false))
}