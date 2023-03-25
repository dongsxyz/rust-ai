use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

/// An p tag.
pub struct P {
    content: String,
    internals: Vec<VoiceTagInternal>,
}

impl P {
    pub fn new(content: String) -> Self {
        Self {
            content,
            internals: vec![],
        }
    }

    /// Insert additional tag as internal
    ///
    /// Note: text contents should also be inserted if you need to.
    pub fn insert(self, tag: impl Into<VoiceTagInternal>) -> Self {
        let converted = Into::<VoiceTagInternal>::into(tag);

        assert!(converted.allow_inside(VoiceTagName::P));

        let mut internals = vec![];
        internals.extend(self.internals);
        internals.push(converted);

        Self {
            content: String::new(),
            internals: internals,
            ..self
        }
    }
}

impl From<String> for P {
    fn from(value: String) -> Self {
        Self {
            content: value,
            internals: vec![],
        }
    }
}

impl Into<VoiceTagInternal> for P {
    fn into(self) -> VoiceTagInternal {
        let attributes = HashMap::new();

        VoiceTagInternal::new(
            VoiceTagName::P,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}
