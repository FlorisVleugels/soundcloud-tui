use std::fs;
use super::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Padding},
    Frame,
};

const MAIN_CONSTRAINTS: [Constraint; 3] = [Constraint::Length(3), Constraint::Min(0), Constraint::Length(6)];
const TOP_BAR_CONSTRAINTS: [Constraint; 2] = [Constraint::Percentage(95), Constraint::Percentage(5)];
const BODY_CONSTRAINTS: [Constraint; 2] = [Constraint::Percentage(85), Constraint::Percentage(15)];
const BODY_BAR_CONSTRAINTS: [Constraint; 2] = [Constraint::Ratio(1,2); 2];

pub fn render(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::vertical(MAIN_CONSTRAINTS);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());

    draw_top_bar(frame, app, title_area);
    draw_body(frame, main_area);
    draw_status_bar(frame, status_area);
}

fn draw_top_bar(frame: &mut Frame, app: &mut App, rect: Rect) {
    let horizontal_title = Layout::horizontal(TOP_BAR_CONSTRAINTS);
    let [search_area, help_area] = horizontal_title.areas(rect);

    let paragraph = Paragraph::new("Type ?")
        .block(Block::bordered().title("Help"));

    draw_search_box(frame, app, search_area);
    frame.render_widget(paragraph, help_area);
}

fn draw_status_bar(frame: &mut Frame, rect: Rect) {
    frame.render_widget(Block::bordered().title("Playing (Test Song - Test Band)"), rect);
}

fn draw_body(frame: &mut Frame, rect: Rect) {
    let horizontal_body = Layout::horizontal(BODY_CONSTRAINTS);
    let [body_area, bar_area] = horizontal_body.areas(rect);

    let vertical_bar = Layout::vertical(BODY_BAR_CONSTRAINTS);
    let [top_area, bot_area] = vertical_bar.areas(bar_area);

    let changelog: String = fs::read_to_string("CHANGELOG.md").unwrap();
    let banner = Text::from(changelog);
    let paragraph = Paragraph::new(banner)
        .block(Block::bordered()
            .title("Welcome!")
            .padding(Padding::new(5,0,2,0))
        );

    frame.render_widget(paragraph, body_area);
    frame.render_widget(Block::bordered().title("Library"), top_area);
    frame.render_widget(Block::bordered().title("Playlists"), bot_area);
}

fn draw_search_box(frame: &mut Frame, app: &App, rect: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
    .block(Block::bordered().title("Search"));

    frame.render_widget(input, rect);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor_position(Position::new(
                rect.x + app.character_index as u16 + 1,
                rect.y + 1,
        )),
    }
}
