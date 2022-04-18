use cosmwasm_std::{DepsMut, Env, Response, Addr, MessageInfo, Empty};
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
    info: MessageInfo,
    name : Option<String>, 
    description : Option<String>,
    total_size : u64, 
    each_size : u64,
    size_unit : Option<String>,
    addr : String, 
    total_lands : u16, 
    price : u64,
    price_denom : Option<String>
    ) -> Result<Response, ContractError> {
   
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
        deps, _env, owner, name, description, total_size, each_size, size_unit, 
        addr, total_lands, price, price_denom) ,

        Err(_) => {
            
            let c = IndexCounter::new();
            let _ = LAND_NFT_COUNTER.save(deps.storage, &c);
            let key = LandNft::key(c.get_index());
            add_land_nft_by_key(key, deps, _env, owner, name, description,  total_size, each_size, size_unit, 
                addr, total_lands, price, price_denom) 

        },
    }

}

fn add_land_nft_by_key(_key : String , deps: DepsMut,  _env : Env,  
    owner : Addr, 
    name : Option<String>, 
    description : Option<String>,
    total_size : u64, 
    each_size : u64,
    size_unit : Option<String>,
    addr : String, 
    total_lands : u16, 
    price : u64,
    price_denom : Option<String>) -> Result<Response, ContractError> {
   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let empty = stored_land.may_load(deps.storage)?;
    assert_eq!(None, empty); // check if it wasn't previosuly added

    let date_created = _env.block.time;

    let new_land = LandNft::new (Some(_key.clone()), name, description, 
    owner ,total_size, each_size, size_unit,
    addr, total_lands, price, price_denom, date_created );

    LAND_NFTS.save(deps.storage, _key.as_str(), &new_land)?;

    Ok(Response::new().add_attribute("key", _key).add_attribute("method", "add_land_nft"))
}

pub fn update_land_nft(
    deps: DepsMut,  _env : Env,  
    info: MessageInfo,
    for_key : String , 
    name : Option<String>, 
    description : Option<String>,
    total_size : u64, 
    each_size : u64,
    size_unit : Option<String>,
    addr : String, 
    total_lands : u16, 
    price : u64,
    price_denom : Option<String>) -> Result<Response, ContractError> {
   
    
    if !is_allowed_admin(info.clone()) {

        return Err(ContractError::Unauthorized {});
    }    

    let owner = info.clone().sender;

    let stored_land = LAND_NFTS.key(for_key.as_str());
    
    let _stored_land_nft = stored_land.may_load(deps.storage)?;
   
    if _stored_land_nft.is_none() {

        return Err(ContractError::InvalidLandNft{});
    }

    let date_updated = _env.block.time;

    let new_land = LandNft::new ( 
    Some(for_key.clone()), name, description, 
    owner ,total_size, each_size, size_unit,
    addr, total_lands, price, price_denom, date_updated );


    LAND_NFTS.save(deps.storage, for_key.as_str(), &new_land)?;

    Ok(Response::new().add_attribute("key", for_key).add_attribute("method", "update_land_nft"))
}


pub fn remove_land_nft ( 
    deps: DepsMut,  
    _env : Env,  
    info: MessageInfo,
    for_key : String ) -> Result<Response, ContractError>{

    if !is_allowed_admin(info.clone()) {

        return Err(ContractError::Unauthorized {});
    }   
    
    
    let stored_land = LAND_NFTS.key(for_key.as_str());
    
    let _stored_land_nft = stored_land.may_load(deps.storage)?;
   
    if _stored_land_nft.is_none() {

        return Err(ContractError::InvalidLandNft{});
    }

    LAND_NFTS.remove(deps.storage, for_key.as_str());

    Ok(Response::new().add_attribute("method", "remove_land_nft").add_attribute("key", for_key))
    
}



pub fn add_land_nft_royalty(deps: DepsMut,  
    _env : Env, 
    info: MessageInfo,
    _key : String ,
    mut royalty :LandNftRoyalty) -> Result<Response, ContractError> {
   

    if !is_allowed_admin(info.clone()) {

        return Err(ContractError::Unauthorized {});
    }    

   
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let date_updated = _env.block.time;
    royalty.date_updated = Some(date_updated);

    land_nft.add_royalty(royalty, date_updated);
    
    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "add_royalty"))
}

pub fn remove_land_nft_royalty(deps: DepsMut,  _env : Env, 
    info: MessageInfo,
    _key : String ,
    creator_wallet : Addr ) -> Result<Response, ContractError> {
   
    if !is_allowed_admin(info.clone()) {

        return Err(ContractError::Unauthorized {});
    }    
    
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let date_updated = _env.block.time;
   
    land_nft.remove_royalty(creator_wallet, date_updated);

    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "remove_royalty"))
}




pub fn add_land_nft_media_type(deps: DepsMut,  _env : Env, 
    info: MessageInfo,
    _key : String ,
    mut media_type : LandNftMediaType) -> Result<Response, ContractError> {
   
    if !is_allowed_admin(info.clone()) {

        return Err(ContractError::Unauthorized {});
    }    
    
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage)
    .expect("Failed to find land nft").expect("x.!.Failed to unwrap!!");

    let date_updated = _env.block.time;
    media_type.date_updated = Some(date_updated);

    land_nft.add_media_type(media_type, date_updated);
    
    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "add_media_type"))
}

pub fn remove_land_nft_media_type(deps: DepsMut,  _env : Env,
    info: MessageInfo,
    _key : String ,
    url : String ) -> Result<Response, ContractError> {
   
    if !is_allowed_admin(info.clone()) {

        return Err(ContractError::Unauthorized {});
    }    
    
    let stored_land = LAND_NFTS.key(_key.as_str());
    
    let mut land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").unwrap();

    let date_updated = _env.block.time;
   
    land_nft.remove_media_type(url, date_updated);
    
    LAND_NFTS.save(deps.storage, _key.as_str(), &land_nft)?;

    Ok(Response::new().add_attribute("method", "remove_media_type"))
}


pub type Extension = Option<cw721_metadata_onchain::Metadata>;

pub type MyNftMintingContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;


pub fn ins_land_nft_for_minting(deps: DepsMut,  _env : Env, 
    info: MessageInfo, _key : String) -> Result<Response, ContractError> {

    let stored_land = LAND_NFTS.key(_key.as_str());

    let land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").expect(
        format!("Failed to unwrap, key not found :\"{}\"", _key).as_str());

    let msg =  cw721_base::InstantiateMsg {
        name: crate::contract::CONTRACT_NAME.to_string(),
        symbol: land_nft.symbol,
        minter: String::from(info.sender.clone()),
    };

    let res = MyNftMintingContract::default().instantiate(deps, _env.clone(), info.clone(),msg);

    match res {

        Ok(_) => Ok(Response::new().add_attribute("method", "land_nft_instantiated")),

        Err(e) => Err(ContractError::CustomError{error : e}), 

    }
}


pub fn mint_land_nft(deps: DepsMut,  _env : Env, 
    info: MessageInfo, _key : String) -> Result<Response, ContractError> {

    let stored_land = LAND_NFTS.key(_key.as_str());

    let land_nft = stored_land.may_load(deps.storage).expect("Failed to find land nft").expect(
        format!("Failed to unwrap, key not found :\"{}\"", _key).as_str());

    let key = land_nft.key.expect("Failed to unwrap land nft's key");
    let msg = cw721_base::msg::MintMsg {
        token_id: key.clone() ,
        owner: info.sender.clone().to_string(),
        token_uri: Some(format!("https://blog.techchee.com/{}", key)),
        extension: None ,
    };

    let mint_msg = cw721_base::msg::ExecuteMsg::Mint(msg.clone());

    let res = MyNftMintingContract::default().execute(deps, _env.clone(), info, mint_msg);


    match res {

        Ok(_) =>  {

            Ok(Response::new().add_attribute("method", "land_nft_minted"))
        },

        Err(e) => Err(ContractError::CustomErrorMesg{message : e.to_string()}), 

    }


   

}