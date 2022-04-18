#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{LandNftMediaType, LandNftRoyalty};
use crate::ins::{add_land_nft, add_land_nft_media_type, add_land_nft_royalty, 
    remove_land_nft_royalty, remove_land_nft_media_type, update_land_nft, remove_land_nft};
use crate::get::{get_all_land_nfts, get_land_nft_media_types, get_land_nft_royalties, get_land_nft};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
   
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
       
        ExecuteMsg::AddLandNft {
            total_size, each_size, size_unit, addr, total_lands, price, price_denom
        }=> add_land_nft(deps, _env, info, total_size, each_size, size_unit, 
            addr, total_lands, price, price_denom),

        ExecuteMsg::UpdateLandNft {
            for_key, total_size, each_size, size_unit, addr, total_lands, price, price_denom
        }=> update_land_nft(deps, _env, info, for_key, total_size, each_size, size_unit, 
            addr, total_lands, price, price_denom),
    
        ExecuteMsg::RemoveLandNft {
            for_key
        }=> remove_land_nft(deps, _env, info, for_key),

        ExecuteMsg::AddLandNftMediaType{
            for_key,
            url,
            media_type,
            is_default
        }=> {

            let media_type = LandNftMediaType{ url : url, media_type : 
                media_type, is_default : is_default, date_updated : Some(_env.block.time) };
            add_land_nft_media_type(deps, _env, info,for_key,media_type)

        },

        ExecuteMsg::AddLandNftRoyalty {
            for_key,
            creator_wallet,
            index,
            royalty,
        } => {

            let royalty = LandNftRoyalty{ creator_wallet : creator_wallet, index : index, 
                royalty : royalty, date_updated : Some(_env.block.time)};
            add_land_nft_royalty(deps, _env, info, for_key, royalty)

        },

        ExecuteMsg::RemoveLandNftMediaType{
            for_key,
            url
        }=> remove_land_nft_media_type(deps, _env, info, for_key, url),

        ExecuteMsg::RemoveLandNftRoyalty {
            for_key,
            creator_wallet,
        } => remove_land_nft_royalty(deps, _env, info,for_key, creator_wallet),

    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
       
        QueryMsg::GetAllLandNfts { start_after, limit } =>
        to_binary(&get_all_land_nfts(deps, start_after, limit)?),

        QueryMsg::GetLandNftMediaTypes { for_key, media_type} =>
        to_binary(&get_land_nft_media_types(deps, for_key, media_type )?),

        QueryMsg::GetLandNftRoyalties { for_key } => 
        to_binary(&get_land_nft_royalties(deps, for_key)?),

        QueryMsg::GetLandNft { key } => 
        to_binary( &get_land_nft(deps, key)?),
    
    }
}

