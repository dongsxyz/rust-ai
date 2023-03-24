//!
//! # Configuration
//!
//! Provide configuration related types and functions/methods.
//!
//! Note: `config.yml` must locate in your current working directory.

////////////////////////////////////////////////////////////////////////////////

use std::{fs::read_to_string, path::PathBuf};

use log::error;
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

    /// Azure config mapping
    pub azure: Azure,
}

impl Config {
    /// Load contents from local config file.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = PathBuf::from("config.yml");
        if !config_path.exists() {
            return Err("`config.yml` doesn't exist!".into());
        }

        return if let Ok(config_contents) = read_to_string(config_path) {
            match serde_yaml::from_str(&config_contents) {
                Ok(config) => Ok(config),
                Err(e) => {
                    error!(target: "global", "Unable to parse config: {:?}", e);
                    Err(e.into())
                }
            }
        } else {
            Err("Unable to read `config.yml`".into())
        };
    }
}

/// A mapping for OpenAI configuration contents
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAi {
    /// API key obtained from <https://openai.com>.
    pub api_key: String,
}

/// A mapping for Azure (Global) configuration contents
#[derive(Debug, Serialize, Deserialize)]
pub struct Azure {
    /// Configuration content for cognitive/speech.
    pub speech: AzureSpeech,
}

/// Service key for use in multiple Azure services.
#[derive(Debug, Serialize, Deserialize)]
pub struct AzureSpeech {
    /// Key content from <https://portal.azure.com/>
    pub key: String,

    /// Region name from <https://portal.azure.com/>
    pub region: String,
}
