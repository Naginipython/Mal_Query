use mal_query::myanimelist::{retrieval::*, models::{MangaMediaType, MangaRankingType}};

#[tokio::test]
async fn does_search_manga_receive_expected_results() {
    let test = search_manga("naruto", 3).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.data.len(), 3);
            let s = data.titles();
            println!("{s:?}");
            s.into_iter().for_each(|manga| {
                assert!(manga.to_lowercase().contains("naruto"))
            })
        }
    }
}

#[tokio::test]
async fn does_get_manga_recieve_expected_results() {
    let test = get_manga(115922).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.id, 115922);
            assert_eq!(data.title, "Kage no Jitsuryokusha ni Naritakute!");
            assert_eq!(data.media_type, Some(MangaMediaType::LightNovel));
        }
    }
}

#[tokio::test]
async fn does_get_manga_url_receieve_expected_results() {
    let test = get_manga_from_url("https://myanimelist.net/manga/98971/Suki_x_Suki").await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.id, 98971);
            assert_eq!(data.title, "Suki x Suki");
            assert_eq!(data.num_volumes,  Some(2));
            assert_eq!(data.num_chapters,  Some(23));
            assert_eq!(data.media_type, Some(MangaMediaType::Manga));
            assert_eq!(data.start_date, Some("2016-05-26".to_string()));
        }
    }
}

#[tokio::test]
async fn does_manga_ranking_receive_expected_results() {
    let test = get_manga_rankings(MangaRankingType::All, 10).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            // Due to the nature of the rankings changing, I believe I can only check this
            let mut rank = 1;
            data.data.iter().for_each(|manga| {
                assert_eq!(manga.rank, Some(rank));
                rank += 1;
            })
        }
    }
}

#[tokio::test]
async fn does_get_user_mangalist_receieve_expected_results() {
    let test = get_user_mangalist("naginis_api", 50).await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            let akame = data.data.iter().find(|manga| manga.title == "Akame ga Kill!");
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