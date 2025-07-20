use std::path::PathBuf;
use std::{ 
    fs, path::Path
};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use dirs;

pub struct AppConfig {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn load() -> Self {
        // if get client config give error, make new, if fs: error
        let config_path = Self::path();

        // Handle error for reading config file if not there create it
        let file = fs::File::open(config_path).unwrap();
        let d = serde_yaml::Deserializer::from_reader(file);

        let config = Value::deserialize(d).unwrap();

        let value: Self = serde_yaml::from_value(config).unwrap();

        value
    }

    fn path() -> PathBuf {
        match dirs::home_dir() {
            Some(path) => {
                Path::new(&path)
                    .join(CONFIG_DIR)
                    .join(APP_CONFIG_DIR)
                    .join(CLIENT_CONFIG_FILE)
            },
            None => panic!("Unable to get home directory."),
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
        let config_path = Self::path();
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_path)
            .unwrap();
        serde_yaml::to_writer(file, &self).unwrap();
    }

    pub fn auth_url(&self, code_challenge: &String) -> String {
        let code_verifier = match &self.code_verifier {
            Some(code) => code,
            None => panic!()
        };

        format!("{}\
            ?clientid={}\
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
