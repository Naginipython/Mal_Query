use std::{error::Error, collections::HashMap};
use serde::Deserialize;
use super::{models::*, TOKEN};

#[derive(Debug, Deserialize)]
pub struct UpdateAnime {
    id: u32,
    params: HashMap<String, String>,
}
impl UpdateAnime {
    /**
    Takes an anime id, and initializes data for an update
    */
    pub async fn new(id: u32) -> Self {
        UpdateAnime {
            id,
            params: HashMap::new(),
        }
    }
    /**
    Takes an existing MalAnimeData variable, and initializes data for an update
    */
    pub fn from_malanimedata(mal_data: &MalAnimeData) -> Self {
        UpdateAnime {
            id: mal_data.id,
            params: HashMap::new(),
        }
    }
    /**
    // Calls the MyAnimeList API to update the user's anime entry based on the fields the other methods implement.<br>
    // The user must first be logged in, for the app to have received a Token from MyAnimList.<br>
    // Example usage:
    ```
    use mal_query::myanimelist::user::UpdateAnime;
    async fn example() {
        let status = UpdateAnime::new(33)
            .await
            .update_score(10)
            .expect("Score should be between 0-10")
            .update()
            .await
            .unwrap(); // Assuming successful
        assert_eq!(status.score, 10);
    }
    ```
    */
    pub async fn update(&mut self) -> Result<MyListStatus, Box<dyn Error>> {
        let url = format!("https://api.myanimelist.net/v2/anime/{}/my_list_status", self.id);
        let token = TOKEN.lock()?;
        if token.is_empty() { return Err("User is not logged in")? }

        let client = reqwest::Client::new();
        let res = client
            .put(url)
            .header("Authorization", format!("Bearer {}", *token))
            .form(&self.params)
            .send()
            .await?;

        if res.status().is_success() {
            let data = res.text().await?;
            let result: MyListStatus = serde_json::from_str(&data)?;
            return Ok(result);
        } else {
            return Err(format!("Request failed with status {:?}", res.status()))?;
        }
    }
    // TODO: describe
    pub fn update_status(&mut self, new_status: Status) -> &mut Self {
        let s: &str;
        match new_status {
            Status::Completed => s = "completed",
            Status::Dropped => s = "dropped",
            Status::OnHold => s = "on_hold",
            Status::PlanToWatch => s = "plan_to_watch",
            Status::Watching => s = "watching",
        }
        self.params.insert("status".to_string(), s.to_string());
        self
    }
    // TODO: describe
    pub fn update_is_rewatching(&mut self, new_is_rewatching: bool) -> &mut Self {
        self.params.insert("is_rewatching".to_string(), new_is_rewatching.to_string());
        self
    }
    // TODO: describe
    pub fn update_score(&mut self, new_score: u32) -> Result<&mut Self, Box<dyn Error>> {
        if new_score > 10 { return Err("Score has to be 0-10")? }
        self.params.insert("score".to_string(), new_score.to_string());
        Ok(self)
    }
    // TODO: describe
    pub async fn update_num_watched_episodes(&mut self, new_num_watched_episodes: u32) -> &mut Self {
        self.params.insert("num_watched_episodes".to_string(), new_num_watched_episodes.to_string());
        self
    }
    // TODO: describe
    pub async fn update_priority(&mut self, new_priority: u32) -> Result<&mut Self, Box<dyn Error>> {
        if new_priority > 2 { return Err("Priority has to be 0-2")? }
        self.params.insert("priority".to_string(), new_priority.to_string());
        Ok(self)
    }
    // TODO: describe
    pub async fn update_num_times_rewatched(&mut self, new_num_times_rewatched: u32) -> &mut Self {
        self.params.insert("num_times_rewatched".to_string(), new_num_times_rewatched.to_string());
        self
    }
    // TODO: describe
    pub async fn update_rewatch_value(&mut self, new_rewatch_value: u32) -> Result<&mut Self, Box<dyn Error>> {
        if new_rewatch_value > 5 { return Err("rewatch_value has to be 0-5")? }
        self.params.insert("rewatch_value".to_string(), new_rewatch_value.to_string());
        Ok(self)
    }
    // TODO: get_anime() doesn't receive complete MyListStatus page, where tag contains data but MAL API doesn't include it.
    // Update: I don't understand how to get that data from the MyAnimeList API, except when it returns a result of an update
    // pub async fn update_tags(&mut self, new_tags: &str) -> &mut Self {
    //     self.params.insert("tags".to_string(), new_tags.to_string());
    //     self
    // }
    // TODO: get_anime() doesn't receive complete MyListStatus page, where comment contains data but MAL API doesn't include it.
    // pub async fn update_comment(&mut self, new_comment: &str) -> &mut Self {
    //     self.params.insert("comment".to_string(), new_comment.to_string());
    //     self
    // }
}

// TODO: describe
pub async fn delete_anime(id: u32) -> Result<(), Box<dyn Error>> {
    let url = format!("https://api.myanimelist.net/v2/anime/{id}/my_list_status");
    let token = TOKEN.lock()?;
    if token.is_empty() { return Err("User is not logged in")? }

    let client = reqwest::Client::new();
    let res = client
        .delete(url)
        .header("Authorization", format!("Bearer {}", *token))
        .send()
        .await?;

    match res.status().is_success() {
        true => Ok(()),
        false => Err(format!("Request failed with status {:?}", res.status()))?,
    }
}