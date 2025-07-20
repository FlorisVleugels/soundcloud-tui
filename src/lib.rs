mod app;
mod events;
mod handlers;
mod soundcloud;
mod ui;

use app::App;
use soundcloud::client::Client;
use std::{sync::mpsc, thread};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let (tx , rx) = mpsc::channel();

    let handle = thread::spawn(|| {
        Client::init(tx);
    });

    let mut app = App::init();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app, &rx))?;
        if events::handle(&mut app)? {
            break Ok(());
        }
    }
}
