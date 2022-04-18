use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

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


pub const MEDIA_TYPE_IMAGE : u8 = 1;

pub const MEDIA_TYPE_VIDEO : u8 = 2;

pub const MEDIA_TYPE_ANIMATION : u8 = 3;

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

    pub is_default : bool,

    pub date_updated : Option<Timestamp>, 
}

impl PartialEq for LandNftMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IndexCounter {
    index : u32,
}

impl IndexCounter {

    pub fn new() -> IndexCounter{
        
        IndexCounter{ index : 1}
    }

    pub fn increment(&mut self) {

        self.index += 1;   
    }

    pub fn get_index(&self) -> u32{

        self.index 
    }
}

pub const DEFAULT_PRICE_DENOM : &str = "uusd";

pub const LAND_NFT_COUNTER: Item<IndexCounter> = Item::new("land_nft_counter");

pub const LAND_NFT_KEY_PREFIX : &str = "land_nft";

pub const LAND_NFTS : Map<&str, LandNft> = Map::new("land_nfts");

pub const LAND_NFT_STATUS_MINTED : u8 = 1;

pub const LAND_NFT_STATUS_TRANSFERRED : u8 = 2;

pub const DEFAULT_LAND_NFT_SYMBOL : &str = "neworld-land-nft";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LandNft {

    pub key : Option<String>,

    pub owner : Addr, 

    pub total_size : u64, 

    pub each_size : Option<u64>,

    pub size_unit : Option<String>,
    
    pub addr : Option<String>,

    pub total_lands : u16, 

    pub price : u64, 

    pub price_denom : Option<String>,

    pub status : Option<u8>,

    pub symbol : String, 

    pub (crate) media_types : Option<Vec<LandNftMediaType>>,

    pub (crate) royalties : Option<Vec<LandNftRoyalty>>,

    pub (crate) other_attributes : Option<Vec<Attribute>>,

    pub date_created : Timestamp,

    pub date_updated : Timestamp, 
}

impl LandNft {

    pub fn new( owner : Addr, 
        total_size : u64, 
        each_size : u64,
        size_unit : Option<String>, 
        addr : String, 
        total_lands : u16, 
        price : u64, price_denom : Option<String>, 
        date_created : Timestamp) -> LandNft {

        let mut pdenom = price_denom;

        if pdenom.is_none(){
            pdenom =  Some(String::from(DEFAULT_PRICE_DENOM));
        }

        let new_land = LandNft { key : None, 
            owner : owner, total_size : total_size,
            each_size : Some(each_size), size_unit : size_unit,  
            addr: Some(addr), total_lands : total_lands, price : price, 
            price_denom: pdenom, status : None, symbol : DEFAULT_LAND_NFT_SYMBOL.to_string(),
            media_types : None, royalties : None, other_attributes : None, 
            date_created : date_created, date_updated : date_created  };
        
        return new_land;

    }


    pub fn default(date_created : Timestamp) -> LandNft{

        Self::new( Addr::unchecked("xxxxx"),  0, 0, None, 
        String::from("xxxx"), 0,0, None,  date_created )
    }


    pub fn key(index : u32) -> String {

        format!("{}_{}", LAND_NFT_KEY_PREFIX, index )
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



