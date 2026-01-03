use std::error::Error;

use crate::{
    playback::Playback,
    soundcloud::models::{Playlists, TableStates, Track, Tracks},
    ui::constants::LIBRARY_ITEMS,
};

pub struct App {
    pub input: String,
    pub mode: Mode,
    pub focus: Focus,
    pub body: Body,
    pub playback: Option<Playback>,
    pub volume: f32,
    pub shuffle: bool,
    pub repeat: bool,
    pub show_help: bool,
    pub liked_playlists: Option<Playlists>,
    pub tracks: Option<Tracks>,
    recents: Option<Tracks>,
    pub table_states: TableStates,
    pub current_track: Option<Track>,
    pub search_index: usize,
    pub playlists_index: usize,
    pub library_index: usize,
    pub body_index: usize,
    pub body_title: String,
}

pub enum Mode {
    Normal,
    Editing,
}

pub enum Focus {
    Body,
    Library,
    Playlists,
    Status,
}

pub enum Body {
    Welcome,
    Tracks,
    Waveform,
}

impl App {
    pub fn init() -> Self {
        let table_states = TableStates::init();
        Self {
            input: String::new(),
            mode: Mode::Normal,
            focus: Focus::Playlists,
            body: Body::Welcome,
            playback: None,
            volume: 1.00,
            shuffle: false,
            repeat: false,
            show_help: false,
            liked_playlists: None,
            tracks: None,
            recents: None,
            table_states,
            current_track: None,
            search_index: 0,
            playlists_index: 0,
            library_index: 0,
            body_index: 0,
            body_title: String::new(),
        }
    }

    pub fn increase_index(&mut self) {
        match self.focus {
            Focus::Body => {
                self.table_states.tracks.select_next();
            }
            Focus::Library => {
                if self.library_index < LIBRARY_ITEMS.len() - 1 {
                    self.library_index += 1
                } else {
                    self.library_index = 0;
                }
            }
            Focus::Playlists => {
                if self.playlists_index
                    < self
                        .liked_playlists
                        .as_ref()
                        .unwrap()
                        .collection
                        .iter()
                        .len()
                        - 1
                {
                    self.playlists_index += 1
                } else {
                    self.playlists_index = 0;
                }
            }
            _ => {}
        }
    }

    pub fn decrease_index(&mut self) {
        match self.focus {
            Focus::Body => {
                self.table_states.tracks.select_previous();
            }
            Focus::Library => {
                if self.library_index == 0 {
                    self.library_index = LIBRARY_ITEMS.len() - 1
                } else {
                    self.library_index -= 1
                }
            }
            Focus::Playlists => {
                if self.playlists_index == 0 {
                    self.playlists_index = self
                        .liked_playlists
                        .as_ref()
                        .unwrap()
                        .collection
                        .iter()
                        .len()
                        - 1
                } else {
                    self.playlists_index -= 1
                }
            }
            _ => {}
        }
    }

    fn _increase(_i: &mut usize, _length: &usize) {
        todo!()
    }

    pub fn set_title(&self) -> &str {
        &self
            .liked_playlists
            .as_ref()
            .unwrap()
            .collection
            .get(self.playlists_index)
            .unwrap()
            .title
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help
    }

    pub fn show_tracks(&mut self) {
        self.body = Body::Tracks;
        self.focus = Focus::Body;
    }

    pub fn set_track(&mut self) {
        if let Some(tracks) = &self.tracks {
            let track = tracks.collection.get(self.body_index).unwrap().clone();
            self.update_recents(track.clone());
            self.current_track = Some(track);
        }
        if let Some(playback) = &mut self.playback {
            playback.cancel();
        }
    }

    fn update_recents(&mut self, track: Track) {
        match &mut self.recents {
            Some(tracks) => {
                tracks.collection.push(track);
            }
            None => {
                self.recents = Some(Tracks {
                    collection: vec![track],
                    next_href: None,
                })
            }
        }
    }

    pub async fn play_track(&mut self) -> Result<(), Box<dyn Error>> {
        self.focus = Focus::Status;
        self.playback.as_mut().unwrap().start().await?;
        Ok(())
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.search_index.saturating_sub(1);
        self.search_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.search_index.saturating_add(1);
        self.search_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.search_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.search_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.search_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn _reset_index() {
        todo!()
    }

    pub fn set_recents(&mut self) {
        self.tracks = self.recents.as_ref().map(|tracks| Tracks {
            collection: tracks.collection.clone(),
            next_href: None,
        });
    }

    pub fn toggle_playback(&mut self) {
        if let Some(playback) = self.playback.as_mut() {
            playback.toggle();
        };
    }

    pub fn volume_up(&mut self) {
        if let Some(playback) = self.playback.as_mut() {
            playback.increase(&mut self.volume);
        };
    }

    pub fn volume_down(&mut self) {
        if let Some(playback) = self.playback.as_mut() {
            playback.decrease(&mut self.volume);
        };
    }
}
