#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    let result = soundcloud_tui::run(&mut terminal);
    ratatui::restore();
    result
}
