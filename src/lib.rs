mod app;
mod auth;
mod config;
mod ui;
mod handlers;
mod events;

use app::App;
use std::thread;
use config::ClientConfig;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    ClientConfig::init();

    thread::spawn(|| {
        auth::serve();
    });

    let mut app = App::init();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;
        if events::handle(&mut app)? {
            break Ok(());
        }
    }
}
