fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = soundcloud_tui::run(&mut terminal);
    ratatui::restore();
    result
}
