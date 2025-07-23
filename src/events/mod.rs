use super::app::{App, InputMode};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn handle(app: &mut App) -> std::io::Result<bool> {
    match app.is_authenticated {
        false => Ok(false),
        true => {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('/') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {},
                        KeyCode::Char(to_insert) => app.enter_char(to_insert),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Right => app.move_cursor_right(),
                        KeyCode::Esc => app.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
            Ok(false)
        }
    }
}
