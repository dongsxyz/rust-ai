use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct MSTTS_BackgroundAudio {
    src: String,
    volume: Option<u32>,
    fadein: Option<u32>,
    fadeout: Option<u32>,
}

impl MSTTS_BackgroundAudio {
    /// The URI location of the background audio file.
    pub fn new(src: String) -> Self {
        Self {
            src: src,
            volume: None,
            fadein: None,
            fadeout: None,
        }
    }

    /// The volume of the background audio file. Accepted values: 0 to 100
    /// inclusive. The default value is 1.
    pub fn volume(self, vol: u32) -> Self {
        Self {
            volume: Some(vol),
            ..self
        }
    }

    /// The duration of the background audio fade-in as milliseconds. The
    /// default value is 0, which is the equivalent to no fade in. Accepted
    /// values: 0 to 10000 inclusive.
    pub fn fadein(self, fd: u32) -> Self {
        Self {
            fadein: Some(fd),
            ..self
        }
    }

    /// The duration of the background audio fade-out in milliseconds. The
    /// default value is 0, which is the equivalent to no fade out. Accepted
    /// values: 0 to 10000 inclusive.
    pub fn fadeout(self, fd: u32) -> Self {
        Self {
            fadeout: Some(fd),
            ..self
        }
    }
}

impl Into<VoiceTagInternal> for MSTTS_BackgroundAudio {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::new();
        attributes.insert("src".into(), self.src);
        if let Some(volume) = self.volume {
            attributes.insert("volume".into(), volume.to_string());
        }
        if let Some(fadein) = self.fadein {
            attributes.insert("fadein".into(), fadein.to_string());
        }
        if let Some(fadeout) = self.fadeout {
            attributes.insert("fadeout".into(), fadeout.to_string());
        }

        VoiceTagInternal::new(
            VoiceTagName::MSTTS_BackgroundAudio,
            attributes,
            vec![],
            String::new(),
        )
    }
}
