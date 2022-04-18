use crate::resp::{LandNftMediaTypesResponse, LandNftRoyaltiesResponse, LandNftsResponse, LandNftResponse};
use cosmwasm_std::{Deps, Env, StdResult, Order};
use crate::state::{LAND_NFTS, LandNftMediaType, LandNftRoyalty, LandNft};
use cw_storage_plus::Bound;

pub fn get_land_nft(deps: Deps, _key : String ) -> StdResult<LandNftResponse>{

    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").expect(
        format!("Failed to unwrap, key not found :\"{}\"", _key).as_str());
    
    Ok (LandNftResponse { land_nft : land_nft })
}

pub fn get_land_nft_royalties (deps: Deps, _key : String ) -> StdResult<LandNftRoyaltiesResponse>{

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


pub fn get_land_nft_media_types (deps: Deps,  _key : String, media_type : u8 ) -> StdResult<LandNftMediaTypesResponse>{

    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let media_types = land_nft.all_media_types();

    let mut return_media_types : Vec<LandNftMediaType> = vec![];

    if media_types.is_some() {

        return_media_types = media_types.unwrap().into_iter().filter(|mt| mt.media_type == media_type).collect::<Vec<LandNftMediaType>>();
    }

    Ok (LandNftMediaTypesResponse { media_types : return_media_types})

}


pub const DEFAULT_LIMIT : u32 = 10;

pub const MAX_LIMIT : u32 = 20;


pub fn get_all_land_nfts(deps : Deps , start_after: Option<String>, limit: Option<u32>) 
->StdResult<LandNftsResponse> {

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let land_nfts : StdResult <Vec<LandNft>> = 
    LAND_NFTS
    //.prefix()
    .range(deps.storage, start, None, Order::Ascending)
    .take(limit)
    .map(|itm| {
        
        let (_k, v) = itm?;

        Ok(LandNft {
            key : Some(String::from_utf8(_k)?),
            owner : v.owner,
            name : v.name, 
            description : v.description, 
            total_size : v.total_size,
            each_size : v.each_size,
            size_unit : v.size_unit, 
            addr : v.addr,
            total_lands : v.total_lands, 
            price : v.price, 
            price_denom : v.price_denom, 
            status : v.status, 
            symbol : v.symbol, 
            media_types : v.media_types,
            royalties : v.royalties,
            other_attributes : v.other_attributes, 
            date_created : v.date_created,
            date_updated : v.date_updated, 
        })
    }).collect();

    Ok(LandNftsResponse {
        land_nfts: land_nfts?,
    })
}