use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug, Default)]
pub struct Attribute {
    
    pub display_type: Option<String>,
    
    pub attribute_type: String,
    
    pub value: Option<String>,

}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.attribute_type == other.attribute_type
    }
}


pub const STATE: Item<State> = Item::new("state");

pub const MEDIA_TYPE_IMAGE : u8 = 1;

pub const MEDIA_TYPE_VIDEO : u8 = 2;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LandNftRoyalty {

    pub creator_wallet : Addr, 

    pub index : u8, 

    pub royalty : u16, 

    pub date_updated : Option<Timestamp>, 
}

impl PartialEq for LandNftRoyalty {
    fn eq(&self, other: &Self) -> bool {
        self.creator_wallet == other.creator_wallet
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LandNftMediaType {

    pub url : String, 

    pub media_type : u8, 

    pub date_updated : Option<Timestamp>, 
}

impl PartialEq for LandNftMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

pub const DEFAULT_PRICE_DENOM : &str = "uusd";

pub const LAND_NFTS : Map<&str, LandNft> = Map::new("neworld.land_nfts");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LandNft {

    pub key : Option<String>,

    pub update_authority : Addr, 

    pub size : u64, 

    pub addr : Option<String>,

    pub total_lands : u16, 

    pub price : u64, 

    pub price_denom : Option<String>,

    media_types : Option<Vec<LandNftMediaType>>,

    royalties : Option<Vec<LandNftRoyalty>>,

    other_attributes : Option<Vec<Attribute>>,

    pub date_created : Timestamp,

    pub date_updated : Timestamp, 
}

impl LandNft {

    pub fn new( update_authority : Addr, 
        size : u64, addr : String, 
        total_lands : u16, price : u64, 
        date_created : Timestamp) -> LandNft {

        let new_land = LandNft { key : None, 
            update_authority : update_authority, size : size,
            addr: Some(addr), total_lands : total_lands, price : price, 
            price_denom: Some(String::from(DEFAULT_PRICE_DENOM)),
            media_types : None, royalties : None, other_attributes : None, 
            date_created : date_created, date_updated : date_created  };
        
        return new_land;

    }
}

impl LandNft{
    pub fn add_media_type(&mut self, media_type : LandNftMediaType, date_updated : Timestamp ){

        self.date_updated = date_updated;

        if self.media_types == None {
            let v :Vec<LandNftMediaType> = Vec::new();
            self.media_types = Some(v);
        }

        if let Some(ref mut vector) = self.media_types {
            if !vector.contains(&media_type){
                vector.push(media_type);
            }
        }
    }

    pub fn remove_media_type(&mut self, url : String, date_updated : Timestamp){

        self.date_updated = date_updated;

        if let Some(ref mut vector) = self.media_types {
                
            let pos : Option<usize> = vector.iter().position(|m| *m.url == url );

            if pos.is_some() {
                vector.remove(pos.unwrap());
            }
        }
    }

    pub fn all_media_types(&self) -> Option<Vec<LandNftMediaType>>{

        self.media_types.clone()
    }


    pub fn media_type_count(&self) -> usize{

        if let Some(ref vector) = self.media_types {
            
            return vector.len();
        }

        return 0; 
    }

}

impl LandNft {

    pub fn add_royalty(&mut self, royalty : LandNftRoyalty, date_updated : Timestamp ){

        self.date_updated = date_updated;

        if self.royalties == None {
            let v :Vec<LandNftRoyalty> = Vec::new();
            self.royalties = Some(v);
        }

        if let Some(ref mut vector) = self.royalties {
            if !vector.contains(&royalty){
                vector.push(royalty);
            }
        }
    }


    pub fn remove_royalty(&mut self, creator_wallet : Addr, date_updated : Timestamp){

        self.date_updated = date_updated;

        if let Some(ref mut vector) = self.royalties {
                
            let pos : Option <usize> = vector.iter().position(|r| r.creator_wallet == creator_wallet );

            if pos.is_some() {
                vector.remove(pos.unwrap());
            }
        }
    }

    pub fn all_royalties(&self) -> Option<Vec<LandNftRoyalty>>{

        self.royalties.clone()
    }

    pub fn royalty_count(&self) -> usize{

        if let Some(ref vector) = self.royalties {
            
            return vector.len();
        }

        return 0; 
    }

}



impl LandNft {

    pub fn add_other_attribute(&mut self, attribute : Attribute ){


        if self.other_attributes == None {
            let v :Vec<Attribute> = vec![];
            self.other_attributes = Some(v);
        }

        if let Some(ref mut vector) = self.other_attributes {
            if !vector.contains(&attribute){
                vector.push(attribute);
            }
        }
    }


    pub fn remove_other_attribute(&mut self, attribute_type : String ){

        if let Some(ref mut vector) = self.other_attributes {
            
            let pos : Option<usize>  = vector.iter().position(|r| r.attribute_type == attribute_type );

            if pos.is_some() {
                vector.remove(pos.unwrap());
            }
        } 
    
    }

    pub fn all_other_attributes(&self) -> Option<Vec<Attribute>>{

        self.other_attributes.clone()
    }

    pub fn other_attributes_count(&self) -> usize{

        if let Some(ref vector) = self.other_attributes {
            
            return vector.len();
        }

        return 0; 
    }

}



