use crate::soundcloud::api::{Playlists, Track, Tracks};

pub struct App {
    pub input: String,
    pub mode: Mode,
    pub focus: Focus,
    pub body: Body,
    pub status: Option<Track>,
    pub liked_playlists: Option<Playlists>,
    pub tracks: Option<Tracks>,
    pub search_index: usize,
    pub playlists_index: usize,
    pub library_index: usize,
    pub body_index: usize,
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
}

impl App {
    pub const fn init() -> Self {
        Self {
            input: String::new(),
            mode: Mode::Normal,
            focus: Focus::Playlists,
            body: Body::Welcome,
            status: None,
            liked_playlists: None,
            tracks: None,
            search_index: 0,
            playlists_index: 0,
            library_index: 0,
            body_index: 0,
        }
    }

    pub fn increase_index(&mut self) {
        match self.focus {
            Focus::Body => { 
                if self.body_index < self.tracks.as_ref().unwrap().collection.iter().len() - 1 {
                    self.body_index = self.body_index + 1
                } else {
                    self.body_index = 0;
                }
            }
            Focus::Library => {
                self.library_index = self.library_index.saturating_add(1)
            }
            Focus::Playlists => {
                if self.playlists_index < self.liked_playlists.as_ref().unwrap().collection.iter().len() - 1 {
                    self.playlists_index = self.playlists_index + 1
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
                if self.body_index == 0  {
                    self.body_index = self.tracks.as_ref().unwrap().collection.iter().len() - 1
                } else {
                    self.body_index = self.body_index - 1
                }
            }
            Focus::Library => {
                self.library_index = self.library_index.saturating_sub(1)
            }
            Focus::Playlists => {
                if self.playlists_index == 0  {
                    self.playlists_index = self.liked_playlists.as_ref().unwrap().collection.iter().len() - 1
                } else {
                    self.playlists_index = self.playlists_index - 1
                }
            }
            _ => {}
        }
    }

    fn increase(i: &mut usize, length: &usize) {
        if *i < length - 1 {
        } else {
        }
    }

    pub fn play_track(&mut self) {
        if let Some(tracks) = &self.tracks {
            self.status = Some(tracks.collection.iter().nth(self.body_index).unwrap().clone());
        }
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
}
