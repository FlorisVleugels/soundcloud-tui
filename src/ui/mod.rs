use std::fs;

use ratatui::{
    layout::{Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{App, Body, Focus, Mode};
use constants::*;

mod constants;

pub fn auth(frame: &mut Frame) {
    let paragraph = Paragraph::new(format!(
            "{}\n\n\n\n\nTo continue, please check the tab that opened in your \
            browser and authorize soundcloud-tui",
            HEADER_ASCII))
        .centered()
        .wrap(Wrap { trim: false })
        .block(Block::bordered()
            .title("soundcloud-tui")
            .padding(Padding::new(0, 0, 6, 0))
        );
    frame.render_widget(paragraph, frame.area());
}

pub fn render_app(
    frame: &mut Frame,
    app: &mut App
) {
    let vertical = Layout::vertical(MAIN_CONSTRAINTS);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());

    draw_top_bar(frame, app, title_area);
    draw_body(frame, main_area, app);
    draw_status_bar(frame, status_area);
}

fn draw_top_bar(frame: &mut Frame, app: &mut App, rect: Rect) {
    let horizontal_title = Layout::horizontal(TOP_BAR_CONSTRAINTS);
    let [search_area, help_area] = horizontal_title.areas(rect);

    let paragraph = Paragraph::new("Type ?")
        .block(Block::bordered().title("Help"));

    draw_search(frame, app, search_area);
    frame.render_widget(paragraph, help_area);
}

fn draw_status_bar(frame: &mut Frame, rect: Rect) {
    frame.render_widget(Block::bordered().title("Playing (Test Song - Test Band)"), rect);
}

fn draw_body(
    frame: &mut Frame,
    rect: Rect,
    app: &mut App,
) {
    let horizontal_body = Layout::horizontal(BODY_CONSTRAINTS);
    let [body_area, bar_area] = horizontal_body.areas(rect);

    let vertical_bar = Layout::vertical(BODY_BAR_CONSTRAINTS);
    let [top_area, bot_area] = vertical_bar.areas(bar_area);

    draw_main_panel(frame, body_area, app);
    draw_library(frame, top_area, app);
    draw_playlists(frame, bot_area, app);
}

fn draw_main_panel(
    frame: &mut Frame,
    rect: Rect,
    app: &mut App,
) {
    match app.body {
        Body::Welcome => draw_welcome(frame, rect),
        Body::Tracks => draw_tracks(frame, rect, app),
    }
}

fn draw_welcome(
    frame: &mut Frame,
    rect: Rect,
) {
    let changelog: String = fs::read_to_string("CHANGELOG.md").unwrap();
    let changelog = Text::from(changelog);
    let paragraph = Paragraph::new(format!(
            "{}\nPlease report any bugs or missing features \
            to https://github.com/FlorisVleugels/soundcloud-tui \
            \n\n\n{}", HEADER_ASCII, changelog
            ))
        .wrap(Wrap { trim: false })
        .block(Block::bordered()
            .title("Welcome!")
            .padding(Padding::new(5,5,2,2))
        );

    frame.render_widget(paragraph, rect);
}

fn draw_playlists(
    frame: &mut Frame,
    rect: Rect,
    app: &mut App
) {
    if let Some(playlists) = &app.liked_playlists {
        let mut titles = vec![];
        for (i, playlist) in playlists.collection.iter().enumerate() {
            match app.focus {
                Focus::Playlists => {
                    if &i == &app.playlists_index {
                        titles.push(Line::from(&playlist.title[..])
                            .style(Color::Yellow));
                            } else {
                                titles.push(Line::from(&playlist.title[..]));
                    }
                }
                _ => {
                    titles.push(Line::from(&playlist.title[..]));
                }
            }
        }
        let paragraph = Paragraph::new(titles)
            .block(Block::bordered()
                .title("Playlists")
            );
        frame.render_widget(paragraph, rect);
    } else {
        frame.render_widget(Block::bordered().title("Playlists"), rect);
    }
}

fn draw_tracks(
    frame: &mut Frame,
    rect: Rect,
    app: &mut App
) {
    if let Some(tracks) = &app.tracks {
        let header = Row::new(vec!["Title", "Artists", "Genre", "Duration"])
            .style(Color::Yellow).bottom_margin(1);
        let mut rows = vec![header];
        for (i, track) in tracks.collection.iter().enumerate() {
            match app.focus {
                Focus::Body => {
                    if &i == &app.body_index {
                        rows.push(Row::new(track.table_row_data())
                            .style(Color::Yellow));
                    } else {
                        rows.push(Row::new(track.table_row_data()));
                    }
                }
                _ => {
                    rows.push(Row::new(track.table_row_data()));
                }
            }
        }
        let table = Table::new(rows, TABLE_COLUMN_WIDTHS)
            .block(Block::bordered()
                .title("Tracks")
            );
        frame.render_widget(table, rect);
    } else {
        frame.render_widget(Block::bordered().title("Tracks"), rect);
    }
}

fn draw_library(
    frame: &mut Frame,
    rect: Rect,
    app: &mut App
) {
    let items = vec![
        Line::from("Recently Played"),
        Line::from("Liked Tracks"),
        Line::from("Artists"),
        Line::from("Albums"),
    ];
    let paragraph = Paragraph::new(items)
        .block(Block::bordered()
            .title("Library")
        );
    frame.render_widget(paragraph, rect);
}

fn draw_search(frame: &mut Frame, app: &mut App, rect: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(match app.mode {
            Mode::Editing => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
    .block(Block::bordered().title("Search"));

    frame.render_widget(input, rect);

    if let Mode::Editing = app.mode {
        frame.set_cursor_position(Position::new(
                rect.x + app.search_index as u16 + 1,
                rect.y + 1,
        ))
    }
    match app.mode {
        Mode::Editing => frame.set_cursor_position(Position::new(
                rect.x + app.search_index as u16 + 1,
                rect.y + 1,
        )),
        _ => {}
    }
}
