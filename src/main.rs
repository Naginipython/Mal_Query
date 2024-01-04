#[allow(unused_imports)]
use mal_query::myanimelist::{
    builders::*, 
    models::*,
    retrieval::*, 
    user::*
};


#[tokio::main]
#[allow(unused_variables)]
async fn main() {
    // match mal_query::myanimelist::login::login().await {
    //     Ok(()) => println!("Login Successful"),
    //     Err(e) => eprintln!("Error: {e}"),
    // }
    // let test: MalAnimeSearch = search_anime("One Piece", 1).await.unwrap();
    // println!("{test:?}");
    // let test2: Vec<MalAnimeData> = get_season(2023, Season::Winter).await.unwrap();
    // println!("{test2:?}");
    // let test3: MalAnimeData = get_anime(13659).await.unwrap();
    // println!("{:?}", test3.my_list_status);
    // println!("{:?}", test3.recommendations);
    // println!("{:?}", test3.main_picture.large);
    // println!("{:?}", test3.synopsis);
    // let test4: MalAnimeData = get_anime_from_url("https://myanimelist.net/anime/8769/Ore_no_Imouto_ga_Konnani_Kawaii_Wake_ga_Nai").await.unwrap();
    // println!("{}", test4.title);
    // let test5: Vec<MalAnimeData> = get_anime_ranking(RankingType::Airing, 10).await.unwrap();
    // println!("{test5:?}");
    // let test6: MalAnimeData = Builder::new(33)
    //     .add_id()
    //     .add_num_episodes()
    //     .run()
    //     .await
    //     .unwrap();
    // println!("{test6:?}");
    // let test7: UpdateAnime = UpdateAnime::new(12291).await.unwrap();
    // let search = search_anime("Gakkougurashi", 5).await.unwrap();
    // let akame = search.get(0).unwrap();
    // let akame_detailed = get_anime(akame.id).await.unwrap();
    // println!("{:?}", &akame_detailed.my_list_status);
    // let test2 = UpdateAnime::from_malanimedata(&akame_detailed)
    //     .update_score(10)
    //     .expect("Score is wrong")
    //     .update()
    //     .await;
        
    // match test2 {
    //     Ok(data) => println!("success! {data:?}"),
    //     Err(e) => eprintln!("{e}"),
    // }
    // let test8 = SearchBuilder::new("jujutsu", 3)
    //     .add_id()
    //     .add_num_episodes()
    //     .run()
    //     .await
    //     .unwrap()
    //     .to_titles();
    // println!("{test8:?}");

    // TODO: add field=list_score or something
    let test9 = UserListBuilder::new("naginis_api")
        .sort(Sort::ListScore)
        .limit(10)
        .include_list_status()
        .run()
        .await
        .unwrap();
    println!("{:?}", test9);

    // let test10 = get_user_animelist("Naginipython", 10).await.unwrap();
    // println!("{:?}", test10.get(0).unwrap().list_status);
    
    // let test11 = search_anime("azumanga", 2).await.unwrap();
    // let search = test11.get(0).unwrap();
    // let azumanga = Builder::new(search.id)
    //     .add_my_list_status()
    //     .run()
    //     .await
    //     .unwrap();
    // println!("{azumanga:?}");

    let entry = test9.data.iter().find(|anime| anime.list_status.as_ref().unwrap().tags != Some([].to_vec())).unwrap();
    if let Err(e) = UpdateAnime::from_malanimedata(entry)
        .update_tags(["test1", "test2", "test3"].to_vec())
        .update_comments("This is a test")
        .update_start_date(2024, 1, 4)
        .update_finish_date(2024, 1, 3)
        .update()
        .await {
            eprintln!("{e}");
        }
    println!("{entry:?}");

}
