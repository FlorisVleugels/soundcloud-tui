use std::error::Error;
use std::fs::{self, File};
use std::sync::{Arc, Mutex};

use crate::app::App;
use crate::playback::Playback;
use crate::soundcloud::util;

use super::api;
use super::config::ClientConfig;
use super::models::{AccessToken, RefreshToken};
use super::path;

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
        let refresh_token = match File::open(&token_path) {
            Ok(file) => serde_yaml::from_reader(file)?,
            Err(_) => RefreshToken { token: None },
        };

        if let Some(refresh_token) = refresh_token.token {
            let (access_token, refresh_token) =
                Self::refresh_from_file(&refresh_token, &config, &client).await;
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
                .truncate(true)
                .open(token_path)
                .unwrap();
            serde_yaml::to_writer(file, &self.refresh_token).unwrap();
        }
    }

    async fn access_token(&mut self) -> String {
        if self.access_token.is_expired() {
            self.refresh().await;
        }
        self.access_token.token.clone()
    }

    async fn refresh(&mut self) {
        let refresh_string = self.refresh_token.token.as_ref().unwrap();
        let tokens = api::refresh(refresh_string, &self.config, &self.client)
            .await
            .unwrap();
        self.access_token = AccessToken {
            expires_at: tokens.expires_at(),
            token: tokens.access_token,
        };
        self.refresh_token = RefreshToken {
            token: Some(tokens.refresh_token),
        };
    }

    async fn refresh_from_file(
        refresh_token: &str,
        config: &ClientConfig,
        client: &reqwest::Client,
    ) -> (AccessToken, RefreshToken) {
        let tokens = api::refresh(refresh_token, config, client).await.unwrap();
        (
            AccessToken {
                expires_at: tokens.expires_at(),
                token: tokens.access_token,
            },
            RefreshToken {
                token: Some(tokens.refresh_token),
            },
        )
    }

    async fn fetch_tokens(
        config: &ClientConfig,
        client: &reqwest::Client,
    ) -> (AccessToken, RefreshToken) {
        let tokens = api::oauth_tokens(config, client).await.unwrap();
        (
            AccessToken {
                expires_at: tokens.expires_at(),
                token: tokens.access_token,
            },
            RefreshToken {
                token: Some(tokens.refresh_token),
            },
        )
    }

    pub async fn _search_playlists(&self) {
        todo!()
    }

    pub async fn search_tracks(&mut self, app: &mut App) {
        let token = self.access_token().await;
        let response = api::search_tracks(&token, &self.client, &app.input).await;
        if let Ok(mut tracks) = response {
            for track in &mut tracks.collection {
                util::convert_duration(track);
            }
            app.tracks = Some(tracks)
        }
    }

    pub async fn _search_users(&self) {
        todo!()
    }

    pub async fn liked_playlists(&mut self, app: &Arc<Mutex<App>>) {
        let token = self.access_token().await;
        let response = api::liked_playlists(&token, &self.client).await;
        if let Ok(playlists) = response {
            app.lock().unwrap().liked_playlists = Some(playlists)
        }
    }

    pub async fn liked_tracks(&mut self, app: &mut App) {
        let token = self.access_token().await;
        let response = api::liked_tracks(&token, &self.client).await;
        if let Ok(mut tracks) = response {
            for track in &mut tracks.collection {
                util::convert_duration(track);
            }
            app.tracks = Some(tracks)
        }
    }

    pub async fn playlist_tracks(&mut self, app: &mut App) {
        let token = self.access_token().await;
        let tracks_url = &app
            .liked_playlists
            .as_ref()
            .unwrap()
            .collection
            .get(app.playlists_index)
            .unwrap()
            .tracks_uri;
        let response = api::playlist_tracks(&token, &self.client, tracks_url).await;
        if let Ok(mut tracks) = response {
            for track in &mut tracks.collection {
                util::convert_duration(track);
            }
            app.tracks = Some(tracks)
        }
    }

    pub async fn streams(&mut self, app: &mut App) {
        let token = self.access_token().await;
        if let Some(current_track) = &app.current_track {
            let track_urn = &current_track.urn;
            let response = api::streams(&token, &self.client, track_urn).await;
            if let Ok(streams) = response {
                app.playback = Some(Playback::init(streams));
            }
        }
    }

    pub async fn _waveform(&self, app: &mut App) {
        if let Some(track) = &mut app.current_track {
            let response = api::_waveform(&self.client, &track.waveform_url).await;
            if let Ok(waveform) = response {
                track.waveform = Some(waveform);
            }
        }
    }
}
