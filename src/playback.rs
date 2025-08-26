use rodio::{OutputStreamBuilder, Sink, Source, StreamError};
use symphonia::core::{
    audio::{AudioBufferRef, Signal}, 
    codecs::{DecoderOptions, CODEC_TYPE_NULL}, 
    errors::Error, 
    formats::FormatOptions, 
    io::{MediaSourceStream, ReadOnlySource}, 
    meta::MetadataOptions, 
    probe::Hint
};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::soundcloud::models::Streams;

pub struct Playback {
    pub streams: Streams,
    pub status: Status,
    pub position: u32,
    pub sink: Option<Sink>
}

pub enum Status {
    Playing,
    Paused,
    Available,
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
    pub fn init(streams: Streams) -> Result<Self, StreamError> {
        Ok(Self {
            streams,
            status: Status::Available,
            position: 0,
            sink: None,
        })
    }

    pub async fn stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        let mut stream = reqwest::get(&self.streams.http_mp3_128_url[..]).await?.bytes_stream();

        let (tx, rx) = mpsc::channel(100);
        let source = AudioSource { rx, channels: 2, sample_rate: 44100 };
        sink.append(source);
        self.sink = Some(sink);

        let handle = tokio::spawn(async move {
            let mut hint = Hint::new();
            hint.with_extension("mp3");

            let meta_opts: MetadataOptions = Default::default();
            let fmt_opts: FormatOptions = Default::default();
            let dec_opts: DecoderOptions = Default::default();

            while let Some(chunk) = stream.next().await {
                if let Ok(bytes) = chunk {
                    let src = ReadOnlySource::new(std::io::Cursor::new(bytes));
                    let mss = MediaSourceStream::new(Box::new(src), Default::default());

                    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)
                        .expect("unsupported format");

                    let mut format = probed.format;
                    let track = format.tracks()
                        .iter()
                        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
                        .expect("no supported audio tracks");

                    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)
                        .expect("unsupported codec");

                    loop {
                        let packet = match format.next_packet() {
                            Ok(packet) => packet,
                            Err(Error::ResetRequired) => {
                                unimplemented!();
                            }
                            Err(err) => {
                                break;
                            }
                        };

                        while !format.metadata().is_latest() {
                            format.metadata().pop();
                        }

                        match decoder.decode(&packet) {
                            Ok(decoded) => {
                                match decoded {
                                    AudioBufferRef::F32(buf) => {
                                        let frames = buf.frames();
                                        for frame in 0..frames {
                                            for channel in 0..2 {
                                                let sample = buf.chan(channel)[frame];
                                                let _ = tx.send(sample).await;
                                            }
                                        }
                                    }
                                    _ => unimplemented!(),
                                }
                            }
                            Err(Error::IoError(_)) => {
                                continue;
                            }
                            Err(Error::DecodeError(_)) => {
                                continue;
                            }
                            Err(err) => {
                                break;
                            }
                        }
                    }
                }
            }
        });

        let _ = handle.await;

        Ok(())
    }

    pub fn toggle(&self) {
        //sink.pause()
        //sink.play()
    }

    pub fn increase() {
        //sink.set_volume()
    }

    pub fn decrease() {
        //sink.set_volume()
    }
}
