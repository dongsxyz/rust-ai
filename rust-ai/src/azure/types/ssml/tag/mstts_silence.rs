use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct MSTTS_Silence {
    _type: SilenceType,
    value: String,
}

impl MSTTS_Silence {
    pub fn new(ty: SilenceType, value: String) -> Self {
        Self { value, _type: ty }
    }
}

impl Into<VoiceTagInternal> for MSTTS_Silence {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::new();
        attributes.insert("type".into(), self._type.into());
        attributes.insert("value".into(), self.value);

        VoiceTagInternal::new(
            VoiceTagName::MSTTS_Silence,
            attributes,
            vec![],
            String::new(),
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SilenceType {
    /// Additional silence at the beginning of the text. The value that you set
    ///  is added to the natural silence before the start of text.
    Leading,

    /// Silence at the beginning of the text. The value is an absolute silence
    /// length.
    Leading_exact,

    /// Additional silence at the end of text. The value that you set is added
    ///  to the natural silence after the last word.
    Tailing,

    /// Silence at the end of the text. The value is an absolute silence length.
    Tailing_exact,

    /// Additional silence between adjacent sentences. The actual silence length
    ///  for this type includes the natural silence after the last word in the
    /// previous sentence, the value you set for this type, and the natural
    /// silence before the starting word in the next sentence.
    Sentenceboundary,

    /// Silence between adjacent sentences. The value is an absolute silence
    /// length.
    Sentenceboundary_exact,

    /// Silence at the comma in half-width or full-width format. The value is
    /// an absolute silence length.
    Comma_exact,

    /// Silence at the semicolon in half-width or full-width format. The value
    /// is an absolute silence length.
    Semicolon_exact,

    /// Silence at the enumeration comma in full-width format. The value is an
    /// absolute silence length.
    Enumerationcomma_exact,
}

impl Into<String> for SilenceType {
    fn into(self) -> String {
        (match self {
            Self::Leading => "Leading",
            Self::Leading_exact => "Leading-exact",
            Self::Tailing => "Tailing",
            Self::Tailing_exact => "Tailing-exact",
            Self::Sentenceboundary => "Sentenceboundary",
            Self::Sentenceboundary_exact => "Sentenceboundary-exact",
            Self::Comma_exact => "Comma-exact",
            Self::Semicolon_exact => "Semicolon-exact",
            Self::Enumerationcomma_exact => "Enumerationcomma-exact",
        })
        .into()
    }
}
