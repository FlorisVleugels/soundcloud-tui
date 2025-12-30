use std::sync::{Arc, Mutex};

use crate::app::{App, Body, Focus, Mode};
use crate::soundcloud::client::Client;

pub async fn enter(app: &mut App, client: &Arc<Mutex<Client>>) {
    match app.focus {
        Focus::Playlists => open_playlist(app, client).await,
        Focus::Library => {
            match app.library_index {
                0 => open_recents(app),
                1 => open_liked_tracks(app, client).await,
                _ => unimplemented!(), // Potentially other library items (Artists / Albums etc),
            }
        }
        Focus::Body => play_track(app, client).await,
        Focus::Status => unimplemented!(), //Open cava like plot in main panel
    }
}

pub async fn search(app: &mut App, client: &Arc<Mutex<Client>>) {
    client.lock().unwrap().search_tracks(app).await;
    app.body = Body::Tracks;
    app.focus = Focus::Body;
    app.mode = Mode::Normal;
}

async fn open_playlist(app: &mut App, client: &Arc<Mutex<Client>>) {
    client.lock().unwrap().playlist_tracks(app).await;
    app.body = Body::Tracks;
    app.focus = Focus::Body;
}

fn open_recents(app: &mut App) {
    app.set_recents();
    app.body = Body::Tracks;
    app.focus = Focus::Body;
}

async fn open_liked_tracks(app: &mut App, client: &Arc<Mutex<Client>>) {
    client.lock().unwrap().liked_tracks(app).await;
    app.body = Body::Tracks;
    app.focus = Focus::Body;
}

async fn play_track(app: &mut App, client: &Arc<Mutex<Client>>) {
    app.set_track();
    client.lock().unwrap().streams(app).await;
    if let Err(_) = app.play_track().await {
        todo!()
    }
}
