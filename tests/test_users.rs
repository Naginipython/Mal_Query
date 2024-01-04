use mal_query::myanimelist::{user::{UpdateAnime, delete_anime}, models::Status, retrieval::{get_user_animelist, get_anime}};

// NOTE: tests require a token to be generated, and it does modify the user's database

#[tokio::test]
async fn full_update_works_as_expected() {
    let test = UpdateAnime::new(28851)
        .update_status(Status::Completed)
        .update_is_rewatching(false)
        .update_score(9)
        .expect("Score is not valid")
        .update_num_watched_episodes(2000) // for this show, 1 is correct. MAL API fixes.
        .update_priority(2)
        .expect("Priority is not valid")
        .update_num_times_rewatched(1000) //maxes out 255, it seems
        .update_rewatch_value(5)
        .expect("Rewatch Value is not valid")
        // TODO: test tags, comments, start_date, finish_date
        .update()
        .await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.status, Status::Completed);
            assert_eq!(data.is_rewatching, false);
            assert_eq!(data.score, 9);
            assert_eq!(data.num_episodes_watched, 1);
            assert_eq!(data.priority, Some(2));
            assert_eq!(data.num_times_rewatched, Some(255));
            assert_eq!(data.rewatch_value, Some(5));

            let test2 = get_user_animelist("naginis_api", 50).await;
            match test2 {
                Err(_e) => assert!(false),
                Ok(data2) => {
                    let result = data2.data.iter().find(|anime| anime.id == 28851);
                    match result {
                        None => assert!(false),
                        Some(anime) => {
                            let status = anime.list_status.as_ref().expect("My List Status was unavailable");
                            assert_eq!(status.status, Status::Completed);
                            assert_eq!(status.is_rewatching, false);
                            assert_eq!(status.score, 9);
                            assert_eq!(status.num_episodes_watched, 1);
                            assert_eq!(status.priority, Some(2));
                            assert_eq!(status.num_times_rewatched, Some(255));
                            assert_eq!(status.rewatch_value, Some(5));
                        }
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn small_update_works_as_expected() {
    let test = UpdateAnime::new(35413)
        .update_status(Status::Watching)
        .update_num_watched_episodes(4)
        .update()
        .await;
    match test {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.status, Status::Watching);
            assert_eq!(data.is_rewatching, false);
            assert_eq!(data.score, 0);
            assert_eq!(data.num_episodes_watched, 4);
            assert_eq!(data.priority, Some(0));
            assert_eq!(data.num_times_rewatched, Some(0));
            assert_eq!(data.rewatch_value, Some(0));

            let test2 = get_user_animelist("naginis_api", 50).await;
            match test2 {
                Err(_e) => assert!(false),
                Ok(data2) => {
                    let result = data2.data.iter().find(|anime| anime.id == 35413);
                    match result {
                        None => assert!(false),
                        Some(anime) => {
                            let status = anime.list_status.as_ref().expect("My List Status was unavailable");
                            assert_eq!(status.status, Status::Watching);
                            assert_eq!(status.is_rewatching, false);
                            assert_eq!(status.score, 0);
                            assert_eq!(status.num_episodes_watched, 4);
                            assert_eq!(status.priority, Some(0));
                            assert_eq!(status.num_times_rewatched, Some(0));
                            assert_eq!(status.rewatch_value, Some(0));
                        }
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn update_from_malanimedata_works_as_expected() {
    let malanimedata = get_anime(37430).await;
    match malanimedata {
        Err(_e) => assert!(false),
        Ok(data) => {
            let test = UpdateAnime::from_malanimedata(&data)
                .update_status(Status::PlanToWatch)
                .update()
                .await;

            match test {
                Err(_e) => assert!(false),
                Ok(data2) => {
                    assert_eq!(data2.status, Status::PlanToWatch);

                    let test2 = get_user_animelist("naginis_api", 50).await;
                    match test2 {
                        Err(_e) => assert!(false),
                        Ok(data2) => {
                            let result = data2.data.iter().find(|anime| anime.id == 37430);
                            match result {
                                None => assert!(false),
                                Some(anime) => {
                                    let status = anime.list_status.as_ref().expect("My List Status was unavailable");
                                    assert_eq!(status.status, Status::PlanToWatch);
                                }
                            }
                        }
                    }

                }
            }
        }
    }
}

#[tokio::test]
async fn delete_anime_works_as_expected() {
    let insert = UpdateAnime::new(34881)
        .update_status(Status::OnHold)
        .update()
        .await;
    match insert {
        Err(_e) => assert!(false),
        Ok(data) => {
            assert_eq!(data.status, Status::OnHold);

            if let Err(_e) = delete_anime(34881).await {
                assert!(false);
            }

            let test2 = get_user_animelist("naginis_api", 50).await;
            match test2 {
                Err(_e) => assert!(false),
                Ok(data2) => {
                    assert!(!data2.data.iter().any(|anime| anime.id == 34881));
                }
            }
        }
    }
}