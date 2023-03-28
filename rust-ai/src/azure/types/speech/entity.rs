use serde::{Deserialize, Serialize};

/// An optional mapping of locales to speech model entities. If no model is
/// given for a locale, the default base model is used.
///
/// Keys must be locales contained in the candidate locales, values are
/// entities for models of the respective locales.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityReference {
    /// The location of the referenced entity.
    ///
    /// Must be a valid URI string.
    #[serde(rename = "self")]
    pub _self: String,
}

impl From<String> for EntityReference {
    fn from(value: String) -> Self {
        Self { _self: value }
    }
}

impl EntityReference {
    pub fn from(s: String) -> Self {
        Self { _self: s }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityError {
    /// The code of this error.
    pub code: String,

    /// The message for this error.
    pub description: String,
}

impl Default for EntityError {
    fn default() -> Self {
        Self {
            code: String::new(),
            description: String::new(),
        }
    }
}
