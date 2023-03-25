use std::collections::HashMap;

use crate::azure::Locale;

use super::{
    mstts_background_audio::MSTTS_BackgroundAudio, voice::Voice, VoiceTagInternal, VoiceTagName,
};

#[derive(Debug, Clone)]
pub struct Speak {
    voice: Vec<Voice>,
    background_audio: Option<MSTTS_BackgroundAudio>,
    lang: Locale,
}

impl Speak {
    pub fn new(voice: Voice) -> Self {
        Self {
            voice: vec![voice],
            background_audio: None,
            lang: Locale::en_US,
        }
    }

    pub fn background_audio(self, background_audio: MSTTS_BackgroundAudio) -> Self {
        Self {
            background_audio: Some(background_audio),
            ..self
        }
    }

    /// Insert an additional voice element
    pub fn voice(self, voice: Voice) -> Self {
        let mut voices = vec![];
        voices.extend(self.voice);
        voices.push(voice);
        Self {
            voice: voices,
            ..self
        }
    }
}

impl From<Locale> for Speak {
    fn from(value: Locale) -> Self {
        Self {
            lang: value,
            voice: vec![],
            background_audio: None,
        }
    }
}

impl Into<VoiceTagInternal> for Speak {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();

        attributes.insert("version".into(), "1.0".into());
        attributes.insert("xmlns".into(), "http://www.w3.org/2001/10/synthesis".into());
        attributes.insert("xmlns:mstts".into(), "https://www.w3.org/2001/mstts".into());
        attributes.insert("xml:lang".into(), self.lang.into());

        let mut internals = vec![];
        if let Some(background_audio) = self.background_audio {
            internals.push(background_audio.into());
        }
        internals.extend(
            self.voice
                .iter()
                .map(|v| Into::<VoiceTagInternal>::into(v.clone()))
                .collect::<Vec<VoiceTagInternal>>(),
        );

        VoiceTagInternal::new(VoiceTagName::Speak, attributes, internals, String::new())
    }
}
