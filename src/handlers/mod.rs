use std::sync::{Arc, Mutex};

use crate::app::{App, Focus, Mode};
use crate::soundcloud::client::Client;

pub async fn enter(app: &mut App, client: &Arc<Mutex<Client>>) {
    match app.focus {
        Focus::Playlists => open_playlist(app, client).await,
        Focus::Library => {
            match app.states.library.selected().unwrap() {
                0 => open_recents(app),
                1 => open_liked_tracks(app, client).await,
                _ => unimplemented!(), // Potentially other library items (Artists / Albums etc),
            }
        }
        Focus::Body => play_track(app, client).await,
        Focus::Status => unimplemented!(), //Open cava like plot in main panel
    }
}

pub async fn toggle_like(app: &mut App, client: &Arc<Mutex<Client>>) {
    if let Some(tracks) = &mut app.tracks {
        let i = app.states.tracks.selected().unwrap();
        let track = tracks.collection.get_mut(i).unwrap();
        match track.user_favorite {
            true => {
                client.lock().unwrap().unlike_track(&track.urn[..]).await;
                track.user_favorite = false;
            },
            false => {
                client.lock().unwrap().like_track(&track.urn[..]).await;
                track.user_favorite = true;
            }
        }
    }

}

pub async fn search(app: &mut App, client: &Arc<Mutex<Client>>) {
    client.lock().unwrap().search_tracks(app).await;
    app.body_title = app.input.clone();
    app.mode = Mode::Normal;
    app.show_tracks();
}

async fn open_playlist(app: &mut App, client: &Arc<Mutex<Client>>) {
    client.lock().unwrap().playlist_tracks(app).await;
    app.body_title = app.set_title().to_string();
    app.show_tracks();
}

fn open_recents(app: &mut App) {
    app.set_recents();
    app.body_title = "Recently Played".to_string();
    app.show_tracks();
}

async fn open_liked_tracks(app: &mut App, client: &Arc<Mutex<Client>>) {
    client.lock().unwrap().liked_tracks(app).await;
    app.body_title = "Liked Tracks".to_string();
    app.show_tracks();
}

async fn play_track(app: &mut App, client: &Arc<Mutex<Client>>) {
    app.set_track();
    client.lock().unwrap().streams(app).await;
    if let Err(_) = app.play_track().await {
        todo!()
    }
}
