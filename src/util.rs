pub fn format_duration(duration: u64) -> String {
    format!("{}:{:02}", (duration / 60), (duration % 60))
}
