use std::{collections::HashMap, u8};

use reqwest::Error;

use super::models::*;
use super::config::ClientConfig;

const BASE_URL: &str = "https://api.soundcloud.com/";
const TOKEN_URL: &str = "https://secure.soundcloud.com/oauth/token";

pub async fn oauth_tokens(
    config: &ClientConfig,
    client: &reqwest::Client,
) -> Result<OauthTokens, Error> {
    let mut params = HashMap::new();
    let verifier = &config.code_verifier.as_ref().unwrap()[..];
    let code = &config.client_code().unwrap()[..];
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

pub async fn _search_tracks() {
    
}

pub async fn liked_playlists(
    access_token: &String,
    client: &reqwest::Client
) -> Result<Playlists, Error> {
    let limit = "limit=20";
    let url = format!("{}me/likes/playlists?{}&linked_partitioning=true", BASE_URL, limit);

    let response = client
        .get(url)
        .header("accept", "application/json; charset=utf-8")
        .header("Authorization", format!("OAuth {}", access_token))
        .send()
        .await?
        .json::<Playlists>()
        .await?;

    Ok(response)
}

pub async fn liked_tracks(
    access_token: &String,
    client: &reqwest::Client
) -> Result<Tracks, Error> {
    let limit = "limit=20";
    let access = "access=playable";
    let url = format!("{}me/likes/tracks?{}&{}&linked_partitioning=true", BASE_URL, limit, access);

    let response = client
        .get(url)
        .header("accept", "application/json; charset=utf-8")
        .header("Authorization", format!("OAuth {}", access_token))
        .send()
        .await?
        .json::<Tracks>()
        .await?;

    Ok(response)
}

pub async fn playlist_tracks(
    access_token: &String,
    client: &reqwest::Client,
    tracks_url: &str,
) -> Result<Tracks, Error> {
    let access = "access=playable";
    let url = format!("{}?{}&linked_partitioning=true", tracks_url, access);

    let response = client
        .get(url)
        .header("accept", "application/json; charset=utf-8")
        .header("Authorization", format!("OAuth {}", access_token))
        .send()
        .await?
        .json::<Tracks>()
        .await?;

    Ok(response)
}

pub async fn stream(
    access_token: &String,
    client: &reqwest::Client,
    stream_url: &str,
) -> Result<Tracks, Error> {
    let response = client
        .get(stream_url)
        .header("accept", "application/json; charset=utf-8")
        .header("Authorization", format!("OAuth {}", access_token))
        .send()
        .await?
        .json::<Tracks>()
        .await?;

    Ok(response)
}

pub async fn waveform(
    client: &reqwest::Client,
    waveform_url: &str
) -> Result<Vec<u8>, Error> {
    let response = client
        .get(waveform_url)
        .header("accept", "application/json; charset=utf-8")
        .send()
        .await?
        .json::<Vec<u8>>()
        .await?;

    Ok(response)
}
