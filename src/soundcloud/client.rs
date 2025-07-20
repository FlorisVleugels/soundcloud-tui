use std::sync::mpsc::Sender;

use super::config::ClientConfig;
use super::api;
use super::auth::{redirect, pkce::PKCE};

pub struct AccessToken(pub String);
pub struct RefreshToken(pub String);

pub struct Client {
    config: ClientConfig,
    access_token: AccessToken,
    refresh_token: RefreshToken,
    client: reqwest::Client,
}

impl Client {
    pub fn init(tx: Sender<String>) -> Self {
        let mut config = ClientConfig::load();
        let client = reqwest::Client::new();

        match (&config.client_code, &config.code_verifier) {
            (Some(_), Some(_)) => (), 
            _ => {
                let keys = PKCE::new();
                config.code_verifier = Some(keys.verifier);
                let auth_url = config.auth_url(&keys.challenge);
                tx.send(auth_url).unwrap();

                redirect::serve(&mut config);
            }
        };

        //let (access_token, refresh_token) = Self::fetch_tokens(&config, &client).await;
        let (access_token, refresh_token) = (AccessToken(String::from("asdf")), RefreshToken(String::from("asdf")));
        
        Client {
            config,
            access_token,
            refresh_token,
            client,
        }
    }
    
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    fn refresh(&mut self) {
        
    }

    async fn fetch_tokens(
        config: &ClientConfig,
        client: &reqwest::Client
    ) -> (AccessToken, RefreshToken) {
        api::oauth_token(&config, &client).await.unwrap()
    }

    fn search_playlists(&self) {
        
    }

    fn search_tracks(&self) {
        
    }

    fn search_users(&self) {

    }
}
