use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct MSTTS_Viseme {
    _type: SilenceType,
}

impl MSTTS_Viseme {
    pub fn new(ty: SilenceType) -> Self {
        Self { _type: ty }
    }
}

impl From<SilenceType> for MSTTS_Viseme {
    fn from(value: SilenceType) -> Self {
        Self { _type: value }
    }
}

impl Into<VoiceTagInternal> for MSTTS_Viseme {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::new();
        attributes.insert("type".into(), self._type.into());

        VoiceTagInternal::new(
            VoiceTagName::MSTTS_Viseme,
            attributes,
            vec![],
            String::new(),
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SilenceType {
    /// lip-sync with viseme ID and audio offset output
    Redlips_Front,

    /// blend shapes output
    FacialExpression,
}

impl Into<String> for SilenceType {
    fn into(self) -> String {
        (match self {
            Self::Redlips_Front => "redlips_front",
            Self::FacialExpression => "FacialExpression",
        })
        .into()
    }
}
