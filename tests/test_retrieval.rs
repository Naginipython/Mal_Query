use mal_query::myanimelist::{retrieval::*, models::{Season, RankingType}};

#[tokio::test]
async fn does_search_anime_receive_expected_results() {
    let test = search_anime("jujutsu", 3).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.data.len(), 3);
            let s = data.titles();
            println!("{s:?}");
            s.into_iter().for_each(|anime| {
                assert!(anime.to_lowercase().contains("jujutsu"))
            })
        }
    }
}

#[tokio::test]
async fn does_get_season_receive_expected_results() {
    let test = get_season(2023, Season::Fall).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            let s = data.to_titles();
            println!("{s:?}");
            // main
            assert!(s.iter().any(|anime| anime == "Sousou no Frieren"));
            // Mal API doesn't include this, for some reason???????
            // assert!(s.iter().any(|anime| anime == "Spy x Family Season 2"));
            assert!(s.iter().any(|anime| anime == "Goblin Slayer II"));
            //leftovers
            assert!(s.iter().any(|anime| anime == "Zom 100: Zombie ni Naru made ni Shitai 100 no Koto"));
            //ona
            assert!(s.iter().any(|anime| anime == "Tensei shitara Slime Datta Ken: Coleus no Yume"));
            //movie
            assert!(s.iter().any(|anime| anime == "Boku no Hero Academia: UA Heroes Battle"));
        }
    }
}

#[tokio::test]
async fn does_get_anime_recieve_expected_results() {
    let test = get_anime(22199).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.id, 22199);
            assert_eq!(data.title, "Akame ga Kill!");
            assert_eq!(data.num_episodes,  Some(24));
            let season = data.start_season.expect("Start Season has not been filled");
            assert_eq!(season.year, 2014);
            assert_eq!(season.season, Season::Summer);
        }
    }
}

#[tokio::test]
async fn does_get_anime_url_receieve_expected_results() {
    let test = get_anime_from_url("https://myanimelist.net/anime/6594/Katanagatari").await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.id, 6594);
            assert_eq!(data.title, "Katanagatari");
            assert_eq!(data.num_episodes,  Some(12));
            let season = data.start_season.expect("Start Season has not been filled");
            assert_eq!(season.year, 2010);
            assert_eq!(season.season, Season::Winter);
        }
    }
}

#[tokio::test]
async fn does_anime_ranking_receive_expected_results() {
    let test = get_anime_rankings(RankingType::All, 10).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            // Due to the nature of the rankings changing, I believe I can only check this
            let mut rank = 1;
            data.data.iter().for_each(|anime| {
                assert_eq!(anime.rank, Some(rank));
                rank += 1;
            })
        }
    }
}

#[tokio::test]
async fn does_get_user_animelist_receieve_expected_results() {
    let test = get_user_animelist("naginis_api", 50).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            let akame = data.data.iter().find(|anime| anime.title == "Akame ga Kill!");
            match akame {
                None => assert!(false),
                Some(result) => {
                    assert_eq!(result.title, "Akame ga Kill!");
                    assert_eq!(result.list_status.as_ref().unwrap().score, 10);
                }
            }
        }
    }
}