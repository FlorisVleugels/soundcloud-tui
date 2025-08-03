use std::error::Error;
use std::fs::{self, File};
use std::sync::{Arc, Mutex};
use serde_yaml::Value;
use serde::{Serialize, Deserialize};

use crate::app::App;

use super::api::{self, PlaylistResponse};
use super::config::ClientConfig;
use super::path;

pub struct AccessToken(pub String);

#[derive(Serialize, Deserialize)]
pub struct RefreshToken{
    token: Option<String>
}

pub struct Client {
    config: ClientConfig,
    pub access_token: AccessToken,
    refresh_token: RefreshToken,
    client: reqwest::Client,
}

impl Client {
    pub async fn init(config: ClientConfig) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let token_path = path("refresh.yml")?;
        let token_file = File::open(token_path)?;
        let d = serde_yaml::Deserializer::from_reader(token_file);
        let token = Value::deserialize(d).unwrap();
        let value: RefreshToken = serde_yaml::from_value(token).unwrap();

        if let Some(refresh_token) = value.token {
            let (access_token, refresh_token) = Self::refresh_from_file(&refresh_token, &config, &client).await;
            Ok(Client {
                config,
                access_token,
                refresh_token,
                client,
            })
        } else {
            let (access_token, refresh_token) = Self::fetch_tokens(&config, &client).await;
            Ok(Client {
                config,
                access_token,
                refresh_token,
                client,
            })
        }
    }
    
    pub fn store_refresh_token(&self) {
        if let Ok(token_path) = path("refresh.yml") {
            let file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(token_path)
                .unwrap();
            serde_yaml::to_writer(file, &self.refresh_token).unwrap();
        }
    }

    fn access_token(&self) -> &AccessToken {
        // base64 decode jwt, check if expired. If yes call refresh, then return token
        &self.access_token
    }

    async fn refresh(&mut self) {
        let refresh_string = self.refresh_token.token.as_ref().unwrap();
        let tokens = api::refresh(&refresh_string,
            &self.config, &self.client).await.unwrap();
        self.refresh_token = RefreshToken{ token: Some(tokens.refresh_token) };
        self.access_token = AccessToken(tokens.access_token);
    }

    async fn refresh_from_file(
        refresh_token: &String,
        config: &ClientConfig,
        client: &reqwest::Client
    ) -> (AccessToken, RefreshToken) {
        let tokens = api::refresh(refresh_token, config, client).await.unwrap();
        (
            AccessToken(tokens.access_token), 
            RefreshToken { token: Some(tokens.refresh_token) },
        )
    }

    async fn fetch_tokens(
        config: &ClientConfig,
        client: &reqwest::Client
    ) -> (AccessToken, RefreshToken) {
        let tokens = api::oauth_tokens(config, client).await.unwrap();
        (
            AccessToken(tokens.access_token), 
            RefreshToken { token: Some(tokens.refresh_token) },
        )
    }

    pub async fn _search_playlists(&self) {
        
    }

    pub async fn _search_tracks(&self) {
        
    }

    pub async fn _search_users(&self) {

    }

    pub async fn liked_playlists(&self, app: &Arc<Mutex<App>>) {
        let response = api::liked_playlists(&self.access_token.0, &self.client).await;
        if let Ok(playlists) = response {
            app.lock().unwrap().playlists = Some(playlists)
        }
    }

    pub async fn _liked_tracks(&self) {
        
    }
}
