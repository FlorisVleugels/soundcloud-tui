use crate::soundcloud::models::Streams;

pub struct Playback {
    pub streams: Option<Streams>,
    pub status: Option<Status>,
    pub volume: Option<u32>,
    pub position: Option<u32>,
}

pub enum Status {
    Playing,
    Paused,
    Unavailable,
}

impl Playback {
    pub fn start(&self) {
        
    }

    pub fn stop(&self) {
        
    }
    
    pub fn resume(&self) {
        
    }
}
