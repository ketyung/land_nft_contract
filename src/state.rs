use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};


pub struct Treasury  {

    pub wallet_address : &'static str ,

    pub percentage : u8, 
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

    pub name : Option<String>,

    pub description : Option<String>, 

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

type Trait = cw721_metadata_onchain::Trait;

impl LandNft {

    pub fn to_metadata_traits(&self) -> Vec<Trait>{

        let mut traits : Vec<Trait> = Vec::new();

        traits.push( Trait {
            display_type : Some("Total Size".to_string()),
            trait_type : "total-size".to_string(),
            value : format!("{} {}", self.total_size, self.size_unit.clone().unwrap_or("m2".to_string()))
        });

        if self.each_size.is_some() {

            traits.push( Trait {
                display_type : Some("Unit Size".to_string()),
                trait_type : "unit-size".to_string(),
                value : format!("{} {}", self.each_size.unwrap_or(0), 
                self.size_unit.clone().unwrap_or("m2".to_string()))
            });
        }
        

        if self.addr.is_some() {

            traits.push( Trait {
                display_type : Some("Address".to_string()),
                trait_type : "address".to_string(),
                value : self.addr.clone().unwrap_or("N/A".to_string())
            });
        }


        traits.push( Trait {
            display_type : Some("Total Number Of Lands".to_string()),
            trait_type : "total-lands".to_string(),
            value : format!("{}", self.total_lands)
        });
 
        traits.push( Trait {
            display_type : Some("Symbol".to_string()),
            trait_type : "symbol".to_string(),
            value : self.symbol.clone()
        });
 
        if self.other_attributes.is_some () {

            let attrbs : Vec<Attribute> = self.other_attributes.clone().unwrap_or(vec![]);
            attrbs.iter().for_each (|aa|{

                let a = aa.clone();
                traits.push (
                    Trait{
                        display_type: a.display_type,
                        trait_type : a.attribute_type,
                        value : a.value.unwrap_or("N/A".to_string()) ,
                    }
                );
            });
        }

        traits

    }
}


impl LandNft {

    pub fn new( 
        key : Option<String>, 
        name : Option<String>,
        description : Option<String>,
        owner : Addr, 
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

        
        let new_land = LandNft { key :key.clone(), 
            name : Some(name.unwrap_or(format!("Neworld Land NFT #{}", key.clone().unwrap_or("unknown.key".to_string())))) , 
            description :  Some(description.unwrap_or(format!("Neworld Land NFT #{}",  key.unwrap_or("unknown.key".to_string())))), 
            owner : owner, total_size : total_size,
            each_size : Some(each_size), size_unit : size_unit,  
            addr: Some(addr), total_lands : total_lands, price : price, 
            price_denom: pdenom, status : None, symbol : DEFAULT_LAND_NFT_SYMBOL.to_string(),
            media_types : None, royalties : None, other_attributes : None, 
            date_created : date_created, date_updated : date_created  };
        
        return new_land;

    }


    pub fn default(date_created : Timestamp) -> LandNft{

        Self::new(None,  None, None,  Addr::unchecked("xxxxx"),  0, 0, None, 
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

    pub fn default_media_type_of(&self, media_type : u8) -> Option<LandNftMediaType>  {

        if self.media_types.is_none(){

            return None;
        }

        let return_media_types : Vec<LandNftMediaType> = self.media_types.clone().unwrap();

        let col = return_media_types.into_iter().filter(|m| {    
            m.media_type == media_type && m.is_default 
        }).collect::<Vec<LandNftMediaType>>();

        let m = col.iter().next();
        if m.is_some (){
            return Some(m.unwrap().clone()); 
        }
        else {
            return None;
        }
        
    }

    pub fn default_media_type_url(&self, media_type : u8) -> Option<String>  {

        let m = self.default_media_type_of(media_type);

        if m.is_none() {
            return None;
        }

        return Some(m.unwrap().url);

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


