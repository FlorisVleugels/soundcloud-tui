pub fn format_duration(duration: u64) -> String {
    if duration < 3600 {
        format!("{}:{:02}", (duration / 60), (duration % 60))
    } else {
        format!(
            "{}:{}:{:02}",
            (duration / 3600),
            (duration % 3600 / 60),
            (duration % 60)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_hour_duration() {
        let result = format_duration(800);
        assert_eq!("13:20", result)
    }

    #[test]
    fn super_hour_duration() {
        let result = format_duration(8000);
        assert_eq!("2:13:20", result)
    }
}
