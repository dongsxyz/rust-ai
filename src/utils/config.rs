//! 
//! # Configuration
//! 
//! Provide configuration related types and functions/methods.
//! 
//! Note: `config.yml` must locate in your current working directory.

////////////////////////////////////////////////////////////////////////////////

use std::{fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml;

/// Configurations from `config.yml`
///
/// Example contents:
/// ```yaml
/// openai:
///   api_key: sk-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
/// ```
///
/// # Examples
///
/// ```rust
/// use rust_ai::utils::config::Config;
///
/// let config = Config::load();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// OpenAI config mappping
    pub openai: OpenAi,
}

impl Config {
    /// Load contents from local config file.
    pub fn load() -> Self {
        let config_path = PathBuf::from("config.yml");
        if !config_path.exists() {
            panic!("`config.yml` doesn't exist!");
        }

        let config_contents = read_to_string(config_path).unwrap();
        serde_yaml::from_str(&config_contents).unwrap()
    }
}

/// A mapping for OpenAI configuration contents
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAi {
    /// API key obtained from <https://openai.com>.
    pub api_key: String,
}
