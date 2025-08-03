use std::error::Error;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let result = soundcloud_tui::run(&mut terminal).await;
    ratatui::restore();
    result
}
