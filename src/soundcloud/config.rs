use std::error::Error;
use std::path::PathBuf;
use std::{ 
    fs::{self, File},
    path::Path
};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use dirs;

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

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "soundcloud-tui";
const CLIENT_CONFIG_FILE: &str = "client.yml";

const AUTH_URL: &str = "https://secure.soundcloud.com/authorize";

impl ClientConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::path()?;

        let config_file = File::open(config_path)?;

        let d = serde_yaml::Deserializer::from_reader(config_file);

        let config = Value::deserialize(d).unwrap();

        let value: Self = serde_yaml::from_value(config).unwrap();

        Ok(value)
    }

    fn path() -> Result<PathBuf, &'static str> {
        match dirs::home_dir() {
            Some(path) => {
                Ok(Path::new(&path)
                    .join(CONFIG_DIR)
                    .join(APP_CONFIG_DIR)
                    .join(CLIENT_CONFIG_FILE)
                )
            },
            None => Err("Unable to get home directory."),
        }
    }

    pub fn set_client_code(&mut self, code: String) {
        self.client_code = Some(code);
        self.save();
    }

    pub fn client_code(&mut self) -> &Option<String> {
        &self.client_code
    }

    fn create_client_config() {
    }

    fn save(&self) {
        // add else for the event that Err() from path()
        if let Ok(config_path) = Self::path() {
            let file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(config_path)
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
