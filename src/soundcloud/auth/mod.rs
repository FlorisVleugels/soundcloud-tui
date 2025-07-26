mod redirect;
mod pkce;

use std::sync::mpsc::Sender;

use pkce::PKCE;
use super::config::ClientConfig;

pub enum Message {
    Authenticating,
    Success
}

pub async fn run(mut config: ClientConfig, tx: Sender<Message>) {
    let keys = PKCE::new();
    config.code_verifier = Some(keys.verifier);

    tx.send(Message::Authenticating).unwrap();
    open::that(config.auth_url(&keys.challenge)).unwrap();

    redirect::serve(&mut config, &tx);
}
