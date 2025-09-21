use std::{collections::VecDeque, io::Read};

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
use tokio_util::bytes::Bytes;

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
    rx: tokio::sync::mpsc::Receiver<Vec<f32>>,
    buffer: VecDeque<f32>,
    channels: u16,
    sample_rate: u32,
}

struct StreamReader<S> {
    stream: S,
    buffer: VecDeque<u8>,
}

impl Iterator for AudioSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer.pop_front() {
            Some(sample) => Some(sample),
            None => {
                if let Some(samples) = self.rx.blocking_recv() {
                    for sample in samples {
                        self.buffer.push_back(sample);
                    }
                }
                self.buffer.pop_front()
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

impl<S> Read for StreamReader<S> 
where
    S: tokio_stream::Stream<Item = Result<Bytes, reqwest::Error>> + Unpin
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut total_copied = 0;
        while total_copied < buf.len() {
            if let Some(byte) = self.buffer.pop_front() {
                buf[total_copied] = byte;
                total_copied += 1;
            } else {
                let chunk = tokio::task::spawn_blocking(move || {
                    self.stream.next()
                })
                match chunk {
                }
            }
        }
        Ok(total_copied)
    }
}

impl<S> StreamReader<S> 
where 
    S: tokio_stream::Stream<Item = Result<Bytes, reqwest::Error>>
{
    fn new(stream: S) -> Self {
        Self {
            stream,
            buffer: VecDeque::new(),
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
        }
    }

    pub async fn stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let bytes_stream = reqwest::get(&self.streams.http_mp3_128_url[..]).await?.bytes_stream();
        let stream_reader = StreamReader::new(bytes_stream);
        stream_reader.buffer.clone()

        let (tx, rx) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            let mut hint = Hint::new();
            hint.with_extension("mp3");

            let meta_opts: MetadataOptions = Default::default();
            let fmt_opts = FormatOptions {
                prebuild_seek_index: false,
                seek_index_fill_rate: 20,
                enable_gapless: true,
            };
            let dec_opts: DecoderOptions = Default::default();

            let mut samples: Vec<f32> = Vec::new();
            let src = ReadOnlySource::new(stream_reader);
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
                                        samples.push(sample);
                                    }
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    Err(Error::IoError(_)) => continue,
                    Err(Error::DecodeError(_)) => continue,
                    Err(_) => break,
                }
            }
            let _ = tx.send(samples).await;
        });

        // Rodio audio streaming
        let output_stream = OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&output_stream.mixer());
        let deque: VecDeque<f32> = VecDeque::new();
        let source = AudioSource { rx, buffer: deque, channels: 2, sample_rate: 44100 };
        sink.append(source);
        self.sink = Some(sink);

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
