use std::collections::HashMap;

use crate::azure::VoiceName;

use super::{VoiceTagInternal, VoiceTagName};

#[derive(Debug, Clone)]
pub struct Voice {
    name: VoiceName,
    effect: Option<VoiceEffect>,
    internals: Vec<VoiceTagInternal>,
    content: String,
}

/// At least one voice element must be specified within each SSML speak
/// element. This element determines the voice that's used for text-to-speech.
///
/// You can include multiple voice elements in a single SSML document. Each
/// voice element can specify a different voice. You can also use the same
/// voice multiple times with different settings, such as when you change the
///  silence duration between sentences.
///
/// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#voice-element>
impl Voice {
    pub fn new(name: VoiceName) -> Self {
        Self {
            name,
            effect: None,
            content: String::new(),
            internals: vec![],
        }
    }

    /// The audio effect processor that's used to optimize the quality of the
    ///  synthesized speech output for specific scenarios on devices.
    ///
    /// For some scenarios in production environments, the auditory experience
    ///  may be degraded due to the playback distortion on certain devices. For
    /// example, the synthesized speech from a car speaker may sound dull and
    /// muffled due to environmental factors such as speaker response, room
    /// reverberation, and background noise. The passenger might have to turn
    /// up the volume to hear more clearly. To avoid manual operations in such
    /// a scenario, the audio effect processor can make the sound clearer by
    /// compensating the distortion of playback.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#voice-element>
    pub fn effect(self, effect: VoiceEffect) -> Self {
        Self {
            effect: Some(effect),
            ..self
        }
    }

    /// Insert additional tag as internal
    ///
    /// Note: text contents should also be inserted if you need to.
    pub fn insert(self, tag: impl Into<VoiceTagInternal>) -> Self {
        let converted = Into::<VoiceTagInternal>::into(tag);

        assert!(converted.allow_inside(VoiceTagName::Voice));

        let mut internals = vec![];
        internals.extend(self.internals);
        internals.push(converted);

        Self {
            content: String::new(),
            internals: internals,
            ..self
        }
    }

    pub fn content(self, content: String) -> Self {
        Self { content, ..self }
    }
}

impl Into<VoiceTagInternal> for Voice {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();

        attributes.insert("name".into(), self.name.into());
        if let Some(effect) = self.effect {
            attributes.insert("effect".into(), effect.into());
        }

        VoiceTagInternal::new(
            VoiceTagName::Voice,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}

impl From<VoiceName> for Voice {
    fn from(value: VoiceName) -> Self {
        Self {
            name: value,
            effect: None,
            internals: vec![],
            content: String::new(),
        }
    }
}

/// Voice effect for SSML `voice` element
///
/// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#voice-element>
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum VoiceEffect {
    /// Optimize the auditory experience when providing high-fidelity speech in
    /// cars, buses, and other enclosed automobiles.
    Eq_Car,

    /// Optimize the auditory experience for narrowband speech in telecom or
    /// telephone scenarios. We recommend a sampling rate of 8 kHz. If the
    /// sample rate isn't 8 kHz, the auditory quality of the output speech
    /// won't be optimized.
    Eq_Telecomhp8k,
}

impl Into<String> for VoiceEffect {
    fn into(self) -> String {
        (match self {
            Self::Eq_Car => "eq_car",
            Self::Eq_Telecomhp8k => "eq_telecomhp8k",
        })
        .into()
    }
}
