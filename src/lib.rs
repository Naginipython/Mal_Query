//! # Mal_Query
//! This crate connects to the MyAnimeList public API v2 Beta, and allows the user to intuitively receive anime
//! data.
//! # Examples
//! ```
//! use mal_query::myanimelist::{
//!     self, 
//!     builders::*, 
//!     models::*,
//!     retrieval::*,
//!     user::*
//! }
//! 
//! #[tokio::main]
//! async fn example() {
//!     // Logging in
//!     match myanimelist::login().await {
//!         Ok(()) => println!("Login Successful"),
//!         Err(e) => eprintln!("Error: {e}"),
//!     }
//! 
//!     // Searching for anime, getting 5 anime with title, id, and main_picture data
//!     let one_search: MalAnimeSearch = search_anime("One Piece", 5).await.unwrap();
//! 
//!     // Getting anime by MyAnimeList ID, getting full MyAnimeList details
//!     let one_piece_detailed: MalAnimeData = get_anime(21).await.unwrap();
//! 
//!     // Getting seasonal data, getting every available anime, with title, id, and main_picture data
//!     let winter_2023: MalAnimeSearch = get_season(2023, Season::Winter).await.unwrap();
//! 
//!     // Getting anime by MyAnimeList URL, getting full MyAnimeList details
//!     let one_piece_by_url: MalAnimeData = get_anime_from_url("https://myanimelist.net/anime/21/One_Piece").await.unwrap();
//! 
//!     // Gets the top 10 anime of all time, according to MyAnimeList, with title, id, and main_picture data
//!     let rankings: MalAnimeSearch = get_anime_rankings(RankingType::All, 10).await.unwrap();
//! 
//!     // Gets 10 anime entries from a user's database, as long as it is public, or logged in. It contains title, id, and main_picture data
//!     let test10 = get_user_animelist("naginis_api", 10).await.unwrap();
//! 
//!     // Builder to receive specific data from MyAnimeList. Here, title, id, main_picture, rank, and num_episodes
//!     // There are 3 types of Builders: Builder, SearchBuilder, and SeasonalBuilder
//!     // All have the same field addons, more in the docs
//!     let one_piece_builder: MalAnimeData = Builder::new(21)
//!         .add_rank()
//!         .add_num_episodes()
//!         .run()
//!         .await
//!         .unwrap();
//! 
//!     // UpdateAnime updates an anime in the user's database. The user must be logged in for this to work.
//!     // `new` can be replace with `from_malanimedata`, to use a MalAnimeData type
//!     let one_piece_update: ListStatus = UpdateAnime::new(21)
//!         .update_score(10)
//!         .expect("Score is invalid")
//!         .update()
//!         .await
//!         .unwrap();
//! 
//!     // UserListBuilder receives specific data from a user's list. Here, it is sorted by score, limited by 10, 
//!     // and includes their list status.
//!     let user_list = UserListBuilder::new("naginis_api")
//!         .sort(Sort::ListScore)
//!         .limit(10)
//!         .include_list_status()
//!         .run()
//!         .await
//!         .unwrap();
//! 
//!     // Useful method to show titles easier:
//!     // You can also use `.titles()` to get references if you don't want to consume the MalAnimeSearch
//!     let results: Vec<String> = one_search.to_titles();
//! }
//! ```

// TODO: feature: append_fields to builds, find intuitive way to add many fields, or Fields::All
// TODO: create getters for option layered structs, for easier access

// TODO: Models: fix authors and serialization

// TODO: Various manga builders

// TODO: update manga
// TODO: delete manga

// -- Jikan --
// TODO: get_anime
// TODO: get_anime_characters
// TODO: get_anime_staff
// etc. https://docs.api.jikan.moe/

pub mod myanimelist;

#[cfg(feature = "jikan")]
pub mod jikan;
