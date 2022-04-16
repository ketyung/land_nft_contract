use cosmwasm_std::{DepsMut, Deps, Env, Response, Addr, StdResult};
use crate::error::ContractError;
use crate::state::{LAND_NFTS, LandNft, LandNftMediaType, LandNftRoyalty, LAND_NFT_COUNTER, Counter};
use crate::resp::{LandNftMediaTypesResponse, LandNftRoyaltiesResponse};


pub fn add_land_nft(deps: DepsMut,  _env : Env, 
    owner : Addr, size : u64, addr : String, 
    total_lands : u16, price : u64) -> Result<Response, ContractError> {
   
    let counter = LAND_NFT_COUNTER.update(deps.storage, |mut counter| -> Result<_, ContractError> {
        counter.increment();
        Ok(counter)
    });

    match counter {

        Ok(c) => add_land_nft_by_key(LandNft::key(c.get_count()), 
        deps, _env, owner, size, addr, total_lands, price) ,

        Err(_) => {
            
            let c = Counter::new();
            let _ = LAND_NFT_COUNTER.save(deps.storage, &c);
            let key = LandNft::key(c.get_count());
            add_land_nft_by_key(key, deps, _env, owner , size, addr, total_lands, price) 

        },
    }

}
pub fn add_land_nft_by_key(_key : String , deps: DepsMut,  _env : Env,  owner : Addr, 
    size : u64, addr : String, total_lands : u16, price : u64) -> Result<Response, ContractError> {
   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let empty = stored_land.may_load(deps.storage)?;
    assert_eq!(None, empty); // check if it wasn't previosuly added

    let date_created = _env.block.time;

    let new_land = LandNft::new ( owner ,size,
    addr, total_lands, price, date_created );

    LAND_NFTS.save(deps.storage, _key.as_str(), &new_land)?;

    Ok(Response::new().add_attribute("method", "add_land_nft"))
}



pub fn add_land_nft_royalty(deps: DepsMut,  _env : Env, _key : String ,mut royalty :LandNftRoyalty) -> Result<Response, ContractError> {
   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let date_updated = _env.block.time;
    royalty.date_updated = Some(date_updated);

    land_nft.add_royalty(royalty, date_updated);
    
    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "add_royalty"))
}



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


pub fn add_land_nft_media_type(deps: DepsMut,  _env : Env, _key : String ,mut media_type : LandNftMediaType) -> Result<Response, ContractError> {
   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let date_updated = _env.block.time;
    media_type.date_updated = Some(date_updated);

    land_nft.add_media_type(media_type, date_updated);
    
    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "add_media_type"))
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
