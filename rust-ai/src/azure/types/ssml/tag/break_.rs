use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

/// Use the break element to override the default behavior of breaks or pauses
/// between words. You can use it to add pauses that are otherwise
/// automatically inserted by the Speech service.
///
/// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-structure>
pub struct Break {
    strength: Option<Strength>,
    time: Option<String>,
}

impl Break {
    pub fn new() -> Self {
        Self {
            strength: None,
            time: None,
        }
    }

    pub fn strength(self, strength: Strength) -> Self {
        Self {
            strength: Some(strength),
            time: None,
        }
    }

    /// The absolute duration of a pause in seconds (such as 2s) or
    /// milliseconds (such as 500ms). Valid values range from 0 to 5000
    /// milliseconds. If you set a value greater than the supported maximum,
    /// the service will use 5000ms. If the time attribute is set, the
    /// strength attribute is ignored.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-structure>
    pub fn time(self, time: String) -> Self {
        Self {
            time: Some(time),
            strength: None,
        }
    }
}

impl Into<VoiceTagInternal> for Break {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();
        if let Some(strength) = self.strength {
            attributes.insert("strength".into(), strength.into());
        }
        if let Some(time) = self.time {
            attributes.insert("time".into(), time);
        }

        VoiceTagInternal::new(VoiceTagName::Break, attributes, vec![], String::new())
    }
}

#[allow(non_camel_case_types)]
pub enum Strength {
    /// Relative duration: 250 ms
    X_Weak,

    /// Relative duration: 500 ms
    Weak,

    /// Relative duration: 750 ms
    Medium,

    /// Relative duration: 1,000 ms
    Strong,

    /// Relative duration: 1,250 ms
    X_Strong,
}

impl Into<String> for Strength {
    fn into(self) -> String {
        (match self {
            Self::X_Weak => "x-weak",
            Self::Weak => "weak",
            Self::Medium => "medium",
            Self::Strong => "strong",
            Self::X_Strong => "x-strong",
        })
        .into()
    }
}
