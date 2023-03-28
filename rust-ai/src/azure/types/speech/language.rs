use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::azure::Locale;

use super::entity::EntityReference;

/// LanguageIdentificationProperties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LanguageIdentificationProperties {
    /// The candidate locales for language identification (example ["en-US",
    /// "de-DE", "es-ES"]). A minimum of 2 and a maximum of 10 candidate
    /// locales, including the main locale for the transcription, is supported.
    #[serde(rename = "candidateLocales")]
    pub candidate_locales: Vec<Locale>,

    /// An optional mapping of locales to speech model entities. If no model is
    /// given for a locale, the default base model is used.
    ///
    /// Keys must be locales contained in the candidate locales, values are
    /// entities for models of the respective locales.
    #[serde(rename = "speechModelMapping")]
    pub speech_model_mapping: HashMap<String, EntityReference>,
}

impl Default for LanguageIdentificationProperties {
    fn default() -> Self {
        Self {
            candidate_locales: vec![],
            speech_model_mapping: HashMap::<String, EntityReference>::new(),
        }
    }
}
