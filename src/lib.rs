mod app;
mod auth;
mod config;
mod ui;
mod handlers;
mod events;

use app::App;
use config::ClientConfig;
use std::thread;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let handle = thread::spawn(|| {
        ClientConfig::init()
    });
    //let client_config = ClientConfig::init();
    let mut app = App::init();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;
        if events::handle(&mut app)? {
            break Ok(());
        }
    }
}
