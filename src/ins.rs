use cosmwasm_std::{DepsMut, Env, Response, Addr, MessageInfo, Empty, coins, BankMsg};
use crate::error::ContractError;
use crate::state::{LAND_NFTS, LandNft, LandNftMediaType, LandNftRoyalty, LAND_NFT_COUNTER, IndexCounter, Treasury};
use std::convert::{TryFrom};

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


// refer to https://docs.opensea.io/docs/metadata-standards
pub type Metadata = cw721_metadata_onchain::Metadata;

pub type Extension = Option<Metadata>;

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

        Ok(_) => Ok(Response::new().add_attribute("method", "land-nft-instantiated")),

        Err(e) => Err(ContractError::CustomError{error : e}), 

    }
}

const DEFAULT_EXTERN_URL_PREFIX : &str = "https://neworld.techchee.com/land-nft";

pub fn mint_land_nft(mut deps: DepsMut,  _env : Env, 
    info: MessageInfo, _key : String, _extern_url_prefix : Option <String>) -> Result<Response, ContractError> {

    let deps_branch = deps.branch();
    let new_owner = info.clone().sender;
    
    let stored_land = LAND_NFTS.key(_key.as_str());

    let land_nft = stored_land.may_load(deps_branch.storage).expect("Failed to find land nft").expect(
        format!("Failed to unwrap, key not found :\"{}\"", _key).as_str());

    if land_nft.status.is_some() {

        return Err(ContractError::CustomErrorMesg{ message : 
        format!("Land NFT {} already minted or transferred", _key)});
    }

    let mut land_nft2 = land_nft.clone();

    let mut  ext_url_prefix = _extern_url_prefix ;

    if ext_url_prefix.is_none(){
        ext_url_prefix = Some(DEFAULT_EXTERN_URL_PREFIX.to_string());
    }

    let key = land_nft.clone().key.expect("Failed to unwrap land nft's key");
    let ext_url : Option<String> = Some(format!("{}/{}", ext_url_prefix.unwrap(), key));


    let mut _image_url : Option<String> = land_nft.default_media_type_url(crate::state::MEDIA_TYPE_IMAGE);
    let mut _video_url : Option<String> = land_nft.default_media_type_url(crate::state::MEDIA_TYPE_VIDEO);
    let mut _anim_url : Option<String> = land_nft.default_media_type_url(crate::state::MEDIA_TYPE_ANIMATION);
    
    
    let ext = Some(Metadata {
        description: land_nft.description,
        name: land_nft.name ,
        image : _image_url, 
        youtube_url : _video_url,
        animation_url : _anim_url, 
        external_url : ext_url.clone() ,
        ..Metadata::default()
    });

    
    let msg = cw721_base::msg::MintMsg {
        token_id: key.clone() ,
        owner: new_owner.to_string(),
        token_uri: ext_url,
        extension: ext ,
    };

    let mint_msg = cw721_base::msg::ExecuteMsg::Mint(msg.clone());

    let res = MyNftMintingContract::default().execute(deps_branch, _env.clone(), info, mint_msg);


    match res {

        Ok(_) =>  {

            let date_updated = _env.block.time;

            land_nft2.date_updated = date_updated;
            land_nft2.status = Some(crate::state::LAND_NFT_STATUS_MINTED);
            land_nft2.owner = new_owner;

            LAND_NFTS.save(deps.storage, key.as_str(), &land_nft2)?;

            let res = pay_treasuries(land_nft.price,land_nft.price_denom, None );

            if res.is_err() {

                return Err(ContractError::CustomErrorMesg{message : 
                    "some error while paying treasuries".to_string()});
            }

            let resp = res.expect("Failed to unwrap pay treasuries' response");
            
            Ok(resp.add_attribute("method", "land-nft-minted"))
        },

        Err(e) => Err(ContractError::CustomErrorMesg{message : e.to_string()}), 

    }

}

pub fn ins_and_mint_nft (mut deps: DepsMut,  _env : Env, 
    info: MessageInfo, for_key : String, _extern_url_prefix : Option <String>)-> Result<Response, ContractError> {

    let deps_branch = deps.branch();

    let res = ins_land_nft_for_minting(deps_branch, _env.clone(), info.clone(), for_key.clone());

    match res {

        Ok(_) =>{

            mint_land_nft(deps, _env, info, for_key, _extern_url_prefix)
        },

        Err(e) =>{

            return Err(ContractError::CustomErrorMesg{message : e.to_string()});
        }
    }
}


pub const TREASURIES : [Treasury; 2] = [
    Treasury {wallet_address : "terra1c4kq5cft2df40q3tr9y0uum6cpksjf0f4y7zz0", percentage : 95},
    Treasury {wallet_address : "terra19c4jcex5zkdky00qqjpu5u5usvjk7wklxsajp3", percentage : 5} ];


pub const DEFAULT_PRICE_DENOM : &str = "uusd";

#[allow(dead_code)]
fn convert(x: u64) -> f64 {
    let result = x as f64;
    if result as u64 != x {
        return 0.0;
    }
    return result;
}

pub fn pay_treasuries (total_amount : u64, _denom : Option<String>, debug : Option<bool>) -> 
Result<Response, ContractError>{

    let mut error : Option<ContractError> = None ;

    TREASURIES.iter().for_each( |t| {

        let perc : f64 = convert(t.percentage as u64) / 100.00;
        let amount = (convert(total_amount) * perc) as u64;

        if debug.is_some() && debug.unwrap_or(false) {

            println!("Paid.amount:{}:{}:{}", t.wallet_address, perc,  amount );

        }
      
        let res =  pay_treasury(t.wallet_address, amount, _denom.clone());

        match res {

            Ok(_) => error = None  ,

            Err(e) => error = Some(ContractError::CustomErrorMesg{ message : e.to_string()} ),

        }

    });

    if error.is_none() {

        Ok(Response::new().add_attribute("action", "paid-all-teasuries"))
    }
    else {

        let err = error.unwrap_or(ContractError::CustomErrorMesg{message: String::from(
            "Some Error When Paying All Treasuries!")});

        Err(err) 
    }
}

fn pay_treasury (wallet_address : &str, amount : u64, _denom : Option <String>)
-> Result<Response, ContractError>{

    if amount == 0 {
        return Err(ContractError::CustomErrorMesg {message : "Invalid Amount".to_string()});
    }

    let mut denom = String::from(DEFAULT_PRICE_DENOM);

    if _denom != None {
        denom = _denom.unwrap_or( String::from( DEFAULT_PRICE_DENOM) );
    }

    let real_amt = u128::try_from(amount);

    let bank_mesg = BankMsg::Send {
        to_address: String::from(wallet_address),
        amount: coins(real_amt.expect("Invalid Amount")  , denom)
    };

    Ok(Response::new().add_attribute("action", "approve").add_message(bank_mesg))

}