mod app;
mod events;
mod handlers;
mod soundcloud;
mod ui;

use std::{
    error::Error, 
    sync::{Arc, Mutex}
};
use tokio::task;

use app::{App, Mode};
use soundcloud::{
    auth::{self},
    client::Client,
    config::ClientConfig
};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::load()?;
    let app = Arc::new(Mutex::new(App::init()));

    match config.is_complete() {
        true => {
            let mut client = Client::init(config);
            app.lock().unwrap().mode = Mode::Normal;
        },
        false => {
            task::spawn(
                auth::run(config.clone(), Arc::clone(&app))
            );
        }
    };

    loop {
        terminal.draw(|frame| ui::render(frame, &mut *app.lock().unwrap()))?;
        if events::handle(&mut *app.lock().unwrap())? {
            break Ok(());
        }
    }
}
