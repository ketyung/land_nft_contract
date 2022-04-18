#[cfg(test)]
mod tests {
  
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins,  Addr, from_binary};
    use crate::ins::*;
    use crate::state::*;
    use crate::get::*;
    use crate::msg::*;
    use crate::contract::*;
    use crate::resp::*;
    use std::mem::size_of;

  
  
    #[test]
    fn test_land_nft(){

        let mut deps = mock_dependencies(&coins(2, "token"));
        let info = mock_info("terra19c4jcex5zkdky00qqjpu5u5usvjk7wklxsajp3", &coins(2, "token"));
       
        let add_mesg = ExecuteMsg::AddLandNft {

            total_size : 12500, 
            each_size : 50,
            size_unit : Some("m2".to_string()),
            addr : "Tmn Kingfisher 3, Lrg Wisma Keto, H78".to_string(), 
            total_lands : 250, 
            price : 35600,
            price_denom : Some("uusd".to_string()),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), add_mesg);
        let itr = res.unwrap().attributes.into_iter();
        
        let mut wkey : Option<String> = None;

        itr.for_each(|i| if i.key == "key" { 
            wkey = Some(i.value);
        });


        let key = wkey.unwrap();
      
        for n in 1..4 {
            
            let add_media_msg = ExecuteMsg::AddLandNftMediaType {
                for_key : key.clone(), 
                url : format!( "https::/imgurl.yy/imgx_{}",n*2303),
                media_type : MEDIA_TYPE_IMAGE,
                is_default : false,
            };

            let _ = execute(deps.as_mut(), mock_env(), info.clone(), add_media_msg);

        }

        let get_msg = QueryMsg::GetLandNft{ key : key.clone()};
        let res = query(deps.as_ref(), mock_env(), get_msg).expect("Failed to unwrap res!!!");

        let value: LandNftResponse = from_binary(&res).unwrap();
        println!("get.res.value:: {:?}", value);

        let rm_msg = ExecuteMsg::RemoveLandNft{ for_key : key.clone()};

        let res = execute(deps.as_mut(), mock_env(), info, rm_msg);
        println!("remove::{:?}", res);


        println!("\n============= Get Again ===========");
        let get_msg = QueryMsg::GetLandNft{ key : key.clone()};
        let res = query(deps.as_ref(), mock_env(), get_msg).expect("Failed to unwrap res!!!");
        let value: LandNftResponse = from_binary(&res).expect("Failed to unwrap, I think it's been removed, result NOT found!!!x!!");
        println!("get.again.res.value:: {:?}", value);

        
    }

    #[test]
    fn test_add_land_nfts(){

        let mut deps = mock_dependencies(&coins(2, "token"));
        let info = mock_info("terra19c4jcex5zkdky00qqjpu5u5usvjk7wklxsajp3", &coins(2, "token"));
       

        let res = add_land_nft(deps.as_mut(),mock_env(), info.clone(), 
        12560, 50, None,  "Tmn Sinar Bak Bak, Lot 90".to_string(), 
        12, 2310, None );


        let itr = res.unwrap().attributes.into_iter();
        
        let mut wkey : Option<String> = None;

        itr.for_each(|i| if i.key == "key" { 

            wkey = Some(i.value);
        });


        let key = wkey.unwrap();
       
        println!("\n\nadded.land.nft:: {}", key.clone());
       
        for n in 1..4 {

            let res = add_land_nft_media_type(deps.as_mut(), mock_env(), info.clone(), 
                key.clone(), LandNftMediaType{
                url : format!( "https://youtube.be/772hxxh_{}", n * 1000),
                media_type : MEDIA_TYPE_VIDEO,
                is_default : false, 
                date_updated : None, 
            });
    
            println!("\n\nadded.land.nft.media.type:: {:?}", res);
    
        }

        for n in 1..3 {

            let _ = add_land_nft_media_type(deps.as_mut(), mock_env(), 
                info.clone(),
                key.clone(), LandNftMediaType{
                url : format!( "https://imgurl.ii/ImgX_{}", n * 1000),
                is_default : false, 
                media_type : MEDIA_TYPE_IMAGE,
                date_updated : None, 
            });
        }

        
        let res = get_all_land_nft_media_types(deps.as_ref(), mock_env(), key.clone());

        println!("\n\nget.all.land.nft.media.types:: {:?}", res);


        let res = get_land_nft_media_types(deps.as_ref(),key.clone(), MEDIA_TYPE_IMAGE);

        println!("\n\nget.image.land.nft.media.types:: {:?}", res);

        for n in 1..2 {

            let _ = add_land_nft_royalty(deps.as_mut(), mock_env(), info.clone(),
            key.clone(), LandNftRoyalty{
               
                creator_wallet : Addr::unchecked("terra1qchccaxyrzk8a4yxu6y2vwzc48jak8qmqm9qtg"),
                index : (n -1),
                royalty: 10000, 
                date_updated : None, 
            });
        }

        let res  = get_land_nft_royalties(deps.as_ref(),key.clone());
        println!("\n\nget.all.land.nft.royalties:: {:?}", res);


        let res  = get_all_land_nfts(deps.as_ref(), Some("key".to_string()), None);
        println!("\n\nget.all.land.nfts:: {:?}", res);

    }


    macro_rules! show_size {
        (header) => (
            println!("{:<22} {:>4}    {}", "Type", "T", "Option<T>");
        );
        ($t:ty) => (
            println!("{:<22} {:4} {:4}", stringify!($t), size_of::<$t>(), size_of::<Option<$t>>())
        )
    }
    

    #[test]
    fn test_show_sizes(){

        show_size!(Box<Vec<LandNftMediaType>>);
        show_size!(Vec<LandNftMediaType>);
        show_size!(Option<Vec<LandNftMediaType>>);
        show_size!(Vec<Box<LandNftMediaType>>);
        show_size!(LandNft);
        show_size!(cw_storage_plus::Map<&str, LandNft>);

    }
  
}
