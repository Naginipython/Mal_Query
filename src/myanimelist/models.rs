use serde::{Deserialize, Serialize};

// -------- enums --------

#[derive(Debug, Deserialize, PartialEq)]
pub enum RankingType {
    All,
    Airing,
    Upcoming,
    TV,
    OVA,
    Movie,
    Special,
    ByPopularity,
    Favorite,
    None,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Season {
    #[serde(rename = "winter")]
    Winter,
    #[serde(rename = "spring")]
    Spring,
    #[serde(rename = "summer")]
    Summer,
    #[serde(rename = "fall")]
    Fall
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Nsfw {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "black")]
    Black,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum MediaType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "tv")]
    TV,
    #[serde(rename = "ova")]
    OVA,
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "ona")]
    ONA,
    #[serde(rename = "music")]
    Music,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum AiringStatus {
    #[serde(rename = "finished_airing")]
    FinishedAiring,
    #[serde(rename = "currently_airing")]
    CurrentlyAiring,
    #[serde(rename = "not_yet_aired")]
    NotYetAired,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Status {
    #[serde(rename = "watching")]
    Watching,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "on_hold")]
    OnHold,
    #[serde(rename = "dropped")]
    Dropped,
    #[serde(rename = "plan_to_watch")]
    PlanToWatch,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Source {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "manga")]
    Manga,
    #[serde(rename = "4_koma_manga")]
    FourKomaManga,
    #[serde(rename = "web_manga")]
    WebManga,
    #[serde(rename = "digital_manga")]
    DigitalManga,
    #[serde(rename = "novel")]
    Novel,
    #[serde(rename = "light_novel")]
    LightNovel,
    #[serde(rename = "visual_novel")]
    VisualNovel,
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "card_game")]
    CardGame,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "picture_book")]
    PictureBook,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "music")]
    Music,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Rating {
    #[serde(rename = "g")]
    G,
    #[serde(rename = "pg")]
    PG,
    #[serde(rename = "pg_13")]
    PG13,
    #[serde(rename = "r")]
    R,
    #[serde(rename = "r+")]
    RPlus,
    #[serde(rename = "rx")]
    RX,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Sort {
    ListScore,
    ListUpdatedAt,
    AnimeTitle,
    AnimeStartDate,
    AnimeId
}

// -------- MalAnimeDataDetailed --------

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct MalAnimeData {
    pub id: u32,
    pub title: String,
    pub main_picture: Picture,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub synopsis: Option<String>,
    pub mean: Option<f32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub num_list_users: Option<u32>,
    pub num_scoring_users: Option<u32>,
    pub nsfw: Option<Nsfw>,
    pub genres: Option<Vec<Genres>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub media_type: Option<MediaType>,
    pub status: Option<AiringStatus>,
    pub list_status: Option<ListStatus>,
    pub num_episodes: Option<u32>,
    pub start_season: Option<StartSeason>,
    pub broadcast: Option<Broadcast>,
    pub source: Option<Source>,
    pub average_episode_duration: Option<u32>,
    pub rating: Option<Rating>,
    pub studios: Option<Vec<Studios>>,
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
    pub related_anime: Option<Vec<Related>>,
    pub related_manga: Option<Vec<Related>>, // TODO: Add Manga Related
    pub recommendations: Option<Vec<Recommended>>,
    pub statistics: Option<Statistics>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct Picture {
    pub large: String,
    pub medium: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AlternativeTitles {
    pub synonyms: Vec<String>,
    pub en: String,
    pub ja: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Genres {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ListStatus {
    pub status: Status,
    pub score: u32,
    pub num_episodes_watched: u32,
    pub is_rewatching: bool,
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub priority: Option<u32>,
    pub num_times_rewatched: Option<u32>,
    pub rewatch_value: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub comments: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct StartSeason {
    pub year: u32,
    pub season: Season,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Broadcast {
    pub day_of_the_week: String, // enum?
    pub start_time: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Studios {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Related {
    pub node: MalAnimeData,
    pub relation_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Recommended {
    pub node: MalAnimeData,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Statistics {
    pub num_list_users: u32,
    pub status: StatisticsStatus,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct StatisticsStatus {
    pub watching: String,
    pub completed: String,
    pub on_hold: String,
    pub dropped: String,
    pub plan_to_watch: String,
}

// -------- Functions --------

#[derive(Debug)]
pub struct MalAnimeSearch {
    pub data: Vec<MalAnimeData>,
}

impl MalAnimeSearch {
    pub fn new(data: Vec<MalAnimeData>) -> Self {
        MalAnimeSearch {
            data
        }
    }
    pub fn to_titles(self) -> Vec<String> {
        self.data.into_iter().map(|x| x.title).collect()
    }
    pub fn get<'a>(&'a self, index: usize) -> Option<&'a MalAnimeData> {
        self.data.get(index)
    }
}