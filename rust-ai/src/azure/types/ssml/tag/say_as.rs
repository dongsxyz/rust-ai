use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

pub struct SayAs {
    content: String,
    format: String,
    detail: String,
    interpret_as: String,
}

impl SayAs {
    /// # Arguments
    /// - `content` - this must be plain text.
    pub fn new(format: String, detail: String, interpret_as: String, content: String) -> Self {
        Self {
            format,
            detail,
            interpret_as,
            content,
        }
    }
}
impl Into<VoiceTagInternal> for SayAs {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();
        attributes.insert("interpret-as".into(), self.interpret_as);
        attributes.insert("format".into(), self.format);
        attributes.insert("detail".into(), self.detail);

        VoiceTagInternal::new(VoiceTagName::SayAs, attributes, vec![], self.content)
    }
}
