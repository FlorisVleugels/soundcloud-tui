mod app;
mod events;
mod handlers;
mod soundcloud;
mod ui;

use std::{
    error::Error, 
    sync::{mpsc::{self, Receiver}, Arc, Mutex}
};
use tokio::task;

use app::{App, Mode};
use soundcloud::{
    auth::{self, Message},
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
            let (tx, rx) = mpsc::channel();
            task::spawn(
                auth::run(config.clone(), tx)
            );
            task::spawn( 
                rcv(rx, Arc::clone(&app), config)
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

async fn rcv(rx: Receiver<Message>, app: Arc<Mutex<App>>, config: ClientConfig) {
    // Dont need to loop this if only keep success message, since recv blocks anyway
    loop {
        match rx.recv().unwrap() {
            Message::Authenticating => {},
            Message::Success => {
                let mut client = Client::init(config);
                app.lock().unwrap().mode = Mode::Normal;
                break
            }
        }
    }
}
