use std::fs::{self, File};
use std::io::ErrorKind;
use serde::{Serialize, Deserialize};

use super::path;

pub struct AppConfig {
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    //pub redirect_uri: String,
    pub client_code: Option<String>,
    pub code_verifier: Option<String>,
}

const AUTH_URL: &str = "https://secure.soundcloud.com/authorize";

impl ClientConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let client_path = path("client.yml")?;
        match File::open(&client_path) {
            Ok(file) => {
                let config = serde_yaml::from_reader(file)?;
                Ok(config)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let config = ClientConfig {
                    client_id: "YOUR_CLIENT_ID".to_string(),
                    client_secret: "YOUR_CLIENT_SECRET".to_string(),
                    client_code: None,
                    code_verifier: None,
                };

                let file = File::create(&client_path)?;
                serde_yaml::to_writer(file, &config)?;

                Err("client.yml created — please update it with your credentials".into())
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn set_client_code(&mut self, code: String) {
        self.client_code = Some(code);
        self.save();
    }

    pub fn client_code(&self) -> Option<&String> {
        self.client_code.as_ref()
    }

    fn create_client_config() {
    }

    fn save(&self) {
        // add else for the event that Err() from path()
        if let Ok(client_path) = path("client.yml") {
            let file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(client_path)
                .unwrap();
            serde_yaml::to_writer(file, &self).unwrap();
        }
    }

    pub fn is_complete(&self) -> bool {
        match (&self.client_code, &self.code_verifier) {
            (Some(_), Some(_)) => true,
            _ => false
        }
    }

    pub fn auth_url(&self, code_challenge: &String) -> String {
        let code_verifier = match &self.code_verifier {
            Some(code) => code,
            None => panic!()
        };

        format!("{}\
            ?client_id={}\
            &redirect_uri={}\
            &response_type=code\
            &code_challenge={}\
            &code_challenge_method=S256\
            &state={}",
            AUTH_URL, 
            self.client_id, 
            "http://localhost:3000", 
            code_challenge, 
            code_verifier
        )
    }
}
