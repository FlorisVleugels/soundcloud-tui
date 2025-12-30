use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{Rng, distr::Alphanumeric};
use ring::digest::{SHA256, digest};

pub struct Pkce {
    pub verifier: String,
    pub challenge: String,
}

impl Pkce {
    pub fn new() -> Self {
        let verifier = Self::code_verifier();
        let challenge = Self::code_challenge(&verifier);

        Pkce {
            verifier,
            challenge,
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
