mod redirect;
mod pkce;

use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration
};

use pkce::PKCE;
use tokio::task;
use tokio_util::sync::CancellationToken;
use crossterm::event::{self, poll, Event, KeyCode};

use crate::ui;
use super::config::ClientConfig;


pub fn auth(
    terminal: &mut ratatui::DefaultTerminal,
    config: &Arc<Mutex<ClientConfig>>
) -> Result<Option<ClientConfig>, Box<dyn Error>>{
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    let cloned_config = Arc::clone(config);

    task::spawn(async move {
        tokio::select! {
            _ = cloned_token.cancelled() => {},
            _ = run(cloned_config) => {}
        }
    });

    while !config.lock().unwrap().is_complete() {
        terminal.draw(|frame| ui::auth(frame))?;
        if handle()? {
            token.cancel();
            return Ok(None)
        }
    }
    return Ok(Some(config.lock().unwrap().clone()))
}

async fn run(config: Arc<Mutex<ClientConfig>>) -> Result<(), Box<dyn Error>>{
    let keys = PKCE::new();
    config.lock().unwrap().code_verifier = Some(keys.verifier);
    open::that(config.lock().unwrap().auth_url(&keys.challenge)).unwrap();
    redirect::serve(&config).await?;
    Ok(())
}


pub fn handle() -> std::io::Result<bool> {
    if poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(true)
            }
        }
    }
    Ok(false)
}
