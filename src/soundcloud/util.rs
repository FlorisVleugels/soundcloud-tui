use std::time::Duration;

use reqwest::Response;

use crate::{
    soundcloud::models::{HlsPlaylist, HlsSegment, Track},
    util::format_duration,
};

pub fn convert_duration(track: &mut Track) {
    let seconds = Duration::from_millis(track.duration).as_secs();
    track.duration = seconds;
    track.duration_str = Some(format_duration(track.duration));
}

fn parse_hls_seg_dur(dur: &str) -> f32 {
    let duration = dur
        .split_once(":")
        .unwrap()
        .1
        .split_once(",")
        .unwrap()
        .0
        .parse::<f32>();
    duration.unwrap()
}

pub async fn parse_m3u8(response: Response) -> Result<HlsPlaylist, anyhow::Error> {
    let mut hls_playlist = HlsPlaylist {
        segments: Vec::new(),
    };
    let text = response.text().await?;
    let mut lines = text.lines().skip(5); // Skipping first 5 header lines

    while let (Some(dur), Some(url)) = (lines.next(), lines.next()) {
        let duration = parse_hls_seg_dur(dur);
        hls_playlist.segments.push(HlsSegment {
            duration,
            url: String::from(url),
        });
    }
    Ok(hls_playlist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hls_duration() {
        let dur = "#EXTINF:4.989302,";
        assert_eq!(parse_hls_seg_dur(dur), 4.989302)
    }
}
