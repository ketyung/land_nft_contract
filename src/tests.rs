#[cfg(test)]
mod tests {
  
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr};
    use crate::msg::*;
    use crate::contract::*;
    use crate::ins::*;
    use crate::state::*;
    use crate::get::*;
    use std::mem::size_of;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }



    #[test]
    fn test_add_land_nfts(){

        let mut deps = mock_dependencies(&coins(2, "token"));
        let info = mock_info("terra1qchccaxyrzk8a4yxu6y2vwzc48jak8qmqm9qtg", &coins(2, "token"));
        let owner = info.sender.clone();

        let res = add_land_nft(deps.as_mut(),mock_env(), 
        owner, 12560, "Tmn Sinar Bak Bak, Lot 90".to_string(), 
        12, 2310);


        let itr = res.unwrap().attributes.into_iter();
        
        let mut wkey : Option<String> = None;

        itr.for_each(|i| if i.key == "key" { 

            wkey = Some(i.value);
        });


        let key = wkey.unwrap();
       
        println!("\n\nadded.land.nft:: {}", key.clone());
       
        for n in 1..4 {

            let res = add_land_nft_media_type(deps.as_mut(), mock_env(), key.clone(), LandNftMediaType{
                url : format!( "https://youtube.be/772hxxh_{}", n * 1000),
                media_type : MEDIA_TYPE_VIDEO,
                is_default : false, 
                date_updated : None, 
            });
    
            println!("\n\nadded.land.nft.media.type:: {:?}", res);
    
        }

        for n in 1..3 {

            let _ = add_land_nft_media_type(deps.as_mut(), mock_env(), key.clone(), LandNftMediaType{
                url : format!( "https://imgurl.ii/ImgX_{}", n * 1000),
                is_default : false, 
                media_type : MEDIA_TYPE_IMAGE,
                date_updated : None, 
            });
        }

        
        let res = get_all_land_nft_media_types(deps.as_ref(), mock_env(), key.clone());

        println!("\n\nget.all.land.nft.media.types:: {:?}", res);


        let res = get_land_nft_media_types(deps.as_ref(), mock_env(), key.clone(), MEDIA_TYPE_IMAGE);

        println!("\n\nget.image.land.nft.media.types:: {:?}", res);

        for n in 1..2 {

            let _ = add_land_nft_royalty(deps.as_mut(), mock_env(), key.clone(), LandNftRoyalty{
               
                creator_wallet : Addr::unchecked("terra1qchccaxyrzk8a4yxu6y2vwzc48jak8qmqm9qtg"),
                index : (n -1),
                royalty: 10000, 
                date_updated : None, 
            });
        }

        let res  = get_all_land_nft_royalties(deps.as_ref(), mock_env(), key.clone());
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
