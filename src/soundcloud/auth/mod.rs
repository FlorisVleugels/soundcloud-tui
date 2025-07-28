mod redirect;
mod pkce;

use std::sync::{Arc, Mutex};

use pkce::PKCE;
use crate::app::App;
use super::config::ClientConfig;


pub async fn run(mut config: ClientConfig, app: Arc<Mutex<App>>) {
    let keys = PKCE::new();
    config.code_verifier = Some(keys.verifier);

    open::that(config.auth_url(&keys.challenge)).unwrap();

    redirect::serve(&mut config, &app);
}
