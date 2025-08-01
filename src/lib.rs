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

    let client = match config.is_complete() {
        true => Client::init(config),
        false => {
            let config = soundcloud::auth(&app, terminal, &Arc::new(Mutex::new(config)))?;
            match config {
                Some(config) => Client::init(config),
                None => return Ok(())
            }
        }
    };

    app.lock().unwrap().mode = Mode::Normal;

    loop {
        terminal.draw(|frame| ui::render_app(frame, &mut *app.lock().unwrap()))?;
        if events::handle(&mut *app.lock().unwrap())? {
            break Ok(());
        }
    }
}
