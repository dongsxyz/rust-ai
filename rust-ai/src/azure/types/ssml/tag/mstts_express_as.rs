use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

/// Styling of (neural) voices
#[allow(non_camel_case_types)]
pub struct MSTTS_ExpressAs {
    style: ExpressAsStyle,
    styledegree: Option<f32>,
    role: Option<ExpressAsRole>,
    internals: Vec<VoiceTagInternal>,
    content: String,
}

impl MSTTS_ExpressAs {
    /// The voice-specific speaking style. You can express emotions like
    /// cheerfulness, empathy, and calm. You can also optimize the voice for
    /// different scenarios like customer service, newscast, and voice
    /// assistant.
    /// If the style value is missing or invalid, the entire mstts:express-as
    /// element is ignored and the service uses the default neutral speech. For
    /// custom neural voice styles, see the [custom neural voice style example](https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#custom-neural-voice-style-example).
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#speaking-styles-and-roles>
    pub fn new(style: ExpressAsStyle) -> Self {
        Self {
            style: style,
            styledegree: None,
            role: None,
            internals: vec![],
            content: String::new(),
        }
    }

    /// The intensity of the speaking style. You can specify a stronger or
    /// softer style to make the speech more expressive or subdued. The range
    /// of accepted values are: 0.01 to 2 inclusive. The default value is 1,
    /// which means the predefined style intensity. The minimum unit is 0.01,
    /// which results in a slight tendency for the target style. A value of 2
    /// results in a doubling of the default style intensity. If the style
    /// degree is missing or isn't supported for your voice, this attribute is
    /// ignored.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#speaking-styles-and-roles>
    pub fn styledegree(self, styledegree: f32) -> Self {
        Self {
            styledegree: Some(styledegree),
            ..self
        }
    }

    /// The speaking role-play. The voice can imitate a different age and
    /// gender, but the voice name isn't changed. For example, a male voice can
    /// raise the pitch and change the intonation to imitate a female voice,
    /// but the voice name won't be changed. If the role is missing or isn't
    /// supported for your voice, this attribute is ignored.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#speaking-styles-and-roles>
    pub fn role(self, role: ExpressAsRole) -> Self {
        Self {
            role: Some(role),
            ..self
        }
    }

    /// Set alternative text content
    ///
    /// Note: this will clear alter internal tags.
    pub fn content(self, content: String) -> Self {
        Self {
            internals: vec![],
            content,
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

impl Into<VoiceTagInternal> for MSTTS_ExpressAs {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::<String, String>::new();
        attributes.insert("style".into(), self.style.into());
        if let Some(styledegree) = self.styledegree {
            attributes.insert("styledegree".into(), styledegree.to_string());
        }
        if let Some(role) = self.role {
            attributes.insert("role".into(), role.into());
        }

        VoiceTagInternal::new(
            VoiceTagName::MSTTS_ExpressAs,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ExpressAsStyle {
    /// Expresses an excited and high-energy tone for promoting a product or service.
    Advertisement_Upbeat,
    /// Expresses a warm and affectionate tone, with higher pitch and vocal energy. The speaker is in a state of attracting the attention of the listener. The personality of the speaker is often endearing in nature.
    Affectionate,
    /// Expresses an angry and annoyed tone.
    Angry,
    /// Expresses a warm and relaxed tone for digital assistants.
    Assistant,
    /// Expresses a cool, collected, and composed attitude when speaking. Tone, pitch, and prosody are more uniform compared to other types of speech.
    Calm,
    /// Expresses a casual and relaxed tone.
    Chat,
    /// Expresses a positive and happy tone.
    Cheerful,
    /// Expresses a friendly and helpful tone for customer support.
    Customerservice,
    /// Expresses a melancholic and despondent tone with lower pitch and energy.
    Depressed,
    /// Expresses a disdainful and complaining tone. Speech of this emotion displays displeasure and contempt.
    Disgruntled,
    /// Narrates documentaries in a relaxed, interested, and informative style suitable for dubbing documentaries, expert commentary, and similar content.
    Documentary_Narration,
    /// Expresses an uncertain and hesitant tone when the speaker is feeling uncomfortable.
    Embarrassed,
    /// Expresses a sense of caring and understanding.
    Empathetic,
    /// Expresses a tone of admiration when you desire something that someone else has.
    Envious,
    /// Expresses an upbeat and hopeful tone. It sounds like something great is happening and the speaker is really happy about that.
    Excited,
    /// Expresses a scared and nervous tone, with higher pitch, higher vocal energy, and faster rate. The speaker is in a state of tension and unease.
    Fearful,
    /// Expresses a pleasant, inviting, and warm tone. It sounds sincere and caring.
    Friendly,
    /// Expresses a mild, polite, and pleasant tone, with lower pitch and vocal energy.
    Gentle,
    /// Expresses a warm and yearning tone. It sounds like something good will happen to the speaker.
    Hopeful,
    /// Expresses emotions in a melodic and sentimental way.
    Lyrical,
    /// Expresses a professional, objective tone for content reading.
    Narration_Professional,
    /// Express a soothing and melodious tone for content reading.
    Narration_Relaxed,
    /// Expresses a formal and professional tone for narrating news.
    Newscast,
    /// Expresses a versatile and casual tone for general news delivery.
    Newscast_Casual,
    /// Expresses a formal, confident, and authoritative tone for news delivery.
    Newscast_Formal,
    /// Expresses an emotional and rhythmic tone while reading a poem.
    Poetry_Reading,
    /// Expresses a sorrowful tone.
    Sad,
    /// Expresses a strict and commanding tone. Speaker often sounds stiffer and much less relaxed with firm cadence.
    Serious,
    /// Speaks like from a far distant or outside and to make self be clearly heard
    Shouting,
    /// Expresses a relaxed and interesting tone for broadcasting a sports event.
    Sports_Commentary,
    /// Expresses an intensive and energetic tone for broadcasting exciting moments in a sports event.
    Sports_Commentary_Excited,
    /// Speaks very softly and make a quiet and gentle sound
    Whispering,
    /// Expresses a very scared tone, with faster pace and a shakier voice. It sounds like the speaker is in an unsteady and frantic status.
    Terrified,
    /// Expresses a cold and indifferent tone.
    Unfriendly,
}

impl Into<String> for ExpressAsStyle {
    fn into(self) -> String {
        (match self {
            Self::Advertisement_Upbeat => "advertisement_upbeat",
            Self::Affectionate => "affectionate",
            Self::Angry => "angry",
            Self::Assistant => "assistant",
            Self::Calm => "calm",
            Self::Chat => "chat",
            Self::Cheerful => "cheerful",
            Self::Customerservice => "customerservice",
            Self::Depressed => "depressed",
            Self::Disgruntled => "disgruntled",
            Self::Documentary_Narration => "documentary-narration",
            Self::Embarrassed => "embarrassed",
            Self::Empathetic => "empathetic",
            Self::Envious => "envious",
            Self::Excited => "excited",
            Self::Fearful => "fearful",
            Self::Friendly => "friendly",
            Self::Gentle => "gentle",
            Self::Hopeful => "hopeful",
            Self::Lyrical => "lyrical",
            Self::Narration_Professional => "narration-professional",
            Self::Narration_Relaxed => "narration-relaxed",
            Self::Newscast => "newscast",
            Self::Newscast_Casual => "newscast-casual",
            Self::Newscast_Formal => "newscast-formal",
            Self::Poetry_Reading => "poetry-reading",
            Self::Sad => "sad",
            Self::Serious => "serious",
            Self::Shouting => "shouting",
            Self::Sports_Commentary => "sports_commentary",
            Self::Sports_Commentary_Excited => "sports_commentary_excited",
            Self::Whispering => "whispering",
            Self::Terrified => "terrified",
            Self::Unfriendly => "unfriendly",
        })
        .into()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ExpressAsRole {
    /// The voice imitates a girl
    Girl,

    /// The voice imitates a boy
    Boy,

    /// The voice imitates a young adult female
    YoungAdultFemale,

    /// The voice imitates a young adult male
    YoungAdultMale,

    /// The voice imitates an older adult female
    OlderAdultFemale,

    /// The voice imitates an older adult male
    OlderAdultMale,

    /// The voice imitates a senior female
    SeniorFemale,

    /// The voice imitates a senior male
    SeniorMale,
}

impl Into<String> for ExpressAsRole {
    fn into(self) -> String {
        (match self {
            Self::Girl => "Girl",
            Self::Boy => "Boy",
            Self::YoungAdultFemale => "YoungAdultFemale",
            Self::YoungAdultMale => "YoungAdultMale",
            Self::OlderAdultFemale => "OlderAdultFemale",
            Self::OlderAdultMale => "OlderAdultMale",
            Self::SeniorFemale => "SeniorFemale",
            Self::SeniorMale => "SeniorMale",
        })
        .into()
    }
}
