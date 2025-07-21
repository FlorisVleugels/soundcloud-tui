mod app;
mod events;
mod handlers;
mod soundcloud;
mod ui;

use std::sync::mpsc;

use app::App;
use soundcloud::client::{Client, Message};
use tokio::task;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {

    let (tx, rx) = mpsc::channel();

    let join = task::spawn({
        Client::init(tx)
    });

    let mut app = App::init();
    loop {
        let received = rx.recv().unwrap();

        let auth_url = match received {
            Message::AuthUrl(auth_url) => Some(auth_url),
            Message::Authenticated(_) => {
                app.is_authenticated = true;
                None
            },
        };

        terminal.draw(|frame| ui::render(frame, &mut app, &auth_url))?;
        if events::handle(&mut app)? {
            break Ok(());
        }
    }
}
