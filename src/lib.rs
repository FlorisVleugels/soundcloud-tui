mod app;
mod events;
mod handlers;
mod soundcloud;
mod ui;

use std::{
    error::Error, 
    sync::{Arc, Mutex}
};

use app::{App, Mode};
use soundcloud::{
    client::Client,
    config::ClientConfig
};

pub async fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::load()?;
    let app = Arc::new(Mutex::new(App::init()));

    let cancel_token = match config.is_complete() {
        true => {
            let mut client = Client::init(config);
            app.lock().unwrap().mode = Mode::Normal;
            None
        },
        false => soundcloud::auth(&app, config.clone())
    };

    loop {
        terminal.draw(|frame| ui::render(frame, &mut *app.lock().unwrap()))?;
        if events::handle(&mut *app.lock().unwrap())? {
            if let Some(token) = cancel_token {
                token.cancel();
            }
            break Ok(());
        }
    }
}
