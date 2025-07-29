mod redirect;
mod pkce;

use std::{
    error::Error,
    sync::{Arc, Mutex}
};

use pkce::PKCE;
use tokio::task;
use tokio_util::sync::CancellationToken;

use crate::app::App;
use super::config::ClientConfig;


pub fn auth(app: &Arc<Mutex<App>>, config: ClientConfig) -> Option<CancellationToken> {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    let cloned_app = Arc::clone(app);
    task::spawn(async move {
        tokio::select! {
            _ = cloned_token.cancelled() => {},
            _ = run(config, cloned_app) => {}
        }
    });
    Some(token)
}

async fn run(mut config: ClientConfig, app: Arc<Mutex<App>>) -> Result<(), Box<dyn Error>>{
    let keys = PKCE::new();
    config.code_verifier = Some(keys.verifier);

    open::that(config.auth_url(&keys.challenge)).unwrap();

    redirect::serve(&mut config, &app).await?;
    Ok(())
}
