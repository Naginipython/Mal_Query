use std::{error::Error, fs::File, io::Write};
use reqwest::Body;
use serde_json::Value;
use url::Url;
use tiny_http::Server;
use super::{CLIENT_ID, TOKEN};

// TODO: describe
pub async fn login() -> Result<(), Box<dyn Error>> {
    // Creates and sends the url
    let code_verify = pkce::code_verifier(50);
    let code_challenge = pkce::code_challenge(&code_verify);
    let auth_url: String = format!("https://myanimelist.net/v1/oauth2/authorize?response_type=code&client_id={}&code_challenge={code_challenge}", CLIENT_ID.clone());
    // open::that(&auth_url)?; //opens link automatically

    // Opens a localhost server
    let server = Server::http("127.0.0.1:8080").expect("Failed to create server");
    println!("Please visit: {}", auth_url);

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