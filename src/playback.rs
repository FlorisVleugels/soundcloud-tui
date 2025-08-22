use std::error::Error;
use rodio::{OutputStreamBuilder, Sink, Source};
use symphonia::core::io::MediaSourceStream;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

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

struct AudioSource {
    rx: tokio::sync::mpsc::Receiver<f32>,
    channels: u16,
    sample_rate: u32,
}

impl Iterator for AudioSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.rx.blocking_recv()
    }
}

impl Source for AudioSource {
    fn channels(&self) -> rodio::ChannelCount {
        self.channels 
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.sample_rate
    }

    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}


impl Playback {
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        let mut stream = reqwest::get(&self.streams.http_mp3_128_url[..]).await?.bytes_stream();

        let (tx, mut rx) = mpsc::channel(100);
        let source = AudioSource { rx, channels: 2, sample_rate: 100 }
        sink.append(source);

        tokio::spawn(async move {
            while let Some(chunk) = stream.next().await {
                MediaSourceStream::new(Box::new(chunk), options);
                let _ = tx.send(chunk).await;
            }
        });

        Ok(())
    }

    pub fn toggle(&self) {
    }
    
    pub fn resume(&self) {
        
    }
}
