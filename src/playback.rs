use crate::soundcloud::models::Streams;
use std::process::{Child, Command, Stdio};

pub struct Playback {
    pub streams: Streams,
    pub status: Option<Status>,
    pub volume: Option<u32>,
    pub position: Option<u32>,
    pub cmd: Option<Child>
}

pub enum Status {
    Playing,
    Paused,
    Unavailable,
}

impl Playback {
    pub fn start(&mut self) -> Result<(), std::io::Error> {
        let cmd = Command::new("ffplay")
            .arg("-vn")
            .arg("-nodisp")
            .arg(&self.streams.hls_mp3_128_url[..])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        self.cmd = Some(cmd);
        Ok(())
    }

    pub fn stop(&self) {
        
    }
    
    pub fn resume(&self) {
        
    }

    pub fn proc(&self) {
        
    }
}
