use std::time::{Duration, SystemTime};

use ratatui::widgets::{ListState, TableState};
use serde::{Deserialize, Serialize};

pub struct AccessToken {
    pub expires_at: SystemTime,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OauthTokens {
    pub access_token: String,
    expires_in: u16,
    scope: String,
    pub refresh_token: String,
    token_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Playlist {
    pub title: String,
    pub tracks_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Playlists {
    pub collection: Vec<Playlist>,
    pub next_href: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Track {
    pub title: String,
    pub duration: u64,
    pub duration_str: Option<String>,
    pub user: User,
    pub user_favorite: bool,
    pub metadata_artist: Option<String>,
    pub urn: String,
    pub genre: Option<String>,
    pub waveform_url: String,
    pub waveform: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct Tracks {
    pub collection: Vec<Track>,
    pub next_href: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct Streams {
    pub hls_mp3_128_url: String,
    // Other formats provided by Soundcloud, not needed atm.
    //pub hls_aac_160_url: String,
    //pub preview_mp3_128_url: String,
}

pub struct States {
    pub tracks: TableState,
    pub playlists: ListState,
    pub library: ListState,
}

#[derive(Debug)]
pub struct HlsPlaylist {
    pub segments: Vec<HlsSegment>,
}

#[derive(Debug)]
pub struct HlsSegment {
    pub url: String,
    pub duration: f32,
}

impl Track {
    pub fn table_row_data(&self) -> Vec<&str> {
        vec![
            &self.title,
            &self.user.username,
            &self.genre.as_deref().unwrap_or(""),
            &self.duration_str.as_deref().unwrap_or(""),
            if self.user_favorite { "❤️" } else { "" },
        ]
    }
}

impl AccessToken {
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
}

impl OauthTokens {
    pub fn expires_at(&self) -> SystemTime {
        SystemTime::now()
            .checked_add(Duration::from_secs(self.expires_in.into()))
            .unwrap()
    }
}

impl States {
    pub fn init() -> Self {
        States {
            tracks: TableState::default().with_selected(Some(0)),
            playlists: ListState::default().with_selected(Some(0)),
            library: ListState::default().with_selected(Some(0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expired_token() {
        let token = AccessToken {
            expires_at: SystemTime::now(),
            token: Default::default(),
        };
        std::thread::sleep(Duration::from_millis(1));
        assert!(token.is_expired())
    }

    #[test]
    fn unexpired_token() {
        let token = AccessToken {
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(10))
                .unwrap(),
            token: Default::default(),
        };
        assert!(!token.is_expired())
    }
}
