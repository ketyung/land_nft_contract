use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  
    AddLandNft { total_size : u64, 
        each_size : u64,
        size_unit : Option<String>,
        addr : String, 
        total_lands : u16, 
        price : u64,
        price_denom : Option<String>
    },

    AddLandNftMediaType {
        for_key : String, 
        url : String,
        media_type : u8,
        is_default : bool,
    },

    AddLandNftRoyalty {
        for_key : String, 
        creator_wallet : Addr,
        index  : u8,
        royalty : u16,
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},

    GetAllLandNfts { 
        start_after : Option<String>,
        limit: Option<u32>
    },
  
    GetLandNftMediaTypes {
        for_key : String,
        media_type : u8,
    },

    GetLandNftRoyalties {

        for_key : String,      
    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
