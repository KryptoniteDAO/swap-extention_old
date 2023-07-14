use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128};
use crate::helper::AssetInfo;

/// ## Description
/// This structure stores the main config parameters for a constant product pair contract.
#[cw_serde]
pub struct Config {
    /// Asset information for the two assets in the pool
    pub asset_infos: [AssetInfo; 2],
    pub swap_0_to_1_price: Uint128,
}

/// ## Description
/// Stores the config struct at the given key
pub const CONFIG: Item<Config> = Item::new("config");