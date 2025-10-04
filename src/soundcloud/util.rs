use std::time::Duration;

use crate::soundcloud::models::Track;

pub fn parse_duration(track: &mut Track) {
    let seconds = Duration::from_millis(track.duration.into())
        .as_secs();
    track.duration = seconds;
    track.duration_str = Some(format!("{}:{:02}", (seconds / 60), (seconds % 60)));
}
