use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

pub struct Math {
    content: String,
}

impl Math {
    /// # Arguments
    /// - `content` - this must be plain text or MathML contents. See <https://developer.mozilla.org/en-US/docs/Web/MathML/Element>.
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
impl From<String> for Math {
    fn from(value: String) -> Self {
        Self { content: value }
    }
}

impl Into<VoiceTagInternal> for Math {
    fn into(self) -> VoiceTagInternal {
        let attributes = HashMap::<String, String>::new();

        VoiceTagInternal::new(VoiceTagName::Math, attributes, vec![], self.content)
    }
}
