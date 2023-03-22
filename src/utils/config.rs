use std::{fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub openai: OpenAi,
}

impl Config {
    pub fn load() -> Self {
        let config_path = PathBuf::from("config.yml");
        if !config_path.exists() {
            panic!("`config.yml` doesn't exist!");
        }

        let config_contents = read_to_string(config_path).unwrap();
        serde_yaml::from_str(&config_contents).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAi {
    pub api_key: String,
}
