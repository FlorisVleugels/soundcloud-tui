use std::time::Duration;

use crate::{
    soundcloud::models::Track, 
    util::format_duration
};

pub fn convert_duration(track: &mut Track) {
    let seconds = Duration::from_millis(track.duration)
        .as_secs();
    track.duration = seconds;
    track.duration_str = Some(format_duration(track.duration));
}
