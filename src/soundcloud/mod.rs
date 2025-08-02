mod auth;
pub mod api;
pub mod client;
pub mod config;

pub use auth::auth;

use std::path::{Path, PathBuf};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "soundcloud-tui";

fn path(file: &str) -> Result<PathBuf, &'static str> {
    match dirs::home_dir() {
        Some(path) => {
            Ok(Path::new(&path)
                .join(CONFIG_DIR)
                .join(APP_CONFIG_DIR)
                .join(file)
            )
        },
        None => Err("Unable to get home directory."),
    }
}
