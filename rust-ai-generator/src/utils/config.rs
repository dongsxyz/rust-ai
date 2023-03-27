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
//! azure:
//!   speech:
//!     key: 4c7eXXXXXXXXXXXXXXXXXXXXXXX54c32
//!     region: westus
//! ```

////////////////////////////////////////////////////////////////////////////////

use std::{fs::read_to_string, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_yaml;

/// Configurations from `config.yml`
///
/// Example contents:
/// ```yaml
/// azure:
///   speech:
///     key: 4c7eXXXXXXXXXXXXXXXXXXXXXXX54c32
///     region: westus
/// ```
///
/// # Examples
///
/// ```rust
/// use rust_ai_generator::utils::config::Config;
///
/// let config = Config::load();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
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
                    Err(e.into())
                }
            }
        } else {
            Err("Unable to read `config.yml`".into())
        };
    }
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
