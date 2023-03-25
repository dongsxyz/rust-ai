use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

pub struct Sub {
    content: String,
    alias: String,
}

impl Sub {
    /// # Arguments
    /// - `alias` - .
    /// - `content` - this must be plain text.
    pub fn new(alias: String, content: String) -> Self {
        Self {
            alias,
            content,
        }
    }
}
impl Into<VoiceTagInternal> for Sub {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();
        attributes.insert("alias".into(), self.alias);

        VoiceTagInternal::new(VoiceTagName::Sub, attributes, vec![], self.content)
    }
}
