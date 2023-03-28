use serde::{Deserialize, Serialize};

/// DiarizationProperties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiarizationProperties {
    /// DiarizationSpeakersProperties
    pub speakers: DiarizationSpeakersProperties,
}

impl Default for DiarizationProperties {
    fn default() -> Self {
        Self {
            speakers: DiarizationSpeakersProperties::default(),
        }
    }
}

/// DiarizationSpeakersProperties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiarizationSpeakersProperties {
    /// A hint for the minimum number of speakers for diarization. Must be
    /// smaller than or equal to the maxSpeakers property.
    ///
    /// Minimal: 1
    #[serde(rename = "minCount")]
    pub min_count: usize,

    /// The maximum number of speakers for diarization. Must be less than 36 and
    /// larger than or equal to the minSpeakers property.
    ///
    /// Minimal: 1
    #[serde(rename = "maxCount")]
    pub max_count: usize,
}

impl Default for DiarizationSpeakersProperties {
    fn default() -> Self {
        Self {
            min_count: 1,
            max_count: 1,
        }
    }
}
