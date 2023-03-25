//!
//! # Basic Structure
//!
//! ```ssml+xml
//! <speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xmlns:mstts="https://www.w3.org/2001/mstts" xml:lang="string">
//! <mstts:backgroundaudio src="string" volume="string" fadein="string" fadeout="string"/>
//! <voice name="string" effect="string">
//!     <audio src="string"/></audio>
//!     <bookmark mark="string"/>
//!     <break strength="string" time="string" />
//!     <emphasis level="value"></emphasis>
//!     <lang xml:lang="string"></lang>
//!     <lexicon uri="string"/>
//!     <math xmlns="http://www.w3.org/1998/Math/MathML"></math>
//!     <mstts:audioduration value="string"/>
//!     <mstts:express-as style="string" styledegree="value" role="string"></mstts:express-as>
//!     <mstts:silence type="string" value="string"/>
//!     <mstts:viseme type="string"/>
//!     <p></p>
//!     <phoneme alphabet="string" ph="string"></phoneme>
//!     <prosody pitch="value" contour="value" range="value" rate="value" volume="value"></prosody>
//!     <s></s>
//!     <say-as interpret-as="string" format="string" detail="string"></say-as>
//!     <sub alias="string"></sub>
//! </voice>
//! </speak>
//! ```

////////////////////////////////////////////////////////////////////////////////

pub mod tag;
pub mod voice_name;

pub use self::tag::*;
use super::locale::Locale;

/// SSML format support
/// 
/// **Note**: to use the characters &, <, and > within the SSML element's value 
/// or text, you must use the entity format. Specifically you must use &amp; in 
/// place of &, use &lt; in place of <, and use &gt; in place of >. Otherwise 
/// the SSML will not be parsed correctly.
/// 
/// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup-structure#special-characters>
#[derive(Debug, Clone)]
pub struct SSML {
    speak: Speak,
}

impl Default for SSML {
    fn default() -> Self {
        Self {
            speak: Speak::from(Locale::en_US),
        }
    }
}

impl SSML {
    pub fn lang(self, lang: Locale) -> Self {
        Self {
            speak: Speak::from(lang),
            ..self
        }
    }

    pub fn speak(self, speak: Speak) -> Self {
        Self { speak, ..self }
    }
}

impl Into<String> for SSML {
    fn into(self) -> String {
        let speak: VoiceTagInternal = self.speak.into();
        speak.into()
    }
}

impl ToString for SSML {
    fn to_string(&self) -> String {
        Into::<String>::into(self.clone())
    }
}
