use crate::state::{LandNftMediaType, LandNftRoyalty};
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