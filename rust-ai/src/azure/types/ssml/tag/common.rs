use std::collections::HashMap;

enum TagEnclosureType {
    Closed,
    Open,
    None,
}

#[derive(Debug, Clone)]
pub struct VoiceTagInternal {
    tag_name: VoiceTagName,
    attributes: HashMap<String, String>,
    inner: Vec<Self>,
    content: String,
}

impl VoiceTagInternal {
    pub fn new(
        tag_name: VoiceTagName,
        attributes: HashMap<String, String>,
        inner: Vec<Self>,
        content: String,
    ) -> Self {
        Self {
            tag_name,
            attributes,
            inner,
            content,
        }
    }

    /// Test if current node can be a child of given target.
    pub fn allow_inside(&self, outer: VoiceTagName) -> bool {
        type T = VoiceTagName;
        match outer {
            // <audio>
            T::Audio => [
                T::Audio,
                T::Break,
                T::P,
                T::S,
                T::Phoneme,
                T::Prosody,
                T::SayAs,
                T::Sub,
            ]
            .contains(&self.tag_name),
            // <bookmark>, <break>, <lexicon>, <mstts:audioduration>, <mstts:backgroundaudio>, <mstts:silence>, <mstts:viseme>
            T::Bookmark
            | T::Break
            | T::Lexicon
            | T::MSTTS_AudioDuration
            | T::MSTTS_BackgroundAudio
            | T::MSTTS_Silence
            | T::MSTTS_Viseme
            | T::TEXT => false,
            // <emphasis>, <mstts:express-as>
            T::Emphasis | T::MSTTS_ExpressAs => [
                T::Audio,
                T::Break,
                T::Emphasis,
                T::Lang,
                T::Phoneme,
                T::Prosody,
                T::SayAs,
                T::Sub,
            ]
            .contains(&self.tag_name),
            // <lang>
            T::Lang => ![T::MSTTS_BackgroundAudio, T::Voice, T::Speak].contains(&self.tag_name),
            // <math>, <phoneme>, <say-as>, <sub>
            T::Math | T::Phoneme | T::SayAs | T::Sub => T::TEXT == self.tag_name,
            // <speak>
            T::Speak => [T::Voice, T::MSTTS_BackgroundAudio].contains(&self.tag_name),
            // <voice>
            T::Voice => ![T::Speak, T::MSTTS_BackgroundAudio].contains(&self.tag_name),
            // <prosody>,
            T::Prosody => [
                T::Audio,
                T::Break,
                T::P,
                T::Phoneme,
                T::Prosody,
                T::SayAs,
                T::Sub,
                T::S,
            ]
            .contains(&self.tag_name),
            T::P => [
                T::Audio,
                T::Break,
                T::Phoneme,
                T::Prosody,
                T::MSTTS_ExpressAs,
                T::SayAs,
                T::Sub,
                T::S,
            ]
            .contains(&self.tag_name),
            T::S => [
                T::Audio,
                T::Break,
                T::Phoneme,
                T::Prosody,
                T::MSTTS_ExpressAs,
                T::SayAs,
                T::Sub,
            ]
            .contains(&self.tag_name),
        }
    }
}

impl Into<String> for VoiceTagInternal {
    fn into(self) -> String {
        let convert_type = match self.tag_name {
            VoiceTagName::TEXT => TagEnclosureType::None,
            VoiceTagName::Bookmark
            | VoiceTagName::Lexicon
            | VoiceTagName::MSTTS_AudioDuration
            | VoiceTagName::MSTTS_Silence
            | VoiceTagName::MSTTS_Viseme => TagEnclosureType::Open,
            _ => TagEnclosureType::Closed,
        };

        let mut output = match convert_type {
            TagEnclosureType::None => String::new(),
            _ => format!("<{}", Into::<String>::into(self.tag_name)),
        };

        // Concat attributes
        if self.attributes.len() > 0 {
            let mut sorted_attributes = self
                .attributes
                .iter()
                .map(|attr| (attr.0.clone(), attr.1.clone()))
                .collect::<Vec<(String, String)>>();
            sorted_attributes.sort_by(|a, b| a.0.cmp(&b.0));

            sorted_attributes.iter().for_each(|attr| {
                output.push_str(&format!(r#" {}="{}""#, attr.0, attr.1));
            })
        }

        // Close first tag.
        output.push_str(match convert_type {
            TagEnclosureType::Open => "/>",
            TagEnclosureType::Closed => ">",
            TagEnclosureType::None => "",
        });

        // Insert inner tags
        if self.inner.len() > 0 {
            self.inner.iter().for_each(|inner| {
                output.push_str(&Into::<String>::into(inner.clone()));
            })
        } else {
            // Insert content
            output.push_str(&self.content.clone());
        }

        // Insert close tag
        match convert_type {
            TagEnclosureType::Closed => output.push_str(&format!(
                "</{}>",
                Into::<String>::into(self.tag_name.clone())
            )),
            _ => (),
        };

        output
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceTagName {
    /// SSML tag element "`audio`"
    Audio,
    /// SSML tag element "`bookmark`"
    Bookmark,
    /// SSML tag element "`break`"
    Break,
    /// SSML tag element "`emphasis`"
    Emphasis,
    /// SSML tag element "`lang`"
    Lang,
    /// SSML tag element "`lexicon`"
    Lexicon,
    /// SSML tag element "`math`"
    Math,
    /// SSML tag element "`mstts:backgroundaudio`"
    MSTTS_BackgroundAudio,
    /// SSML tag element "`mstts:audioduration`"
    MSTTS_AudioDuration,
    /// SSML tag element "`mstts:express-as`"
    MSTTS_ExpressAs,
    /// SSML tag element "`mstts:silence`"
    MSTTS_Silence,
    /// SSML tag element "`mstts:viseme`"
    MSTTS_Viseme,
    /// SSML tag element "`p`"
    P,
    /// SSML tag element "`phoneme`"
    Phoneme,
    /// SSML tag element "`prosody`"
    Prosody,
    /// SSML tag element "`s`"
    S,
    /// SSML tag element "`say-as`"
    SayAs,
    /// SSML tag element "`sub`"
    Sub,
    /// SSML tag element "`speak`"
    Speak,
    /// SSML tag element "`voice`"
    Voice,
    /// Plain text element
    TEXT,
}

impl Into<String> for VoiceTagName {
    fn into(self) -> String {
        (match self {
            Self::Audio => "audio",
            Self::Bookmark => "bookmark",
            Self::Break => "break",
            Self::Emphasis => "emphasis",
            Self::Lang => "lang",
            Self::Lexicon => "lexicon",
            Self::Math => "math",
            Self::MSTTS_AudioDuration => "mstts:audioduration",
            Self::MSTTS_BackgroundAudio => "mstts:backgroundaudio",
            Self::MSTTS_ExpressAs => "mstts:express-as",
            Self::MSTTS_Silence => "mstts:silence",
            Self::MSTTS_Viseme => "mstts:viseme",
            Self::P => "p",
            Self::Phoneme => "phoneme",
            Self::Prosody => "prosody",
            Self::S => "s",
            Self::SayAs => "say-as",
            Self::Sub => "sub",
            Self::Speak => "speak",
            Self::Voice => "voice",
            _ => "",
        })
        .into()
    }
}

impl ToString for VoiceTagInternal {
    fn to_string(&self) -> String {
        Into::<String>::into(self.clone())
    }
}
