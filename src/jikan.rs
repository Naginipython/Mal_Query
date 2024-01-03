use std::error::Error;
use serde_json::Value;

pub async fn get_anime(id: u32) -> Result<Value, Box<dyn Error>> {
    let base_url = format!("https://api.jikan.moe/v4/anime/{id}/full");

    let client = reqwest::Client::new();
    let res = client.get(&base_url)
        .send()
        .await?;

    if res.status().is_success() {
        let data: Value = res.json().await?;
        return Ok(data);
    } else {
        return Err(format!("Request failed with status {:?}", res.status()))?;
    }
}