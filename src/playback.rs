use rodio::{OutputStreamBuilder, Sink, Source};
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
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        let mut stream = reqwest::get(&self.streams.http_mp3_128_url[..]).await?.bytes_stream();

        let (tx, mut rx) = mpsc::channel(100);
        let source = AudioSource { rx, channels: 2, sample_rate: 44100 };
        sink.append(source);

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

                    let track_id = track.id;

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

                        if packet.track_id() != track_id {
                            continue;
                        }

                        match decoder.decode(&packet) {
                            Ok(decoded) => {
                                match decoded {
                                    AudioBufferRef::F32(buf) => {
                                        for &sample in buf.chan(0) {
                                            let _ = tx.send(sample).await;
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
                                panic!("{}", err);
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
    }

    pub fn resume(&self) {

    }
}
