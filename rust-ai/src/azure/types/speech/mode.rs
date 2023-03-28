use serde::{Deserialize, Serialize};

/// The mode used for punctuation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PunctuationMode {
    /// No punctuation.
    None,

    /// Dictated punctuation marks only, i.e., explicit punctuation.
    Dictated,

    /// Automatic punctuation.
    Automatic,

    /// Dictated punctuation marks or automatic punctuation.
    DictatedAndAutomatic,
}

impl Into<String> for PunctuationMode {
    fn into(self) -> String {
        (match self {
            Self::None => "None",
            Self::Dictated => "Dictated",
            Self::Automatic => "Automatic",
            Self::DictatedAndAutomatic => "DictatedAndAutomatic",
        })
        .into()
    }
}

/// ProfanityFilterMode
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProfanityFilterMode {
    /// Disable profanity filtering.
    None,

    /// Remove profanity.
    Removed,

    /// Add "profanity" XML tags</Profanity>
    Tags,

    /// Mask the profanity with * except of the first letter, e.g., f***
    Masked,
}

impl Into<String> for ProfanityFilterMode {
    fn into(self) -> String {
        (match self {
            Self::None => "None",
            Self::Removed => "Removed",
            Self::Tags => "Tags",
            Self::Masked => "Masked",
        })
        .into()
    }
}
