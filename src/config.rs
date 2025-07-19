use std::path::PathBuf;
use std::{ 
    fs, path::Path
};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use dirs;
use super::auth;
use std::thread;

pub struct AppConfig {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientConfig {
    client_id: String,
    client_secret: String,
    client_code: Option<String>,
}

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "soundcloud-tui";
const CLIENT_CONFIG_FILE: &str = "client.yml";

impl ClientConfig {
    pub fn init() -> Self {
        // if get client config give error, make new, if fs: error
        let config_path = Self::path();

        // Handle error for reading config file if not there create it
        let file = fs::File::open(config_path).unwrap();
        let d = serde_yaml::Deserializer::from_reader(file);

        let config = Value::deserialize(d).unwrap();

        let mut value: Self = serde_yaml::from_value(config).unwrap();

        if let Some(_) = value.client_code {
            value
        } else {
            let handle = thread::spawn(move || {
                auth::serve(&mut value);
                value
            });
            handle.join().unwrap()
        }
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
}
