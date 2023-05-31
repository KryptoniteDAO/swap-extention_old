use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use crate::helper::{Asset, AssetInfo};
use cosmwasm_schema::{cw_serde,QueryResponses};

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
        to_address: Option<String>,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig {},
    #[returns(bool)]
    QueryIsSwapWhitelist {
        caller: Addr
    },
    #[returns(PairConfigResponse)]
    QueryPairConfig {
        asset_infos: [AssetInfo; 2]
    },
    #[returns(SwapInfoResponse)]
    QuerySwapInfo {
        asset_infos: [AssetInfo; 2]
    },
    #[returns(SimulationResponse)]
    QuerySimulation {
        asset_infos: [AssetInfo; 2],
        offer_asset: Asset
    },
    #[returns(ReverseSimulationResponse)]
    QueryReverseSimulation {
        asset_infos: [AssetInfo; 2],
        ask_asset: Asset
    },
    #[returns(CumulativePricesResponse)]
    QueryCumulativePrices {
        asset_infos: [AssetInfo; 2],
    },
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
#[derive(QueryResponses)]
pub enum SwapQueryMsg{
    /// Returns information about a swap simulation in a [`SimulationResponse`] object.
    #[returns(SimulationResponse)]
    Simulation { offer_asset: Asset },
    /// Returns information about cumulative prices in a [`CumulativePricesResponse`] object.
    #[returns(ReverseSimulationResponse)]
    ReverseSimulation { ask_asset: Asset },
    /// Returns information about the cumulative prices in a [`CumulativePricesResponse`] object
    #[returns(CumulativePricesResponse)]
    CumulativePrices {},
}

/// This structure holds the parameters that are returned from a swap simulation response
#[cw_serde]
pub struct SimulationResponse {
    /// The amount of ask assets returned by the swap
    pub return_amount: Uint128,
    /// The spread used in the swap operation
    pub spread_amount: Uint128,
    /// The amount of fees charged by the transaction
    pub commission_amount: Uint128,
}


/// This structure holds the parameters that are returned from a reverse swap simulation response.
#[cw_serde]
pub struct ReverseSimulationResponse {
    /// The amount of offer assets returned by the reverse swap
    pub offer_amount: Uint128,
    /// The spread used in the swap operation
    pub spread_amount: Uint128,
    /// The amount of fees charged by the transaction
    pub commission_amount: Uint128,
}

/// This structure is used to return a cumulative prices query response.
#[cw_serde]
pub struct CumulativePricesResponse {
    /// The two assets in the pool to query
    pub assets: [Asset; 2],
    /// The total amount of LP tokens currently issued
    pub total_share: Uint128,
    /// The last value for the token0 cumulative price
    pub price0_cumulative_last: Uint128,
    /// The last value for the token1 cumulative price
    pub price1_cumulative_last: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}