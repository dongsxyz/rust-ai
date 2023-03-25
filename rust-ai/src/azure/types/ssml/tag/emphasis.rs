use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

/// An emphasis tag.
pub struct Emphasis {
    level: Option<EmphasisLevel>,
    content: String,
    internals: Vec<VoiceTagInternal>,
}

impl Emphasis {
    pub fn new(content: String) -> Self {
        Self {
            level: None,
            content,
            internals: vec![],
        }
    }

    /// When the level attribute isn't specified, the default level is
    /// moderate.
    pub fn level(self, level: EmphasisLevel) -> Self {
        Self {
            level: Some(level),
            ..self
        }
    }

    /// Insert additional tag as internal
    ///
    /// Note: text contents should also be inserted if you need to.
    pub fn insert(self, tag: impl Into<VoiceTagInternal>) -> Self {
        let converted = Into::<VoiceTagInternal>::into(tag);

        assert!(converted.allow_inside(VoiceTagName::Emphasis));

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

impl From<String> for Emphasis {
    fn from(value: String) -> Self {
        Self {
            content: value,
            level: None,
            internals: vec![],
        }
    }
}

impl Into<VoiceTagInternal> for Emphasis {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::new();
        if let Some(level) = self.level {
            attributes.insert("level".into(), level.into());
        }
        VoiceTagInternal::new(
            VoiceTagName::Emphasis,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}

pub enum EmphasisLevel {
    Reduced,
    None,
    Moderate,
    Strong,
}

impl Into<String> for EmphasisLevel {
    fn into(self) -> String {
        (match self {
            Self::Reduced => "reduced",
            Self::None => "none",
            Self::Moderate => "moderate",
            Self::Strong => "strong",
        })
        .into()
    }
}
