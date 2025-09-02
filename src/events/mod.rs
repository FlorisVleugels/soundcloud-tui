use std::sync::{Arc, Mutex};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, Body, Focus, Mode};
use crate::soundcloud::client::Client;

pub async fn handle(app: &mut App, client: &Arc<Mutex<Client>>) -> std::io::Result<bool> {
    match app.mode {
        Mode::Normal => { 
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('?') => app.toggle_help(),
                    KeyCode::Char('/') => {
                        app.mode = Mode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('j') => app.increase_index(),
                    KeyCode::Char('k') => app.decrease_index(),
                    KeyCode::Char('l') => app.focus = Focus::Library,
                    KeyCode::Char('p') => app.focus = Focus::Playlists,
                    KeyCode::Enter => {
                        match app.focus {
                            Focus::Playlists => {
                                client.lock().unwrap().playlist_tracks(app).await;
                                app.body = Body::Tracks;
                                app.focus = Focus::Body;
                            }
                            Focus::Library => {
                                client.lock().unwrap().liked_tracks(app).await;
                                app.body = Body::Tracks;
                                app.focus = Focus::Body;
                            }
                            Focus::Body => {
                                app.play_track();
                                client.lock().unwrap().streams(app).await;
                                app.focus = Focus::Status;
                                app.playback.as_mut().unwrap().stream().await;
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Esc => {
                        app.body = Body::Welcome;
                        app.focus = Focus::Playlists;
                    }
                    _ => {}
                }
            }
            Ok(false)
        },
        Mode::Editing => {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press { 
                    match key.code {
                        KeyCode::Enter => {}
                        KeyCode::Char(to_insert) => app.enter_char(to_insert),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Right => app.move_cursor_right(),
                        KeyCode::Esc => app.mode = Mode::Normal,
                        _ => {}
                    }}
            }
            Ok(false)
        }
    }
}
