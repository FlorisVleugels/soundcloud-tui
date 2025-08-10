use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OauthTokens {
    pub access_token: String,
    expires_in: u16,
    scope: String,
    pub refresh_token: String,
    token_type: String
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
    pub duration: u32,
    pub duration_str: Option<String>,
    pub user: User,
    pub metadata_artist: Option<String>,
    pub stream_url: String,
    pub genre: String,
    pub waveform_url: String,
    pub waveform: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct Tracks {
    pub collection: Vec<Track>,
    pub next_href: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
}

impl Track {
    pub fn table_row_data(&self) -> Vec<&str> {
        vec![
            &self.title[..],
            &self.user.username[..],
            &self.genre[..],
            &self.duration_str.as_ref().unwrap()[..],
        ]
    }
}
