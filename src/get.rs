use crate::resp::{LandNftMediaTypesResponse, LandNftRoyaltiesResponse};
use cosmwasm_std::{Deps, Env, StdResult};
use crate::state::{LAND_NFTS, LandNftMediaType, LandNftRoyalty,};


pub fn get_all_land_nft_royalties (deps: Deps,  _env : Env, _key : String ) -> StdResult<LandNftRoyaltiesResponse>{

    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let royalties = land_nft.all_royalties();

    let mut return_royalties : Vec<LandNftRoyalty> = Vec::new();

    if royalties.is_some() {

        return_royalties = royalties.unwrap();
    }

    Ok (LandNftRoyaltiesResponse { royalties : return_royalties })

}


pub fn get_all_land_nft_media_types (deps: Deps,  _env : Env, _key : String ) -> StdResult<LandNftMediaTypesResponse>{

    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let media_types = land_nft.all_media_types();

    let mut return_media_types : Vec<LandNftMediaType> = Vec::new();

    if media_types.is_some() {

        return_media_types = media_types.unwrap();
    }

    Ok (LandNftMediaTypesResponse { media_types : return_media_types})

}


pub fn get_land_nft_media_types (deps: Deps,  _env : Env, _key : String, media_type : u8 ) -> StdResult<LandNftMediaTypesResponse>{

    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let media_types = land_nft.all_media_types();

    let mut return_media_types : Vec<LandNftMediaType> = vec![];

    if media_types.is_some() {

        return_media_types = media_types.unwrap().into_iter().filter(|mt| mt.media_type == media_type).collect::<Vec<LandNftMediaType>>();
    }

    Ok (LandNftMediaTypesResponse { media_types : return_media_types})

}
