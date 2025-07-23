mod app;
mod events;
mod handlers;
mod soundcloud;
mod ui;

use std::sync::mpsc;
use tokio::task;

use app::App;
use soundcloud::{
    auth::{self, Message},
    client::Client,
    config::ClientConfig
};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let config = ClientConfig::load();
    let mut app = App::init();

    let mut auth_url = None;
    let mut rx_opt = match config.is_complete() {
        true => {
            let mut client = Client::init(config.clone());
            app.is_authenticated = true;
            None
        },
        false => {
            let (tx, rx) = mpsc::channel();
            task::spawn(
                auth::run(config.clone(), tx)
            );
            Some(rx)
        }
    };

    loop {
        if let Some(rx) = &rx_opt {
            match rx.recv().unwrap() {
                Message::AuthUrl(url) => {
                    auth_url = Some(url)
                },
                Message::Authenticated(_) => {
                    let mut client = Client::init(config.clone());
                    app.is_authenticated = true;
                    auth_url = None;
                    rx_opt = None;
                },
            }
        }

        terminal.draw(|frame| ui::render(frame, &mut app, &auth_url))?;
        if events::handle(&mut app)? {
            break Ok(());
        }
    }
}
