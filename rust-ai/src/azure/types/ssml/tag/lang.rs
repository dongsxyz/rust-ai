use std::collections::HashMap;

use crate::azure::Locale;

use super::{VoiceTagInternal, VoiceTagName};

/// A lang tag.
pub struct Lang {
    lang: Option<String>,
    content: String,
    internals: Vec<VoiceTagInternal>,
}

impl Lang {
    pub fn new(lang: Locale) -> Self {
        Self {
            lang: Some(lang.into()),
            content: String::new(),
            internals: vec![],
        }
    }

    /// Insert additional tag as internal
    ///
    /// Note: text contents should also be inserted if you need to.
    pub fn insert(self, tag: impl Into<VoiceTagInternal>) -> Self {
        let converted = Into::<VoiceTagInternal>::into(tag);

        assert!(converted.allow_inside(VoiceTagName::Lang));

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

impl From<String> for Lang {
    fn from(value: String) -> Self {
        Self {
            content: value,
            lang: None,
            internals: vec![],
        }
    }
}

impl Into<VoiceTagInternal> for Lang {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::new();
        if let Some(lang) = self.lang {
            attributes.insert("xml:lang".into(), lang);
        }
        VoiceTagInternal::new(
            VoiceTagName::Lang,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}
