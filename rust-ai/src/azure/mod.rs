//!
//! # Azure
//!
//! | Category          | Capability | Endpoint       |
//! | :---------------- | :--------- | :------------- |
//! | Cognitive service | Speech     | Text-to-Speech, Voice-List |
//!
//! Note:
//! - Azure CN is not supported by this repo yet.

////////////////////////////////////////////////////////////////////////////////

/// Azure global endpoints
pub mod endpoint;

/// Azure services
pub mod apis;

/// Azure types definition
pub mod types;

pub use apis::speech::{Speech, SpeechModel};
pub use types::ssml;
pub use types::Gender;
pub use types::Locale;
pub use types::MicrosoftOutputFormat;
pub use types::VoiceName;
pub use types::SSML;
