use std::collections::HashMap;

use super::{VoiceTagInternal, VoiceTagName};

/// An prosody tag.
pub struct Prosody {
    pitch: Option<String>,
    contour: Option<String>,
    range: Option<String>,
    rate: Option<String>,
    volume: Option<String>,
    content: String,
    internals: Vec<VoiceTagInternal>,
}
impl Default for Prosody {
    fn default() -> Self {
        Self {
            pitch: None,
            contour: None,
            range: None,
            rate: None,
            volume: None,
            content: String::new(),
            internals: vec![],
        }
    }
}
impl Prosody {
    /// Indicates the baseline pitch for the text. Pitch changes can be applied
    /// at the sentence level. The pitch changes should be within 0.5 to 1.5
    /// times the original audio. You can express the pitch as:
    ///
    /// - An absolute value: Expressed as a number followed by "Hz" (Hertz).
    ///   For example, `<prosody pitch="600Hz">some text</prosody>`.
    /// - A relative value:
    ///     - As a relative number: Expressed as a number preceded by "+" or "-"
    ///       and followed by "Hz" or "st" that specifies an amount to change
    ///       the pitch. For example:
    ///       `<prosody pitch="+80Hz">some text</prosody>` or
    ///       `<prosody pitch="-2st">some text</prosody>`. The "st" indicates
    ///       the change unit is semitone, which is half of a tone (a half step)
    ///       on the standard diatonic scale.
    ///     - As a percentage: Expressed as a number preceded by "+"
    ///       (optionally) or "-" and followed by "%", indicating the relative
    ///       change. For example: `<prosody pitch="50%">some text</prosody>` or
    ///       `<prosody pitch="-50%">some text</prosody>`.
    ///     - A constant value:
    ///         - `x-low`
    ///         - `low`
    ///         - `medium`
    ///         - `high`
    ///         - `x-high`
    ///         - `default`
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#adjust-prosody>
    pub fn pitch(self, pitch: String) -> Self {
        Self {
            pitch: Some(pitch),
            ..self
        }
    }

    /// Contour represents changes in pitch. These changes are represented as
    /// an array of targets at specified time positions in the speech output.
    /// Each target is defined by sets of parameter pairs. For example:
    ///
    /// `<prosody contour="(0%,+20Hz) (10%,-2st) (40%,+10Hz)">`
    ///
    /// The first value in each set of parameters specifies the location of the
    /// pitch change as a percentage of the duration of the text. The second
    /// value specifies the amount to raise or lower the pitch by using a
    /// relative value or an enumeration value for pitch (see `pitch`).
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#adjust-prosody>
    pub fn contour(self, contour: String) -> Self {
        Self {
            contour: Some(contour),
            ..self
        }
    }
    /// A value that represents the range of pitch for the text. You can
    /// express `range` by using the same absolute values, relative values, or
    /// enumeration values used to describe `pitch`.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#adjust-prosody>
    pub fn range(self, range: String) -> Self {
        Self {
            range: Some(range),
            ..self
        }
    }

    /// Indicates the speaking rate of the text. Speaking rate can be applied at
    /// the word or sentence level. The rate changes should be within 0.5 to 2
    /// times the original audio. You can express `rate` as:
    ///
    /// - A relative value:
    ///     - As a relative number: Expressed as a number that acts as a
    ///       multiplier of the default. For example, a value of 1 results in
    ///       no change in the original rate. A value of 0.5 results in a
    ///       halving of the original rate. A value of 2 results in twice the
    ///       original rate.
    ///     - As a percentage: Expressed as a number preceded by "+"
    ///       (optionally) or "-" and followed by "%", indicating the relative
    ///       change. For example: `<prosody rate="50%">some text</prosody>` or
    ///       `<prosody rate="-50%">some text</prosody>`.
    /// - A constant value:
    ///     - `x-slow`
    ///     - `slow`
    ///     - `medium`
    ///     - `fast`
    ///     - `x-fast`
    ///     - `default`
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#adjust-prosody>
    pub fn rate(self, rate: String) -> Self {
        Self {
            rate: Some(rate),
            ..self
        }
    }
    /// Indicates the volume level of the speaking voice. Volume changes can be
    /// applied at the sentence level. You can express the `volume` as:
    ///
    /// - An absolute value: Expressed as a number in the range of 0.0 to 100.0,
    ///   from quietest to loudest. An example is 75. The default is 100.0.
    /// - A relative value:
    ///     - As a relative number: Expressed as a number preceded by "+" or "-"
    ///       that specifies an amount to change the volume. Examples are +10 or
    ///       -5.5.
    ///     - As a percentage: Expressed as a number preceded by "+"
    ///       (optionally) or "-" and followed by "%", indicating the relative
    ///       change. For example: `<prosody volume="50%">some text</prosody>`
    ///       or `<prosody volume="+3%">some text</prosody>`.
    /// - A constant value:
    ///     - `silent`
    ///     - `x-soft`
    ///     - `soft`
    ///     - `medium`
    ///     - `loud`
    ///     - `x-loud`
    ///     - `default`
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-voice#adjust-prosody>
    pub fn volume(self, volume: String) -> Self {
        Self {
            volume: Some(volume),
            ..self
        }
    }

    /// Set alternative text content
    ///
    /// Note: this will clear alter internal tags.
    pub fn alt_content(self, txt: String) -> Self {
        Self {
            internals: vec![],
            content: txt,
            ..self
        }
    }

    /// Insert additional tag as internal
    ///
    /// Note: text contents should also be inserted if you need to.
    pub fn insert(self, tag: impl Into<VoiceTagInternal>) -> Self {
        let converted = Into::<VoiceTagInternal>::into(tag);

        assert!(converted.allow_inside(VoiceTagName::Prosody));

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

impl Into<VoiceTagInternal> for Prosody {
    fn into(self) -> VoiceTagInternal {
        let mut attributes = HashMap::new();
        if let Some(pitch) = self.pitch {
            attributes.insert("pitch".into(), pitch);
        }
        if let Some(contour) = self.contour {
            attributes.insert("contour".into(), contour);
        }
        if let Some(range) = self.range {
            attributes.insert("range".into(), range);
        }
        if let Some(rate) = self.rate {
            attributes.insert("rate".into(), rate);
        }
        if let Some(volume) = self.volume {
            attributes.insert("volume".into(), volume);
        }
        VoiceTagInternal::new(
            VoiceTagName::Prosody,
            attributes,
            self.internals.clone(),
            self.content.clone(),
        )
    }
}
