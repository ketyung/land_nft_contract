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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OptionalLandNftResponse {
    
    pub land_nft : Option<LandNft>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LandNftCountResponse {
    pub count : usize,
}
