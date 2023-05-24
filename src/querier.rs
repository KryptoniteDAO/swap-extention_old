use cosmwasm_std::{Addr, BalanceResponse, BankQuery, Deps, QuerierWrapper, QueryRequest, StdResult, to_binary, Uint128, WasmQuery};
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20QueryMsg};
use crate::helper::{AssetInfo, pair_key};
use crate::msg::{ConfigResponse, PairConfigResponse, SwapInfoResponse};
use crate::state::{Config, PairConfig, read_config, read_pair_config, read_swap_info_default_zero, read_swap_whitelist, SwapInfo};

/**
 * Query the config of the oracle
 */
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = read_config(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
    })
}

pub fn query_swap_info(deps: Deps, asset_infos: [AssetInfo; 2]) -> StdResult<SwapInfoResponse> {
    let pair_key = pair_key(&asset_infos);
    let swap_info: SwapInfo = read_swap_info_default_zero(deps.storage,&pair_key)?;
    Ok(SwapInfoResponse {
        total_amount_in: swap_info.total_amount_in,
        total_amount_out: swap_info.total_amount_out,
    })
}

pub fn query_is_swap_whitelist(deps: Deps, caller: Addr) -> StdResult<bool> {
    let is_whitelist: bool = read_swap_whitelist(deps.storage, caller)?;
    Ok(is_whitelist)
}

/**
 * Query the pair config of the asset
 */
pub fn query_pair_config(deps: Deps, asset_infos: [AssetInfo; 2]) -> StdResult<PairConfigResponse> {
    let pair_key = pair_key(&asset_infos);
    let pair_config: PairConfig = read_pair_config(deps.storage, &pair_key)?;

    Ok(PairConfigResponse {
        pair_address: pair_config.pair_address,
        is_disabled: pair_config.is_disabled,
        max_spread: pair_config.max_spread,
        to: pair_config.to,
    })
}


/// Returns a token balance for an account.
/// ## Params
/// * **querier** is an object of type [`QuerierWrapper`].
///
/// * **contract_addr** is an object of type [`Addr`]. This is the token contract for which we return a balance.
///
/// * **account_addr** is an object of type [`Addr`] for which we query the token balance for.
pub fn query_token_balance(
    querier: &QuerierWrapper,
    contract_addr: Addr,
    account_addr: Addr,
) -> StdResult<Uint128> {
    // load balance from the token contract
    let res: Cw20BalanceResponse = querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: String::from(contract_addr),
            msg: to_binary(&Cw20QueryMsg::Balance {
                address: String::from(account_addr),
            })?,
        }))
        .unwrap_or_else(|_| Cw20BalanceResponse {
            balance: Uint128::zero(),
        });

    Ok(res.balance)
}


/// Returns a native token's balance for a specific account.
/// ## Params
/// * **querier** is an object of type [`QuerierWrapper`].
///
/// * **account_addr** is an object of type [`Addr`].
///
/// * **denom** is an object of type [`String`] used to specify the denomination used to return the balance (e.g uluna).
pub fn query_balance(
    querier: &QuerierWrapper,
    account_addr: Addr,
    denom: String,
) -> StdResult<Uint128> {
    let balance: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: String::from(account_addr),
        denom,
    }))?;
    Ok(balance.amount.amount)
}


#[cfg(test)]
mod tests {
    use cosmwasm_std::Decimal;
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, MockApi, MockQuerier, MockStorage};
    use crate::state::{store_config, store_pair_configs, store_swap_whitelist};

    #[test]
    fn test_query_config() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked("owner");
        let config = Config { owner: owner.clone() };
        store_config(&mut deps.storage, &config).unwrap();
        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!(res.owner, owner);
    }

    #[test]
    fn test_query_is_swap_whitelist() {
        let mut deps = mock_dependencies();
        let caller = Addr::unchecked("caller");
        store_swap_whitelist(&mut deps.storage, caller.clone(), true).unwrap();
        let res = query_is_swap_whitelist(deps.as_ref(), caller.clone()).unwrap();
        assert_eq!(res, true);
        let res = query_is_swap_whitelist(deps.as_ref(), Addr::unchecked("other")).unwrap();
        assert_eq!(res, false);
    }
    #[test]
    fn test_query_pair_config() {
        let mut deps = mock_dependencies();
        let asset_infos = [
            AssetInfo::NativeToken {
                denom: "TOKEN1".to_string(),
            },
            AssetInfo::NativeToken {
                denom: "TOKEN2".to_string(),
            },
        ];
        let pair_key = pair_key(&asset_infos);
        let pair_config = PairConfig {
            pair_address: Addr::unchecked("pair"),
            is_disabled: true,
            max_spread: Option::from(Decimal::new(Uint128::from(1000_00u128))),
            to: None,
        };
        store_pair_configs(&mut deps.storage, &pair_key, &pair_config).unwrap();
        // let res = query_pair_config(deps.as_ref(), asset_infos).unwrap();

    }

}
