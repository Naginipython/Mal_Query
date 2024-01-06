use std::{sync::Mutex, fs, error::Error};
use lazy_static::lazy_static;
use reqwest::Response;
use serde_json::Value;
use self::models::{MalAnimeData, MalAnimeSearch, ListStatus};

pub use login::login;

pub mod retrieval;
pub mod login;
pub mod builders;
pub mod models;
pub mod user;

lazy_static! {
    // Couldn't find effective way to hide this
    pub static ref CLIENT_ID: String = String::from("f7e5c56ef3561bb0a290a13d35b02c0b");
    pub static ref TOKEN: Mutex<String> = Mutex::new(fs::read_to_string("token.txt").unwrap_or_default());
}

async fn client_call(url: &str) -> Result<Response, Box<dyn Error>> {
    let token = TOKEN.lock().unwrap();
    let header_key: &str;
    let header_value: String;
    if token.is_empty() {
        header_key = "X-MAL-CLIENT-ID";
        header_value = CLIENT_ID.clone();
    } else {
        header_key = "Authorization";
        header_value = format!("Bearer {}", *token);
    }

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(header_key, header_value)
        .send()
        .await?;
    Ok(res)
}

// To get one anime
async fn run_get(url: &str) -> Result<MalAnimeData, Box<dyn Error>> {
    let res = client_call(url).await?;

    if res.status().is_success() {
        let test = res.text().await?;
        let data: MalAnimeData = serde_json::from_str(&test).unwrap();

        Ok(data)
    } else {
        Err(format!("Request failed with status {:?}", res.status()))?
    }
}

// To get Vec of anime (search)
async fn run_search(url: &str) -> Result<MalAnimeSearch, Box<dyn Error>> {
    let res = client_call(url).await?;

    if res.status().is_success() {
        let data: Value = res.json().await?;
        // Takes the data, and throws it into a Vec of MalAnimeData
        let mut result: Vec<MalAnimeData> = Vec::new();
        data["data"]
            .as_array()
            .expect("Expected an array")
            .iter()
            .for_each(|v| {
                let x = v.get("node").unwrap();
                let mut to_push = serde_json::from_value::<MalAnimeData>(x.clone()).unwrap();
                // get_anime_rankings has slightly different results
                if let Some(r) = v.get("ranking") {
                    to_push.rank = Some(r["rank"].as_u64().unwrap() as u32);
                }
                // get_user_animelist has slightly different results
                if let Some(s) = v.get("list_status") {
                    let status = serde_json::from_value::<ListStatus>(s.clone()).unwrap();
                    to_push.list_status = Some(status);
                }
                result.push(to_push);
            });

        Ok(MalAnimeSearch::new(result))
    } else {
        Err(format!("Request failed with status {:?}", res.status()))?
    }
}

