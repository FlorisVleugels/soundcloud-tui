use std::error::Error;
use rodio::{OutputStreamBuilder, Sink};

use crate::soundcloud::models::Streams;

pub struct Playback {
    pub streams: Streams,
    pub status: Option<Status>,
    pub position: Option<u32>,
    pub sink: Option<Sink>
}

pub enum Status {
    Playing,
    Paused,
    Unavailable,
}

impl Playback {
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // make get request and stream the bytes
        // mp3 decode the stream
        // append the stream to the rodio sink
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let response = reqwest::get(&self.streams.http_mp3_128_url[..]).await?;
        let sink = rodio::play(&stream_handle.mixer(), response.bytes_stream()).unwrap();
        Ok(())
    }

    pub fn toggle(&self) {
    }
    
    pub fn resume(&self) {
        
    }
}
