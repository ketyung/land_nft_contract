use cosmwasm_std::{DepsMut, Env, Response, Addr, MessageInfo};
use crate::error::ContractError;
use crate::state::{LAND_NFTS, LandNft, LandNftMediaType, LandNftRoyalty, LAND_NFT_COUNTER, IndexCounter};


const ALLOWED_ADMINS : [&'static str; 3] = ["terra1ek2jqqyyzm8ywwp8qwp6phmsaclq3uryg48vf9",
"terra1c4kq5cft2df40q3tr9y0uum6cpksjf0f4y7zz0", "terra19c4jcex5zkdky00qqjpu5u5usvjk7wklxsajp3"];


fn is_allowed_admin( info: MessageInfo ) -> bool{

    let admin = info.sender.clone();

   
    let mut allowed : bool = false ;

    let admins = ALLOWED_ADMINS.clone();

    admins.iter().for_each( |a| { 
        if *a == admin.to_string() {
            allowed = true ; 
        }
    });

    allowed
}

pub fn add_land_nft(deps: DepsMut,  _env : Env, 
    info: MessageInfo, size : u64, addr : String, 
    total_lands : u16, price : u64) -> Result<Response, ContractError> {
   
    if !is_allowed_admin(info.clone()) {

        return Err(ContractError::Unauthorized {});
    }    

    let owner = info.clone().sender;

    let counter = LAND_NFT_COUNTER.update(deps.storage, |mut counter| -> Result<_, ContractError> {
        counter.increment();
        Ok(counter)
    });

    match counter {

        Ok(c) => add_land_nft_by_key(LandNft::key(c.get_index()), 
        deps, _env, owner, size, addr, total_lands, price) ,

        Err(_) => {
            
            let c = IndexCounter::new();
            let _ = LAND_NFT_COUNTER.save(deps.storage, &c);
            let key = LandNft::key(c.get_index());
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

    Ok(Response::new().add_attribute("key", _key).add_attribute("method", "add_land_nft"))
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

pub fn remove_land_nft_royalty(deps: DepsMut,  _env : Env, _key : String ,
    creator_wallet : Addr ) -> Result<Response, ContractError> {
   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let date_updated = _env.block.time;
   
    land_nft.remove_royalty(creator_wallet, date_updated);

    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "remove_royalty"))
}




pub fn add_land_nft_media_type(deps: DepsMut,  _env : Env, _key : String ,mut media_type : LandNftMediaType) -> Result<Response, ContractError> {
   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage)
    .expect("Failed to find land nft").expect("x.!.Failed to unwrap!!");

    let date_updated = _env.block.time;
    media_type.date_updated = Some(date_updated);

    land_nft.add_media_type(media_type, date_updated);
    
    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "add_media_type"))
}

pub fn remove_land_nft_media_type(deps: DepsMut,  _env : Env, _key : String ,
    url : String ) -> Result<Response, ContractError> {
   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let date_updated = _env.block.time;
   
    land_nft.remove_media_type(url, date_updated);
    
    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "remove_media_type"))
}
