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
  
    AddLandNft { 
        name : Option<String>,
        description : Option<String>,
        total_size : u64, 
        each_size : u64,
        size_unit : Option<String>,
        addr : String, 
        total_lands : u16, 
        price : u64,
        price_denom : Option<String>
    },

    UpdateLandNft { 
        for_key : String, 
        name : Option<String>,
        description : Option<String>,
        total_size : u64, 
        each_size : u64,
        size_unit : Option<String>,
        addr : String, 
        total_lands : u16, 
        price : u64,
        price_denom : Option<String>
    },

    RemoveLandNft { 
        for_key : String, 
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

    RemoveLandNftMediaType {

        for_key : String, 
        url : String,
    },

    RemoveLandNftRoyalty {
        for_key : String, 
        creator_wallet : Addr,
    },


    InstantiateMinting {
        for_key : String, 
    },

    MintLandNft {
        for_key : String, 
        external_url_prefix : Option<String>,
    },

    InsAndMintLandNft {
        for_key : String, 
        external_url_prefix : Option<String>,      
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  
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
    },

    GetLandNft {
        key : String, 
    },
    /*
    NumOfMintedTokens {
    },
    */
    AllMintedTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

}
