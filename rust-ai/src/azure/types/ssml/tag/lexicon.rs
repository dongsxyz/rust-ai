use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

pub struct Lexicon {
    uri: String,
}

impl Lexicon {
    pub fn new(uri: String) -> Self {
        Self { uri: uri }
    }
}

impl From<String> for Lexicon {
    fn from(value: String) -> Self {
        Self { uri: value }
    }
}

impl Into<VoiceTagInternal> for Lexicon {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();
        attributes.insert("uri".into(), self.uri.clone());

        VoiceTagInternal::new(VoiceTagName::Lexicon, attributes, vec![], String::new())
    }
}
