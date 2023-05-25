use cosmwasm_schema::write_api;

use swap_extention::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        // migrate: MigrateMsg
    }
}
