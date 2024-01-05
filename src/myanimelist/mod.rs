use std::{sync::Mutex, fs, error::Error, fs::File, io::Write};
use lazy_static::lazy_static;
use reqwest::Response;
use serde_json::Value;
use self::models::{MalAnimeData, MalAnimeSearch, ListStatus};
use reqwest::Body;
use url::Url;
use tiny_http::Server;

pub mod retrieval;
pub mod builders;
pub mod models;
pub mod user;

lazy_static! {
    // CLIENT_ID recieve a string from a local file not uploaded here, only containing a MyAnimeList Client Id
    pub static ref CLIENT_ID: String = fs::read_to_string("secret.txt").unwrap_or(String::new());
    pub static ref TOKEN: Mutex<String> = Mutex::new(fs::read_to_string("token.txt").unwrap_or(String::new()));
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

        return Ok(data);
    } else {
        return Err(format!("Request failed with status {:?}", res.status()))?;
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

        return Ok(MalAnimeSearch::new(result));
    } else {
        return Err(format!("Request failed with status {:?}", res.status()))?;
    }
}

/// This function will create a User's authentication token to use more MyAnimeList features, such as updating your lists entries, or viewing your list statistics.<br>
/// Calling this function will send the user to a browser to accept the OAuth2.0 authentication, and than use the callback to generate a user's token.<br>
/// This function returns a Result, for the programmer to handle `Error` cases.<br>
/// ### Example usage:
/// ```
/// use mal_query::myanimelist::login::login;
/// #[tokio::main]
/// async fn login_example() {
///     match login().await {
///         Ok(()) => assert!(true),
///         Err(_e) => assert!(false),
///     }
/// }
/// ```
pub async fn login() -> Result<(), Box<dyn Error>> {
    // Creates and sends the url
    let code_verify = pkce::code_verifier(50);
    let code_challenge = pkce::code_challenge(&code_verify);
    let auth_url: String = format!("https://myanimelist.net/v1/oauth2/authorize?response_type=code&client_id={}&code_challenge={code_challenge}", CLIENT_ID.clone());
    
    // Opens a localhost server
    let server = Server::http("127.0.0.1:8080").expect("Failed to create server");
    open::that(&auth_url)?; //opens link automatically

    // Wait for the callback with the authorization code
    let mut code = String::new();
    for request in server.incoming_requests() {
        // request should look like `localhost:8080/auth?code=...`
        let url = Url::parse(&format!("http://localhost{}", request.url()))?;
        if let Some(val) = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value) {
                code = val.to_string();
                break;
            }
    }

    // Requests the access_token
    let client = reqwest::Client::new();
    let res = client.post("https://myanimelist.net/v1/oauth2/token")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(Body::from(format!(
            "client_id={}&code={}&code_verifier={}&grant_type=authorization_code",
            CLIENT_ID.clone(),
            code,
            code_challenge
        )))
        .send()
        .await?;
    let json: Value = res.json().await?;

    // Saves token to a file and globally
    match json["access_token"].as_str() {
        Some(access_token) => {
            let mut token = TOKEN.lock().unwrap();
            *token = access_token.to_string();

            // Saves code to file
            let mut file = File::create("token.txt")?;
            file.write_all(&*token.as_bytes())?;
            Ok(())
        },
        None => return Err(format!("{json:?}"))?,
    }
}