use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use land_nft_contract::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use land_nft_contract::state::{LandNft, LandNftMediaType, LandNftRoyalty,Attribute};
use land_nft_contract::resp::{LandNftMediaTypesResponse, LandNftRoyaltiesResponse, LandNftResponse, LandNftsResponse};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(LandNft), &out_dir);
    export_schema(&schema_for!(LandNftMediaType), &out_dir);
    export_schema(&schema_for!(LandNftRoyalty), &out_dir);
    export_schema(&schema_for!(Attribute), &out_dir);
    export_schema(&schema_for!(LandNftResponse), &out_dir);
    export_schema(&schema_for!(LandNftsResponse), &out_dir);
    export_schema(&schema_for!(LandNftRoyaltiesResponse), &out_dir);
    export_schema(&schema_for!(LandNftMediaTypesResponse), &out_dir);
    
}
