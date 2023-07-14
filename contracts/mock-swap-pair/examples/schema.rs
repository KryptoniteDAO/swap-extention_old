use cosmwasm_schema::write_api;

use mock_swap_pair::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        // migrate: MigrateMsg
    }
}
