use std::error::Error;
use url::Url;
use super::{models::*, run_search, run_get};

/**
Takes a name and a limiter, and searches uses the MyAnimeList API to create a `MalAnimeSearch`, which holds a vector of anime Name/Ids<br>
Async function, function must be called with `.await`<br>
Returns a `Result<>`, with a success containing a `MalAnimeData`
*/
pub async fn search_anime(name: &str, limit: u32) -> Result<MalAnimeSearch, Box<dyn Error>> {
    let base_url = format!("https://api.myanimelist.net/v2/anime?q={name}&limit={limit}");
    run_search(&base_url).await
}

/**
Takes a year and season, and gets all the anime of that in a `MalAnimeSearch`, which holds a vector of anime Name/Ids<br>
Async function, function must be called with `.await`<br>
Returns a `Result<>`, with a success containing a `MalAnimeData`
*/
pub async fn get_season(year: u32, season: Season) -> Result<MalAnimeSearch, Box<dyn Error>> {
    let s: &str;
    match season {
        Season::Winter => s = "winter",
        Season::Spring => s = "spring",
        Season::Summer => s = "summer",
        Season::Fall => s = "fall"
    }
    let base_url = format!("https://api.myanimelist.net/v2/anime/season/{year}/{s}?limit=500");
    run_search(&base_url).await
}



/**
Takes an anime ID, and gets the full result of the data from the MyAnimeList API<br>
Async function, function must be called with `.await`<br>
Returns a `Result<>`, with a success containing a `MalAnimeDataDetailed`
*/
pub async fn get_anime(id: u32) -> Result<MalAnimeData, Box<dyn Error>> {
    let base_url = format!(
        "https://api.myanimelist.net/v2/anime/{id}?fields=id,title,main_picture,alternative_titles,start_date,end_date,synopsis,mean,rank,
        popularity,num_list_users,num_scoring_users,nsfw,created_at,updated_at,media_type,status,genres,my_list_status,num_episodes,
        start_season,broadcast,source,average_episode_duration,rating,pictures,background,related_anime,related_manga,recommendations,studios,statistics");
    run_get(&base_url).await
}

/**
Takes a MyAnimeList URL to an anime page, parses the URL to get the anime ID, and calls get_anime(id)<br>
Async function, function must be called with `.await`<br>
Returns a `Result<>`, with a success containing a `MalAnimeData`
*/
pub async fn get_anime_from_url(url: &str) -> Result<MalAnimeData, Box<dyn Error>> {
    let parsed = Url::parse(&url)?;
    for segment in parsed.path_segments().ok_or("URL has no path")? {
        if let Ok(id) = segment.parse::<u32>() {
            let result = get_anime(id).await?;
            return Ok(result);
        }
    }
    Err("URL sent contains no anime ID path")?
}

/**
Takes a MyAnimeList ranking-type, and a limit, and retrieves the top `limit` # of anime, of the ranking type<br>
Async function, function must be called with `.await`<br>
Returns a `Result<>`, with a success containing a `MalAnimeData`
*/
pub async fn get_anime_rankings(ranking_type: RankingType, limit: u32) -> Result<MalAnimeSearch, Box<dyn Error>> {
    let r_type: &str;
    match ranking_type {
        RankingType::All => r_type = "all",
        RankingType::Airing => r_type = "airing",
        RankingType::Upcoming => r_type = "upcoming",
        RankingType::TV => r_type = "tv",
        RankingType::OVA => r_type = "ova",
        RankingType::Movie => r_type = "movie",
        RankingType::Special => r_type = "special",
        RankingType::ByPopularity => r_type = "bypopularity",
        RankingType::Favorite => r_type = "favorite",
        RankingType::None => r_type = ""
    }
    let base_url = format!("https://api.myanimelist.net/v2/anime/ranking?ranking_type={r_type}&limit={limit}");
    run_search(&base_url).await
}

/**
Takes a usernamename and a limiter, and searches uses the MyAnimeList API to create a Vector of `MalAnimeData`, which holds anime Name/Ids<br>
Async function, function must be called with `.await`<br>
Returns a `Result<>`, with a success containing a `MalAnimeData`
*/
pub async fn get_user_animelist(username: &str, limit: u32) -> Result<MalAnimeSearch, Box<dyn Error>> {
    let base_url = format!("https://api.myanimelist.net/v2/users/{username}/animelist?fields=list_status{{is_rewatching,num_times_rewatched,rewatch_value,priority,tags,comments,start_date,end_date}}&limit={limit}");
    run_search(&base_url).await
}