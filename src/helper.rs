use std::fmt;
use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Addr, Api, QuerierWrapper, StdResult, Uint128};
use crate::querier::{query_balance, query_token_balance};

/// This enum describes available Token types.
#[cw_serde]
pub enum AssetInfo {
    /// Non-native Token
    Token { contract_addr: Addr },
    /// Native token
    NativeToken { denom: String },
}


impl fmt::Display for AssetInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetInfo::NativeToken { denom } => write!(f, "{}", denom),
            AssetInfo::Token { contract_addr } => write!(f, "{}", contract_addr),
        }
    }
}

impl AssetInfo {
    /// Returns true if the caller is a native token. Otherwise returns false.
    /// ## Params
    /// * **self** is the caller object type
    pub fn is_native_token(&self) -> bool {
        match self {
            AssetInfo::NativeToken { .. } => true,
            AssetInfo::Token { .. } => false,
        }
    }

    /// Checks whether the native coin is IBCed token or not.
    pub fn is_ibc(&self) -> bool {
        match self {
            AssetInfo::NativeToken { denom } => denom.to_lowercase().starts_with("ibc/"),
            AssetInfo::Token { .. } => false,
        }
    }

    /// Returns the balance of token in a pool.
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **pool_addr** is the address of the contract whose token balance we check.
    pub fn query_pool(&self, querier: &QuerierWrapper, pool_addr: Addr) -> StdResult<Uint128> {
        match self {
            AssetInfo::Token { contract_addr, .. } => {
                query_token_balance(querier, contract_addr.clone(), pool_addr)
            }
            AssetInfo::NativeToken { denom, .. } => {
                query_balance(querier, pool_addr, denom.to_string())
            }
        }
    }

    /// Returns True if the calling token is the same as the token specified in the input parameters.
    /// Otherwise returns False.
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **asset** is object of type [`AssetInfo`].
    pub fn equal(&self, asset: &AssetInfo) -> bool {
        match self {
            AssetInfo::Token { contract_addr, .. } => {
                let self_contract_addr = contract_addr;
                match asset {
                    AssetInfo::Token { contract_addr, .. } => self_contract_addr == contract_addr,
                    AssetInfo::NativeToken { .. } => false,
                }
            }
            AssetInfo::NativeToken { denom, .. } => {
                let self_denom = denom;
                match asset {
                    AssetInfo::Token { .. } => false,
                    AssetInfo::NativeToken { denom, .. } => self_denom == denom,
                }
            }
        }
    }

    /// If the caller object is a native token of type ['AssetInfo`] then his `denom` field converts to a byte string.
    ///
    /// If the caller object is a token of type ['AssetInfo`] then his `contract_addr` field converts to a byte string.
    /// ## Params
    /// * **self** is the type of the caller object.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            AssetInfo::NativeToken { denom } => denom.as_bytes(),
            AssetInfo::Token { contract_addr } => contract_addr.as_bytes(),
        }
    }

    /// Returns [`Ok`] if the token of type [`AssetInfo`] is in lowercase and valid. Otherwise returns [`Err`].
    /// ## Params
    /// * **self** is the type of the caller object.
    ///
    /// * **api** is a object of type [`Api`]
    pub fn check(&self, api: &dyn Api) -> StdResult<()> {
        if let AssetInfo::Token { contract_addr } = self {
            api.addr_validate(contract_addr.as_str())?;
        }

        Ok(())
    }
}

/// ## Description
/// This enum describes a Terra asset (native or CW20).
#[cw_serde]
pub struct Asset {
    /// Information about an asset stored in a [`AssetInfo`] struct
    pub info: AssetInfo,
    /// A token amount
    pub amount: Uint128,
}


/// ## Description
/// Calculates a pair key from the specified parameters in the `asset_infos` variable.
/// ## Params
/// `asset_infos` is an array with two items of type [`AssetInfo`].
pub fn pair_key(asset_infos: &[AssetInfo; 2]) -> Vec<u8> {
    let mut asset_infos = asset_infos.to_vec();
    asset_infos.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));

    [asset_infos[0].as_bytes(), asset_infos[1].as_bytes()].concat()
}
