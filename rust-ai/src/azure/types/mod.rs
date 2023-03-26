pub mod common;
pub mod locale;
pub mod tts;
pub mod ssml;
pub mod speech;

pub use locale::Locale;
pub use ssml::voice_name::VoiceName;
pub use common::Gender;
pub use ssml::SSML;
pub use common::MicrosoftOutputFormat;