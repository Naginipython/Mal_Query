use std::error::Error;

use super::{*, models::{MalAnimeData, Season, Status, Sort}};

pub struct Builder {
    url: String,
}
impl Builder {
    // TODO: describe
    pub fn new(id: u32) -> Self {
        Builder {
            url: format!("https://api.myanimelist.net/v2/anime/{id}?fields=").to_string(),
        }
    }
    // TODO: describe
    pub async fn run(&self) -> Result<MalAnimeData, Box<dyn Error>> {
        run_get(&self.url).await
    }
}

pub struct SearchBuilder {
    url: String,
}
impl SearchBuilder {
    // TODO: describe
    pub fn new(name: &str, limit: u32) -> Self {
        SearchBuilder {
            url: format!("https://api.myanimelist.net/v2/anime?q={name}&limit={limit}&fields=").to_string(),
        }
    }
    // TODO: describe
    pub async fn run(&self) -> Result<MalAnimeSearch, Box<dyn Error>> {
        run_search(&self.url).await
    }
}

pub struct SeasonalBuilder {
    url: String,
}
impl SeasonalBuilder {
    // TODO: describe
    pub fn new(year: u32, season: Season) -> Self {
        let s: &str;
        match season {
            Season::Winter => s = "winter",
            Season::Spring => s = "spring",
            Season::Summer => s = "summer",
            Season::Fall => s = "fall"
        }
        SeasonalBuilder {
            url: format!("https://api.myanimelist.net/v2/anime/season/{year}/{s}?fields=").to_string(),
        }
    }
    // TODO: describe
    pub async fn run(&self) -> Result<MalAnimeSearch, Box<dyn Error>> {
        run_search(&self.url).await
    }
}

pub struct UserListBuilder {
    url: String,
}
impl UserListBuilder {
    // TODO: describe
    pub fn new(username: &str) -> Self {
        UserListBuilder {
            url: format!("https://api.myanimelist.net/v2/users/{username}/animelist?")
        }
    }
    // TODO: describe
    pub fn status(&mut self, status: Status) -> &mut Self {
        let s: &str;
        match status {
            Status::Completed => s = "completed",
            Status::Dropped => s = "dropped",
            Status::OnHold => s = "on_hold",
            Status::PlanToWatch => s = "plan_to_watch",
            Status::Watching => s = "watching",
        }
        self.url.push_str(&format!("status={s}&"));
        self
    }
    // TODO: describe
    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        let s: &str;
        match sort {
            Sort::AnimeId => s = "anime_id",
            Sort::AnimeStartDate => s = "anime_start_date",
            Sort::AnimeTitle => s = "anime_title",
            Sort::ListScore => s = "list_score",
            Sort::ListUpdatedAt => s = "list_updated_at"
        }
        self.url.push_str(&format!("sort={s}&"));
        self
    }
    // TODO: describe
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.url.push_str(&format!("limit={limit}&"));
        self
    }
    // TODO: describe
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.url.push_str(&format!("offset={offset}&"));
        self
    }
    // TODO: describe
    pub async fn run(&self) -> Result<MalAnimeSearch, Box<dyn Error>> {
        run_search(&self.url).await
    }
}

pub trait AddFields {
    fn add_id(&mut self) -> &mut Self;
    fn add_title(&mut self) -> &mut Self;
    fn add_main_picture(&mut self) -> &mut Self;
    fn add_alt_titles(&mut self) -> &mut Self;
    fn add_start_date(&mut self) -> &mut Self;
    fn add_end_date(&mut self) -> &mut Self;
    fn add_synopsis(&mut self) -> &mut Self;
    fn add_mean(&mut self) -> &mut Self;
    fn add_rank(&mut self) -> &mut Self;
    fn add_popularity(&mut self) -> &mut Self;
    fn add_num_list_users(&mut self) -> &mut Self;
    fn add_num_scoring_users(&mut self) -> &mut Self;
    fn add_nsfw(&mut self) -> &mut Self;
    fn add_created_at(&mut self) -> &mut Self;
    fn add_updated_at(&mut self) -> &mut Self;
    fn add_media_type(&mut self) -> &mut Self;
    fn add_status(&mut self) -> &mut Self;
    fn add_genres(&mut self) -> &mut Self;
    fn add_my_list_status(&mut self) -> &mut Self;
    fn add_start_season(&mut self) -> &mut Self;
    fn add_num_episodes(&mut self) -> &mut Self;
    fn add_broadcast(&mut self) -> &mut Self;
    fn add_source(&mut self) -> &mut Self;
    fn add_average_episode_duration(&mut self) -> &mut Self;
    fn add_rating(&mut self) -> &mut Self;
    fn add_pictures(&mut self) -> &mut Self;
    fn add_background(&mut self) -> &mut Self;
    fn add_related_anime(&mut self) -> &mut Self;
    fn add_related_manga(&mut self) -> &mut Self;
    fn add_recommendations(&mut self) -> &mut Self;
    fn add_studios(&mut self) -> &mut Self;
    fn add_statistics(&mut self) -> &mut Self;
}

// ChatGPT taught me that I can do this, a macro to add the methods to each struct.
// Figured I'd rather have it one once than like, this times 3
macro_rules! impl_filters_for_builders {
    ($($struct:ident),*) => {
        $(
            impl AddFields for $struct {
                /** 
                Adds the option of an Id to be given to the final result
                */ 
                fn add_id(&mut self) -> &mut Self {
                    self.url.push_str("id,");
                    self
                }
                /** 
                Adds the option of a title to be given to the final result
                */ 
                fn add_title(&mut self) -> &mut Self {
                    self.url.push_str("title,");
                    self
                }
                /** 
                Adds the option of a Main Picture to be given to the final result
                */ 
                fn add_main_picture(&mut self) -> &mut Self {
                    self.url.push_str("main_picture,");
                    self
                }
                /** 
                Adds the option of Alternate Titles to be given to the final result
                */ 
                fn add_alt_titles(&mut self) -> &mut Self {
                    self.url.push_str("alternative_titles,");
                    self
                }
                /** 
                Adds the option of a Start Date to be given to the final result
                */ 
                fn add_start_date(&mut self) -> &mut Self {
                    self.url.push_str("start_date,");
                    self
                }
                /** 
                Adds the option of an End Date to be given to the final result
                */ 
                fn add_end_date(&mut self) -> &mut Self {
                    self.url.push_str("end_date,");
                    self
                }
                /** 
                Adds the option of a Synopsis to be given to the final result
                */ 
                fn add_synopsis(&mut self) -> &mut Self {
                    self.url.push_str("synopsis,");
                    self
                }
                /** 
                Adds the option of a Mean to be given to the final result
                */ 
                fn add_mean(&mut self) -> &mut Self {
                    self.url.push_str("mean,");
                    self
                }
                /** 
                Adds the option of a Rank to be given to the final result
                */ 
                fn add_rank(&mut self) -> &mut Self {
                    self.url.push_str("rank,");
                    self
                }
                /** 
                Adds the option of Popularity to be given to the final result
                */ 
                fn add_popularity(&mut self) -> &mut Self {
                    self.url.push_str("popularity,");
                    self
                }
                /** 
                Adds the option of Num. List Users to be given to the final result
                */ 
                fn add_num_list_users(&mut self) -> &mut Self {
                    self.url.push_str("num_list_users,");
                    self
                }
                /** 
                Adds the option of Num. Scoring Users to be given to the final result
                */ 
                fn add_num_scoring_users(&mut self) -> &mut Self {
                    self.url.push_str("num_scoring_users,");
                    self
                }
                /** 
                Adds the option of NSFW rating to be given to the final result
                */ 
                fn add_nsfw(&mut self) -> &mut Self {
                    self.url.push_str("nsfw,");
                    self
                }
                /** 
                Adds the option of a Created Date to be given to the final result
                */ 
                fn add_created_at(&mut self) -> &mut Self {
                    self.url.push_str("created_at,");
                    self
                }
                /** 
                Adds the option of an Updated Date to be given to the final result
                */ 
                fn add_updated_at(&mut self) -> &mut Self {
                    self.url.push_str("updated_at,");
                    self
                }
                /** 
                Adds the option of a Media Type to be given to the final result
                */ 
                fn add_media_type(&mut self) -> &mut Self {
                    self.url.push_str("media_type,");
                    self
                }
                /** 
                Adds the option of the Airing Status to be given to the final result
                */ 
                fn add_status(&mut self) -> &mut Self {
                    self.url.push_str("status,");
                    self
                }
                /** 
                Adds the option of Genres to be given to the final result
                */ 
                fn add_genres(&mut self) -> &mut Self {
                    self.url.push_str("genres,");
                    self
                }
                /** 
                Adds the option of your List Status to be given to the final result
                */ 
                fn add_my_list_status(&mut self) -> &mut Self {
                    self.url.push_str("my_list_status,");
                    self
                }
                /** 
                Adds the option of Start Season data to be given to the final result
                */ 
                fn add_start_season(&mut self) -> &mut Self {
                    self.url.push_str("start_season,");
                    self
                }
                /** 
                Adds the option of an Episode Count to be given to the final result
                */ 
                fn add_num_episodes(&mut self) -> &mut Self {
                    self.url.push_str("num_episodes,");
                    self
                }
                /** 
                Adds the option of Broadcast data to be given to the final result
                */ 
                fn add_broadcast(&mut self) -> &mut Self {
                    self.url.push_str("broadcast,");
                    self
                }
                /** 
                Adds the option of Source data to be given to the final result
                */ 
                fn add_source(&mut self) -> &mut Self {
                    self.url.push_str("source,");
                    self
                }
                /** 
                Adds the option of an Average Episode Duration to be given to the final result
                */ 
                fn add_average_episode_duration(&mut self) -> &mut Self {
                    self.url.push_str("average_episode_duration,");
                    self
                }
                /** 
                Adds the option of a Rating type to be given to the final result
                */ 
                fn add_rating(&mut self) -> &mut Self {
                    self.url.push_str("rating,");
                    self
                }
                /** 
                Adds the option of MyAnimeList's Pictures to be given to the final result
                */ 
                fn add_pictures(&mut self) -> &mut Self {
                    self.url.push_str("pictures,");
                    self
                }
                /** 
                Adds the option of the show's Background to be given to the final result
                */ 
                fn add_background(&mut self) -> &mut Self {
                    self.url.push_str("background,");
                    self
                }
                /** 
                Adds the option of Related Anime to be given to the final result
                */ 
                fn add_related_anime(&mut self) -> &mut Self {
                    self.url.push_str("related_anime,");
                    self
                }
                /** 
                Adds the option of Related Manga to be given to the final result
                */ 
                fn add_related_manga(&mut self) -> &mut Self {
                    self.url.push_str("related_manga,");
                    self
                }
                /** 
                Adds the option of Recommendations to be given to the final result
                */ 
                fn add_recommendations(&mut self) -> &mut Self {
                    self.url.push_str("recommendations,");
                    self
                }
                /** 
                Adds the option of Studio data to be given to the final result
                */ 
                fn add_studios(&mut self) -> &mut Self {
                    self.url.push_str("studios,");
                    self
                }
                /** 
                Adds the option of MyAnimeList Statistics to be given to the final result
                */ 
                fn add_statistics(&mut self) -> &mut Self {
                    self.url.push_str("statistics,");
                    self
                }
            }
        )*
    };
}

impl_filters_for_builders!(Builder, SearchBuilder, SeasonalBuilder);