use std::fs;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    text::Text,
    widgets::{Block, Padding, Paragraph},
    Frame};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}


fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    use Constraint::{Length, Min, Ratio};

    let vertical = Layout::vertical([Length(3), Min(0), Length(6)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    
    // top
    let horizontal_title = Layout::horizontal([Constraint::Percentage(92), Constraint::Percentage(8)]);
    let [search_area, help_area] = horizontal_title.areas(title_area);

    // body
    let horizontal_body = Layout::horizontal([Constraint::Percentage(85), Constraint::Percentage(15)]);
    let [body_area, bar_area] = horizontal_body.areas(main_area);
    let vertical_bar = Layout::vertical([Ratio(4,3); 2]);
    let [top_area, bot_area] = vertical_bar.areas(bar_area);

    // banner
    let changelog: String = fs::read_to_string("changelog.md").unwrap();
    let banner = Text::from(changelog);
    let paragraph = Paragraph::new(banner).block(Block::bordered()
            .title("Welcome!")
            .padding(Padding::new(5,0,2,0))
            );

    frame.render_widget(paragraph, body_area);
    frame.render_widget(Block::bordered().title("Search"), search_area);
    frame.render_widget(Block::bordered().title("Help"), help_area);
    frame.render_widget(Block::bordered().title("Playing (Test Song - Test Band)"), status_area);

    frame.render_widget(Block::bordered().title("Library"), top_area);
    frame.render_widget(Block::bordered().title("Playlists"), bot_area);

}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}
