use std::{ 
    fs, path::Path
};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use dirs;

pub struct AppConfig {
}

#[derive(Serialize, Deserialize, Debug)]
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
        let config = Self::get_client_config().unwrap();

        let config: Self = serde_yaml::from_value(config).unwrap();

        config
    }

    fn get_client_config() -> Result<Value, serde_yaml::Error>{
        let client_config_path = match dirs::home_dir() {
            Some(path) => {
                Path::new(&path)
                    .join(CONFIG_DIR)
                    .join(APP_CONFIG_DIR)
                    .join(CLIENT_CONFIG_FILE)
            },
            None => panic!("Unable to get home directory."),
        };

        // Handle error for reading config file if not there create it
        let file = fs::File::open(client_config_path).unwrap();
        let d = serde_yaml::Deserializer::from_reader(file);

        Value::deserialize(d)
    }

    fn set_client_code(&mut self) {
    }

    fn create_client_config() {
    }
}
