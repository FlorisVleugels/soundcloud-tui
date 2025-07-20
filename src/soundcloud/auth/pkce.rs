use ring::digest::{digest, SHA256};
use rand::{
    Rng,
    distr::Alphanumeric
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

pub struct PKCE {
    pub verifier: String,
    pub challenge: String,
}

impl PKCE {
    pub fn new() -> Self {
        let verifier = Self::code_verifier();
        let challenge = Self::code_challenge(&verifier);

        PKCE {
            verifier, 
            challenge
        }
    }

    fn code_verifier() -> String {
        let mut rng = rand::rng();
        (0..56).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    fn code_challenge(verifier: &str) -> String {
        let digest = digest(&SHA256, verifier.as_bytes());
        URL_SAFE_NO_PAD.encode(digest)
    }
}
