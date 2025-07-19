use super::config::ClientConfig;
use super::api;

pub struct Client {
    config: ClientConfig,
    access_token: String,
    refresh_token: String,
    client: reqwest::Client
}

impl Client {
    pub fn init() -> Self {
        let config = ClientConfig::init();
        let client = reqwest::Client::new();
        let (access_token, refresh_token) = Self::fetch_tokens(&config, &client);
        
        Client {
            config,
            access_token,
            refresh_token,
            client
        }
    }
    
    fn access_token(&self) -> &String {
        &self.access_token
    }

    fn refresh(&mut self) {
        
    }

    fn fetch_tokens(
        config: &ClientConfig,
        client: &reqwest::Client
    ) -> (String, String) {
        api::oauth_token(&config, &client)
    }

    fn search_playlists(&self) {
        
    }

    fn search_tracks(&self) {
        
    }

    fn search_users(&self) {

    }
}
