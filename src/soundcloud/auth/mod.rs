mod redirect;
mod pkce;

use std::{
    error::Error,
    sync::{Arc, Mutex}
};

use pkce::PKCE;
use tokio::task;
use tokio_util::sync::CancellationToken;

use crate::{app::App, events, ui};
use super::config::ClientConfig;


pub fn auth(
    app: &Arc<Mutex<App>>,
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
        if events::handle(&mut *app.lock().unwrap())? {
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
