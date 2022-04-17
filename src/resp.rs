use crate::state::{LandNftMediaType, LandNftRoyalty, LandNft};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LandNftMediaTypesResponse {

    pub media_types : Vec<LandNftMediaType>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LandNftRoyaltiesResponse {

    pub royalties : Vec<LandNftRoyalty>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LandNftsResponse {

    pub land_nfts : Vec<LandNft>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LandNftResponse {
    
    pub land_nft : LandNft,
}
