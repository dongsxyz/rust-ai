use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

/// A plain text tag.
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self { content: value }
    }
}

impl Into<VoiceTagInternal> for Text {
    fn into(self) -> VoiceTagInternal {
        VoiceTagInternal::new(
            VoiceTagName::TEXT,
            HashMap::new(),
            vec![],
            self.content.clone(),
        )
    }
}
