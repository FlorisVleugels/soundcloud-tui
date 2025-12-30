mod app;
mod events;
mod handlers;
mod playback;
mod soundcloud;
mod ui;
mod util;

use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};

use app::{App, Mode};
use crossterm::event::poll;
use soundcloud::{client::Client, config::ClientConfig};

pub async fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::load()?;
    let app = Arc::new(Mutex::new(App::init()));

    let client = match config.is_complete() {
        true => Client::init(config),
        false => {
            let config = soundcloud::auth(terminal, &Arc::new(Mutex::new(config)))?;
            match config {
                Some(config) => Client::init(config),
                None => return Ok(()),
            }
        }
    };

    app.lock().unwrap().mode = Mode::Normal;
    let client = Arc::new(Mutex::new(client.await?));
    // temp store it also here in case some error. but should call save
    // after every refresh happens, not here
    client.lock().unwrap().store_refresh_token();
    client.lock().unwrap().liked_playlists(&app).await;

    loop {
        terminal.draw(|frame| ui::render_app(frame, &app.lock().unwrap()))?;
        if poll(Duration::from_secs(1))?
            && events::handle(&mut app.lock().unwrap(), &client).await?
        {
            client.lock().unwrap().store_refresh_token();
            if let Some(playback) = &mut app.lock().unwrap().playback {
                playback.cancel();
            }
            break Ok(());
        }
    }
}
