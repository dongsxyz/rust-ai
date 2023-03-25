use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

/// An s tag.
pub struct S {
    content: String,
    internals: Vec<VoiceTagInternal>,
}
impl Default for S {
    fn default() -> Self {
        Self {
            content: String::new(),
            internals: vec![],
        }
    }
}

impl S {
    pub fn new(self, content: String) -> Self {
        Self { content, ..self }
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

        assert!(converted.allow_inside(VoiceTagName::S));

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

impl Into<VoiceTagInternal> for S {
    fn into(self) -> VoiceTagInternal {
        let attributes = HashMap::new();
        VoiceTagInternal::new(
            VoiceTagName::S,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}
