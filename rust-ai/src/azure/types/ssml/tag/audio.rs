use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

pub struct Audio {
    src: String,
    internals: Vec<VoiceTagInternal>,
    content: String,
}

impl Audio {
    pub fn new(src: String) -> Self {
        Self {
            src: src,
            internals: vec![],
            content: String::new(),
        }
    }

    /// Set alternative text content
    ///
    /// Note: this will clear alter internal tags.
    pub fn alt_content(self, txt: String) -> Self {
        Self {
            internals: vec![],
            content: txt,
            ..self
        }
    }

    /// Insert additional tag as internal
    ///
    /// Note: text contents should also be inserted if you need to.
    pub fn insert(self, tag: impl Into<VoiceTagInternal>) -> Self {
        let converted = Into::<VoiceTagInternal>::into(tag);

        assert!(converted.allow_inside(VoiceTagName::Audio));

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

impl Into<VoiceTagInternal> for Audio {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();
        attributes.insert("src".into(), self.src);

        VoiceTagInternal::new(
            VoiceTagName::Audio,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}
