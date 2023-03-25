use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

pub struct Bookmark {
    mark: String,
}

impl Bookmark {
    pub fn new(bmk: String) -> Self {
        Self { mark: bmk }
    }
}

impl Into<VoiceTagInternal> for Bookmark {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();

        attributes.insert("mark".into(), self.mark);

        VoiceTagInternal::new(VoiceTagName::Bookmark, attributes, vec![], String::new())
    }
}
