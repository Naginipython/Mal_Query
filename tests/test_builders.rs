use mal_query::myanimelist::{builders::{Builder, AddFields, SearchBuilder, SeasonalBuilder, UserListBuilder}, models::{AiringStatus, Season, Status, Sort}};

#[tokio::test]
async fn full_builder_works_as_expected() {
    let test = Builder::new(9756)
        .add_id()
        .add_title()
        .add_main_picture()
        .add_alt_titles()
        .add_start_date()
        .add_end_date()
        .add_synopsis()
        .add_mean()
        .add_rank()
        .add_popularity()
        .add_num_list_users()
        .add_num_scoring_users()
        .add_nsfw()
        .add_created_at()
        .add_updated_at()
        .add_media_type()
        .add_status()
        .add_genres()
        .add_my_list_status()
        .add_start_season()
        .add_num_episodes()
        .add_broadcast()
        .add_source()
        .add_average_episode_duration()
        .add_rating()
        .add_pictures()
        .add_background()
        .add_related_anime()
        .add_related_manga()
        .add_recommendations()
        .add_studios()
        .add_statistics()
        .run()
        .await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.id, 9756);
            assert_eq!(data.title, "Mahou Shoujo Madoka★Magica");
            assert!(data.main_picture.large.contains("https://cdn.myanimelist.net/images/anime/"));
            assert!(data.main_picture.medium.contains("https://cdn.myanimelist.net/images/anime/"));
            // Everything else can be an option<_>. For simplicity, I'll check to ensure they're all NOT None
            assert_ne!(data.alternative_titles, None);
            assert_ne!(data.start_date, None);
            assert_ne!(data.end_date, None);
            assert_ne!(data.synopsis, None);
            assert_ne!(data.mean, None);
            assert_ne!(data.rank, None);
            assert_ne!(data.popularity, None);
            assert_ne!(data.num_list_users, None);
            assert_ne!(data.num_scoring_users, None);
            assert_ne!(data.nsfw, None);
            assert_ne!(data.created_at, None);
            assert_ne!(data.updated_at, None);
            assert_ne!(data.media_type, None);
            assert_ne!(data.status, None);
            assert_ne!(data.genres, None);
            // note: small difference
            assert_eq!(data.list_status, None);

            assert_ne!(data.start_season, None);
            assert_ne!(data.num_episodes, None);
            assert_ne!(data.broadcast, None);
            assert_ne!(data.source, None);
            assert_ne!(data.average_episode_duration, None);
            assert_ne!(data.rating, None);
            assert_ne!(data.pictures, None);
            assert_ne!(data.background, None);
            assert_ne!(data.related_anime, None);
            assert_ne!(data.related_manga, None);
            assert_ne!(data.recommendations, None);
            assert_ne!(data.studios, None);
            assert_ne!(data.statistics, None);
        }
    }
}

#[tokio::test]
async fn small_builder_works_as_expected() {
    let test = Builder::new(10620)
        .add_status()
        .add_num_episodes()
        .run()
        .await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.id, 10620);
            assert_eq!(data.title, "Mirai Nikki (TV)");
            assert!(data.main_picture.large.contains("https://cdn.myanimelist.net/images/anime/"));
            assert!(data.main_picture.medium.contains("https://cdn.myanimelist.net/images/anime/"));
            assert_eq!(data.status, Some(AiringStatus::FinishedAiring));
            assert_eq!(data.num_episodes, Some(26));
            // Remaining should be None
            assert_eq!(data.alternative_titles, None);
            assert_eq!(data.start_date, None);
            assert_eq!(data.end_date, None);
            assert_eq!(data.synopsis, None);
            assert_eq!(data.mean, None);
            assert_eq!(data.rank, None);
            assert_eq!(data.popularity, None);
            assert_eq!(data.num_list_users, None);
            assert_eq!(data.num_scoring_users, None);
            assert_eq!(data.nsfw, None);
            assert_eq!(data.created_at, None);
            assert_eq!(data.updated_at, None);
            assert_eq!(data.media_type, None);
            assert_eq!(data.genres, None);
            assert_eq!(data.list_status, None);
            assert_eq!(data.start_season, None);
            assert_eq!(data.broadcast, None);
            assert_eq!(data.source, None);
            assert_eq!(data.average_episode_duration, None);
            assert_eq!(data.rating, None);
            assert_eq!(data.pictures, None);
            assert_eq!(data.background, None);
            assert_eq!(data.related_anime, None);
            assert_eq!(data.related_manga, None);
            assert_eq!(data.recommendations, None);
            assert_eq!(data.studios, None);
            assert_eq!(data.statistics, None);
        }
    }
}

#[tokio::test]
async fn search_builder_works_as_expected() {
    let test = SearchBuilder::new("gurashi", 5)
        .add_nsfw()
        .add_rank()
        .run()
        .await;

    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            data.data.iter().for_each(|anime| {
                assert!(anime.title.to_lowercase().contains("gurashi"));
                assert_ne!(anime.nsfw, None);
                assert_ne!(anime.rank, None);
            });
        }
    }
}

#[tokio::test]
async fn seasonal_builder_works_as_expected() {
    let test = SeasonalBuilder::new(2020, Season::Spring)
        .add_start_season()
        .add_start_date()
        .run()
        .await;

    match test {
        Err(e) => { println!("{e:?}"); assert!(false) },
        Ok(data) => {
            data.data.iter().for_each(|anime| {
                // I wanted to check if it all was 2020 Spring, but turns out, 
                // it varies greatly because of leftovers
                assert_ne!(anime.start_season, None);
                assert_ne!(anime.start_date, None);
            })
        }
    }
}

#[tokio::test]
async fn full_userlist_builder_works_as_intended() {
    let test = UserListBuilder::new("naginis_api")
        .status(Status::Completed)
        .sort(Sort::ListScore)
        .limit(2)
        .offset(1)
        .include_list_status()
        .run()
        .await;

    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.data.len(), 1);
        }
    }
}

#[tokio::test]
async fn small_userlist_builder_works_as_intended() {
    let test = UserListBuilder::new("naginis_api")
        .status(Status::Watching)
        .run()
        .await;

    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.data.len(), 4);
        }
    }
}