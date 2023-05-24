use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use swap_extention::msg::{
    ExecuteMsg, InstantiateMsg, PairConfigMsg, ConfigResponse, SwapInfoResponse, PairConfigResponse, SwapMsg, QueryMsg,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(PairConfigMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(SwapInfoResponse), &out_dir);
    export_schema(&schema_for!(PairConfigResponse), &out_dir);
    export_schema(&schema_for!(SwapMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}
