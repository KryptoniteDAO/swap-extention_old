use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use crate::helper::{Asset, AssetInfo};
use cosmwasm_schema::{cw_serde};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
}


#[cw_serde]
pub struct PairConfigMsg {
    pub asset_infos: [AssetInfo; 2],
    pub pair_address: Addr,
    pub is_disabled: bool,
    pub max_spread: Option<Decimal>,
    pub to: Option<Addr>,
}


#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct SwapInfoResponse {
    pub total_amount_in: Uint128,
    pub total_amount_out: Uint128,
}

#[cw_serde]
pub struct PairConfigResponse {
    pub pair_address: Addr,
    pub is_disabled: bool,
    pub max_spread: Option<Decimal>,
    pub to: Option<Addr>,
}

#[cw_serde]
pub enum SwapMsg{
    Swap {
        offer_asset: Asset,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdatePairConfig {
        asset_infos: [AssetInfo; 2],
        pair_address: Addr,
        max_spread: Option<Decimal>,
        to: Option<Addr>,
    },
    ChangeOwner {
        new_owner: Addr,
    },
    UpdatePairStatus {
        asset_infos: [AssetInfo; 2],
        is_disabled: bool,
    },
    UpdatePairMaxSpread {
        asset_infos: [AssetInfo; 2],
        max_spread: Decimal,
    },
    SetWhitelist {
        caller: Addr,
        is_whitelist: bool,
    },
    SwapDenom {
        from_coin: Coin,
        target_denom: String,
    }
}

#[cw_serde]
pub enum QueryMsg {
    QueryConfig {},
    QueryIsSwapWhitelist {
        caller: Addr
    },
    QueryPairConfig {
        asset_infos: [AssetInfo; 2]
    },
    QuerySwapInfo {
        asset_infos: [AssetInfo; 2]
    },
}