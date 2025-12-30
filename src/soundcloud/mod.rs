pub mod api;
mod auth;
pub mod client;
pub mod config;
pub mod models;
mod util;

pub use auth::auth;

use std::{
    fs,
    path::{Path, PathBuf},
};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "soundcloud-tui";

fn path(file: &str) -> Result<PathBuf, &'static str> {
    match dirs::home_dir() {
        Some(home) => {
            let config_path = Path::new(&home).join(CONFIG_DIR).join(APP_CONFIG_DIR);

            fs::create_dir_all(&config_path)
                .expect("Could not create .config/soundcloud-tui directory");

            Ok(config_path.join(file))
        }
        None => Err("Unable to get home directory."),
    }
}
