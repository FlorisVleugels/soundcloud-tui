use std::sync::{Arc, Mutex};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, Body, Focus, Mode};
use crate::handlers;
use crate::soundcloud::client::Client;

pub async fn handle(app: &mut App, client: &Arc<Mutex<Client>>) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match app.mode {
            Mode::Normal => { 
                match key.code {
                    KeyCode::Char('/') => app.mode = Mode::Editing,
                    KeyCode::Char('j') => app.increase_index(),
                    KeyCode::Char('k') => app.decrease_index(),
                    KeyCode::Char('l') => app.focus = Focus::Library,
                    KeyCode::Char('p') => app.focus = Focus::Playlists,
                    KeyCode::Char(' ') => app.toggle_playback(),
                    KeyCode::Char('+') => app.volume_up(),
                    KeyCode::Char('-') => app.volume_down(),
                    KeyCode::Enter => handlers::enter(app, client).await,
                    KeyCode::Esc => {
                        app.body = Body::Welcome;
                        app.focus = Focus::Playlists;
                    }
                    KeyCode::Char('?') => app.toggle_help(),
                    KeyCode::Char('q') => return Ok(true),
                    _ => {}
                }
            },
            Mode::Editing => {
                if key.kind == KeyEventKind::Press { 
                    match key.code {
                        KeyCode::Enter => {}
                        KeyCode::Char(to_insert) => app.enter_char(to_insert),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Right => app.move_cursor_right(),
                        KeyCode::Esc => app.mode = Mode::Normal,
                        _ => {}
                    }
                }
            }
        };
    }
    Ok(false)
}
