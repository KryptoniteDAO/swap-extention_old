use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for, write_api};

use swap_extention::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, PairConfigMsg, PairConfigResponse, QueryMsg,
    SwapInfoResponse, SwapMsg,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
        // responses:
        // cw20Hook: Cw20HookMsg,
        // borrowerInfoResponse: BorrowerInfoResponse,
        // borrowerInfosResponse: BorrowerInfosResponse,
        // configResponse: ConfigResponse,
    }
    // let mut out_dir = current_dir().unwrap();
    // out_dir.push("schema");
    // create_dir_all(&out_dir).unwrap();
    // remove_schemas(&out_dir).unwrap();
    //
    // export_schema(&schema_for!(InstantiateMsg), &out_dir);
    // export_schema(&schema_for!(QueryMsg), &out_dir);
    // export_schema(&schema_for!(ExecuteMsg), &out_dir);
    // export_schema(&schema_for!(PairConfigMsg), &out_dir);
    // export_schema(&schema_for!(ConfigResponse), &out_dir);
    // export_schema(&schema_for!(SwapInfoResponse), &out_dir);
    // export_schema(&schema_for!(PairConfigResponse), &out_dir);
    // export_schema(&schema_for!(SwapMsg), &out_dir);
}
