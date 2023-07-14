use cosmwasm_schema::{cw_serde,QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use crate::helper::{Asset, AssetInfo};

/// This structure describes the parameters used for creating a contract.
#[cw_serde]
pub struct InstantiateMsg {
    /// Information about the two assets in the pool
    pub asset_infos: [AssetInfo; 2],
    pub swap_0_to_1_price: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    Swap {
        offer_asset: Asset,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    },
    Update0To1Price{
        new_price: Uint128,
    }
}

#[cw_serde]
pub struct SimulationResponse {
    /// The amount of ask assets returned by the swap
    pub return_amount: Uint128,
    /// The spread used in the swap operation
    pub spread_amount: Uint128,
    /// The amount of fees charged by the transaction
    pub commission_amount: Uint128,
}

#[cw_serde]
pub struct ConfigResponse {
   pub  asset_infos: [AssetInfo; 2],
   pub  swap_0_to_1_price: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SimulationResponse)]
    Simulation {
        offer_asset: Asset,
    },

    #[returns(ConfigResponse)]
    Config { }
}