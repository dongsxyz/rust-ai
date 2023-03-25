use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

pub struct Phoneme {
    content: String,
    alphabet: String,
    ph: String,
}

impl Phoneme {
    /// # Arguments
    /// - `content` - this must be plain text.
    pub fn new(alphabet: String, ph: String, content: String) -> Self {
        Self {
            alphabet,
            ph,
            content,
        }
    }
}
impl Into<VoiceTagInternal> for Phoneme {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();
        attributes.insert("alphabet".into(), self.alphabet);
        attributes.insert("ph".into(), self.ph);

        VoiceTagInternal::new(VoiceTagName::Phoneme, attributes, vec![], self.content)
    }
}
