use std::time::Duration;

use super::app::{App, Mode};
use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};

pub fn handle(app: &mut App) -> std::io::Result<bool> {
    match app.mode {
        Mode::Authenticating => {
            if poll(Duration::from_millis(1000))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(true)
                    }
                }
            }
            Ok(false)
        },
        Mode::Normal => { 
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('/') => {
                        app.mode = Mode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(true);
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
                        KeyCode::Enter => {},
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
