use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct MSTTS_AudioDuration {
    value: String,
}

impl MSTTS_AudioDuration {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

impl Into<VoiceTagInternal> for MSTTS_AudioDuration {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::new();
        attributes.insert("value".into(), self.value);
        
        VoiceTagInternal::new(
            VoiceTagName::MSTTS_AudioDuration,
            attributes,
            vec![],
            String::new(),
        )
    }
}
