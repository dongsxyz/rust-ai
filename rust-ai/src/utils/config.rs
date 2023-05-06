//!
//! # Configuration
//!
//! Provide configuration related types and functions/methods.
//!
//! Note: `config.yml` must locate in your current working directory.
//!
//! ## Example
//!
//! ```yaml
//! openai:
//!   api_key: sk-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
//! azure:
//!   speech:
//!     key: 4c7eXXXXXXXXXXXXXXXXXXXXXXX54c32
//!     region: westus
//! ```

////////////////////////////////////////////////////////////////////////////////

use serde_yaml;
use std::{fs::read_to_string, path::PathBuf};

/// Configurations from `config.yml`
///
/// Example contents:
/// ```yaml
/// openai:
///   api_key: sk-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
///   base_endpoint: ""
/// azure:
///   speech:
///     key: 4c7eXXXXXXXXXXXXXXXXXXXXXXX54c32
///     region: westus
/// ```
///
/// # Examples
///
/// ```rust
/// use rust_ai::utils::config::Config;
///
/// let config = Config::load();
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    /// OpenAI config mappping
    pub openai: OpenAi,

    /// Azure config mapping
    pub azure: Azure,
}

impl Config {
    /// Load contents from local config file.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(config_contents) = std::env::var("RUST_AI_CONFIG") {
            return match serde_yaml::from_str(&config_contents) {
                Ok(config) => Ok(config),
                Err(e) => {
                    log::error!(target: "global", "Unable to parse config: {:?}", e);
                    Err(e.into())
                }
            };
        } else {
            let config_path = PathBuf::from("config.yml");
            if !config_path.exists() {
                return Err("`config.yml` doesn't exist!".into());
            }

            return if let Ok(config_contents) = read_to_string(config_path) {
                match serde_yaml::from_str(&config_contents) {
                    Ok(config) => Ok(config),
                    Err(e) => {
                        log::error!(target: "global", "Unable to parse config: {:?}", e);
                        Err(e.into())
                    }
                }
            } else {
                Err("Unable to read `config.yml`".into())
            };
        }
    }
}

/// A mapping for OpenAI configuration contents
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct OpenAi {
    /// API key obtained from <https://openai.com>.
    pub api_key: String,

    /// OpenAI Organization ID
    pub org_id: Option<String>,

    /// Alternative base endpoint for OpenAI.
    pub base_endpoint: Option<String>,
}

impl OpenAi {
    pub fn base_endpoint(&self) -> String {
        self.base_endpoint
            .clone()
            .unwrap_or("https://api.openai.com".to_string())
    }
}

/// A mapping for Azure (Global) configuration contents
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Azure {
    /// Configuration content for cognitive/speech.
    pub speech: AzureSpeech,
}

/// Service key for use in multiple Azure services.
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AzureSpeech {
    /// Key content from <https://portal.azure.com/>
    pub key: String,

    /// Region name from <https://portal.azure.com/>
    pub region: String,
}
