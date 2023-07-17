use std::str::FromStr;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cosmwasm_std::testing::mock_info;
use crate::error::ContractError;
use crate::handler::{change_owner, set_whitelist, update_pair_config, update_pair_max_spread, update_pair_status};
use crate::helper::AssetInfo;
use crate::querier::{query_config, query_is_swap_whitelist, query_pair_config, query_swap_info};
use crate::testing::mock_fn::{mock_instantiate, mock_instantiate_msg};

#[test]
fn test_instantiate() {
    let msg = mock_instantiate_msg();
    let (_, _, _, res) = mock_instantiate(msg);
    assert!(res.is_ok());
}


#[test]
fn test_update_config() {
    let msg = mock_instantiate_msg();
    let (mut deps, _, info, res) = mock_instantiate(msg);
    assert!(res.is_ok());
    // change owner
    let new_owner = Addr::unchecked("new_owner".to_string());
    let res = change_owner(deps.as_mut(), info.clone(), new_owner.clone());
    assert!(res.is_ok());

    let new_info = mock_info("new_owner", &[]);
    // update pair config failed
    let asset_infos = [AssetInfo::NativeToken { denom: "sei".to_string() }, AssetInfo::NativeToken { denom: "factory/xxx/kusd".to_string() }];
    let pair_address = Addr::unchecked("pair_address".to_string());
    let res = update_pair_config(deps.as_mut(), info.clone(),
                                 asset_infos.clone(), pair_address.clone(), None, None);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), ContractError::Unauthorized {});

    // success
    let res = update_pair_config(deps.as_mut(), new_info.clone(),
                                 asset_infos.clone(), pair_address.clone(), None, None);
    assert!(res.is_ok());


    //update pair status failed
    let res = update_pair_status(deps.as_mut(), info.clone(),
                                 asset_infos.clone(), false);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), ContractError::Unauthorized {});

    // success
    let res = update_pair_status(deps.as_mut(), new_info.clone(),
                                 asset_infos.clone(), true);
    assert!(res.is_ok());

    // update pair max spread failed
    let res = update_pair_max_spread(deps.as_mut(), info.clone(),
                                     asset_infos.clone(), Decimal::new(Uint128::from(1000u128)));
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), ContractError::Unauthorized {});

    // success
    let res = update_pair_max_spread(deps.as_mut(), new_info.clone(),
                                     asset_infos.clone(), Decimal::new(Uint128::from(1000u128)));
    assert!(res.is_ok());

    // set whitelist failed
    let res = set_whitelist(deps.as_mut(), info.clone(), Addr::unchecked("whitelist".to_string()), true);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), ContractError::Unauthorized {});

    // success
    let res = set_whitelist(deps.as_mut(), new_info.clone(), Addr::unchecked("whitelist".to_string()), true);
    assert!(res.is_ok());

    //update owner failed
    let res = change_owner(deps.as_mut(), info.clone(), new_owner.clone());
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), ContractError::Unauthorized {});

    // success
    let res = change_owner(deps.as_mut(), new_info.clone(), Addr::unchecked("new_owner_2".to_string()));
    assert!(res.is_ok());

    // query config
    let query_res = query_config(deps.as_ref()).unwrap();
    assert_eq!(query_res.owner, Addr::unchecked("new_owner_2".to_string()));

    //query swap info
    let query_res = query_swap_info(deps.as_ref(), asset_infos.clone()).unwrap();
    assert_eq!(query_res.total_amount_in, Uint128::zero());
    assert_eq!(query_res.total_amount_out, Uint128::zero());

    // query is swap white list
    let query_res = query_is_swap_whitelist(deps.as_ref(), Addr::unchecked("whitelist".to_string())).unwrap();
    assert_eq!(query_res, true);

    // let query_res = query_is_swap_whitelist(deps.as_ref(), Addr::unchecked("whitelist2".to_string()));
    // assert!(query_res.is_err());

    // query pair config
    let query_res = query_pair_config(deps.as_ref(),asset_infos.clone()).unwrap();
    assert_eq!(query_res.pair_address, pair_address);
    assert_eq!(query_res.is_disabled, true);
    assert_eq!(query_res.max_spread, Some(Decimal::from_str("0.000000000000001").unwrap()));
    assert_eq!(query_res.to, None);
}