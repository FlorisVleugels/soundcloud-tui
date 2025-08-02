use std::collections::HashMap;

use reqwest::Error;
use serde::Deserialize;

use super::{config::ClientConfig};

const BASE_URL: &str = "https://api.soundcloud.com/";
const TOKEN_URL: &str = "https://secure.soundcloud.com/oauth/token";

#[derive(Deserialize, Debug)]
pub struct OauthTokens {
    pub access_token: String,
    expires_in: u16,
    scope: String,
    pub refresh_token: String,
    token_type: String
}

pub async fn oauth_tokens(
    config: &ClientConfig,
    client: &reqwest::Client,
) -> Result<OauthTokens, Error> {
    let mut params = HashMap::new();
    let verifier = &config.code_verifier.as_ref().unwrap()[..];
    let code = &config.client_code().as_ref().unwrap()[..];
    params.insert("grant_type", "authorization_code");
    params.insert("client_id", &config.client_id[..]);
    params.insert("client_secret", &config.client_secret[..]);
    params.insert("redirect_uri", "http://localhost:3000");
    params.insert("code_verifier", verifier);
    params.insert("code", code);

    let response = client
        .post(TOKEN_URL)
        .header("accept", "application/json; charset=utf-8")
        .form(&params)
        .send()
        .await?
        .json::<OauthTokens>()
        .await?;

    Ok(response)
}

pub async fn refresh(
    refresh_token: &String,
    config: &ClientConfig,
    client: &reqwest::Client,
    ) -> Result<OauthTokens, Error> {
    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("client_id", &config.client_id[..]);
    params.insert("client_secret", &config.client_secret[..]);
    params.insert("refresh_token", &refresh_token[..]);

    let response = client
        .post(TOKEN_URL)
        .header("accept", "application/json; charset=utf-8")
        .form(&params)
        .send()
        .await?
        .json::<OauthTokens>()
        .await?;

    Ok(response)
}

pub async fn _search_playlists() {
    
}
