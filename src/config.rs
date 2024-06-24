use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub nologin_path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Nologin {
    pub text: String,
    pub logout_timeout: u64,
}

impl Default for Nologin {
    fn default() -> Self {
        Self {
            text: "This account can not be logged into.".to_string(),
            logout_timeout: 5,
        }
    }
}

pub fn read() -> Config {
    let config_str = include_str!("../../config.toml");
    toml::from_str::<Config>(config_str).expect("Failed to parse userspace config.")
}
