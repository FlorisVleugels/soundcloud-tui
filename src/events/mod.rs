use std::time::Duration;

use super::app::{App, Mode};
use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};

pub fn handle(app: &mut App) -> std::io::Result<bool> {
    if let Mode::Authenticating = app.mode {
        loop {
            if poll(Duration::from_millis(100))? {
                return key_reader(app)
            } else {
                return Ok(false)
            }
        }
    } else {
        key_reader(app)
    }
}

fn key_reader(app: &mut App) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match app.mode {
            Mode::Authenticating => 
                if let KeyCode::Char('q') = key.code {
                    return Ok(true);
                }
            Mode::Normal => match key.code {
                KeyCode::Char('/') => {
                    app.mode = Mode::Editing;
                }
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                _ => {}
            },
            Mode::Editing if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {},
                KeyCode::Char(to_insert) => app.enter_char(to_insert),
                KeyCode::Backspace => app.delete_char(),
                KeyCode::Left => app.move_cursor_left(),
                KeyCode::Right => app.move_cursor_right(),
                KeyCode::Esc => app.mode = Mode::Normal,
                _ => {}
            },
            Mode::Editing => {}
        }
    }
    Ok(false)
}
