mod redirect;
mod pkce;

use std::sync::mpsc::Sender;

use pkce::PKCE;
use super::config::ClientConfig;

pub enum Message {
    AuthUrl(String),
    Authenticated(bool)
}

pub async fn run(mut config: ClientConfig, tx: Sender<Message>) {
        let keys = PKCE::new();
        config.code_verifier = Some(keys.verifier);

        let auth_url = config.auth_url(&keys.challenge);
        tx.send(Message::AuthUrl(auth_url)).unwrap();

        redirect::serve(&mut config);
        tx.send(Message::Authenticated(true)).unwrap();
}
