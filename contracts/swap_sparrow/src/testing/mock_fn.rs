use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, StdResult};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
use crate::contract::instantiate;
use crate::msg::InstantiateMsg;

pub const CREATOR: &str = "creator";

pub fn mock_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg {
        owner: Addr::unchecked(CREATOR.clone().to_string()),
    }
}


pub fn mock_instantiate(
    msg: InstantiateMsg,
) -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    Env,
    MessageInfo,
    StdResult<Response>,
) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
    (deps, env, info, res)
}
