use reqwest::Error;
use std::collections::HashMap;

use super::config::ClientConfig;

const BASE_URL: &str = "https://api.soundcloud.com/";
const AUTH_URL: &str = "https://secure.soundcloud.com/oauth/token";

pub async fn oauth_token(
    config: &ClientConfig,
    client: &reqwest::Client
) -> Result<(String, String), Error> {
    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("client_id", &config.client_id[..]);
    params.insert("client_secret", &config.client_secret[..]);
    params.insert("redirect_uri", "http://localhost:3000");
    params.insert("code_verifier", "");
    params.insert("code", "");

    let response = client
        .post(AUTH_URL)
        .header("accept", "application/json; charset=utf-8")
        .form(&params)
        .send()
        .await;

    Ok((String::from(""), String::from("")))
}
