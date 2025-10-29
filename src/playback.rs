use std::{collections::VecDeque, io::Read};
use std::sync::{Arc, Mutex};

use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};
use symphonia::core::{
    audio::{AudioBufferRef, Signal}, 
    codecs::{DecoderOptions, CODEC_TYPE_NULL}, 
    errors::Error, 
    formats::FormatOptions, 
    io::{MediaSourceStream, ReadOnlySource}, 
    meta::MetadataOptions, 
    probe::Hint
};
use tokio::{
    sync::mpsc,
    task::JoinHandle
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::soundcloud::models::Streams;

const MAX_VOLUME: f32 = 1.0;
const VOLUME_INTERVAL: f32 = 0.1;

pub struct Playback {
    streams: Streams,
    pub status: Status,
    position: u32,
    sink: Option<Sink>,
    _output: Option<OutputStream>,
    handle: Option<JoinHandle<()>>,
    token: CancellationToken,
}

pub enum Status {
    Playing,
    Paused,
    Available,
    Unavailable,
}

struct AudioSource {
    rx: tokio::sync::mpsc::Receiver<Vec<f32>>,
    buffer: VecDeque<f32>,
    channels: u16,
    sample_rate: u32,
}

struct StreamBuffer {
    buffer: Arc<Mutex<VecDeque<u8>>>,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Playing => write!(f, "Playing"),
            Status::Paused => write!(f, "Paused"),
            Status::Available => write!(f, "Available"),
            Status::Unavailable => write!(f, "Unavailable"),
        }
    }
}

impl Iterator for AudioSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer.pop_front() {
            Some(sample) => Some(sample),
            None => {
                if let Ok(samples) = self.rx.try_recv() {
                    self.buffer.extend(samples);
                    self.buffer.pop_front()
                } else {
                    None
                }
            }
        }
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

impl Read for StreamBuffer { 
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut total_copied = 0;
        while total_copied < buf.len() {
            if let Some(byte) = self.buffer.lock().unwrap().pop_front() {
                buf[total_copied] = byte;
                total_copied += 1;
            }
        }
        Ok(total_copied)
    }
}

impl StreamBuffer {
    fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }  
}

impl Playback {
    pub fn init(streams: Streams) -> Self {
        Self {
            streams,
            status: Status::Available,
            position: 0,
            sink: None,
            _output: None,
            handle: None,
            token: CancellationToken::new(),
        }
    }

    pub async fn stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stream_buffer = StreamBuffer::new();
        let buffer = Arc::clone(&stream_buffer.buffer);

        let mut bytes_stream = reqwest::get(&self.streams.http_mp3_128_url).await?.bytes_stream();
        let network_token = self.token.clone();
        let network_handle = tokio::spawn(async move {
            tokio::select! {
                _ = network_token.cancelled() => {},
                _ = async { 
                    while let Some(chunk) = bytes_stream.next().await {
                        let bytes = chunk.unwrap();
                        let mut buf = buffer.lock().unwrap();
                        buf.extend(bytes);
                    }
                } => {}
            }
        });

        let (tx, rx) = mpsc::channel(100);
        let decoder_token = self.token.clone();
        let decoder_handle = tokio::spawn(async move {
            tokio::select! {
                _ = decoder_token.cancelled() => {},
                _ = async { 
                    let mut hint = Hint::new();
                    hint.with_extension("mp3");

                    let meta_opts: MetadataOptions = Default::default();
                    let fmt_opts = FormatOptions {
                        prebuild_seek_index: false,
                        seek_index_fill_rate: 20,
                        enable_gapless: true,
                    };
                    let dec_opts: DecoderOptions = Default::default();

                    let src = ReadOnlySource::new(stream_buffer);
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
                                if let AudioBufferRef::F32(buf) = decoded {
                                    let mut samples = Vec::with_capacity(buf.frames() * buf.spec().channels.count());
                                    for frame in 0..buf.frames() {
                                        for channel in 0..buf.spec().channels.count() {
                                            samples.push(buf.chan(channel)[frame]);
                                        }
                                    }
                                    let _ = tx.send(samples).await;
                                }
                            }
                            Err(Error::IoError(_)) => continue,
                            Err(Error::DecodeError(_)) => continue,
                            Err(_) => break,
                        }
                    }
                } => {}
            }
        });

        // Rodio audio streaming
        let mut output_stream = OutputStreamBuilder::open_default_stream()?;
        output_stream.log_on_drop(false);
        let sink = rodio::Sink::connect_new(output_stream.mixer());
        let deque: VecDeque<f32> = VecDeque::new();
        let source = AudioSource { rx, buffer: deque, channels: 2, sample_rate: 44100 };
        sink.append(source);
        self.sink = Some(sink);
        self._output = Some(output_stream);

        let handle = tokio::spawn( async move {
            let _ = tokio::join!(
                network_handle,
                decoder_handle
            );
        });
        self.handle = Some(handle);
        self.status = Status::Playing;

        Ok(())
    }

    pub fn cancel(&mut self) {
        self.token.cancel();
    }

    pub fn toggle(&mut self) {
        match self.status {
            Status::Playing => {
                self.sink.as_ref().unwrap().pause();
                self.status = Status::Paused
            }
            Status::Paused => {
                self.sink.as_ref().unwrap().play();
                self.status = Status::Playing;
            }
            _ => (),
        }
    }

    pub fn increase(&self, volume: &mut f32) {
        if *volume < MAX_VOLUME {
            *volume = *volume + VOLUME_INTERVAL;
            self.sink.as_ref().unwrap().set_volume(*volume);
        }
    }

    pub fn decrease(&self, volume: &mut f32) {
        if *volume > 0.00 {
            *volume = *volume - VOLUME_INTERVAL;
            self.sink.as_ref().unwrap().set_volume(*volume);
        }
    }

    pub fn position(&self) -> u64 {
        self.sink.as_ref().unwrap().get_pos().as_secs()
    }
}
