//! 
//! Azure APIs in Rust like structs.
//! 
//! This is not just a simple wrapping binding of Azure's services, you can 
//! chain parameters/modifiers with sequential method calls.
//! 
//! # Example 
//! 
//! ```rust, ignore
//! use rust_ai::azure::{ssml::Speak, Locale, Speech, VoiceName, SSML};
//! 
//! #[tokio::main]
//! async fn main() -> tokio::io::Result<()> {
//!   let ssml = SSML::from(
//!     Speak::voice_content(
//!       VoiceName::zh_CN_YunhaoNeural,
//!       "亲爱的朋友，美丽中国欢迎你！",
//!     )
//!     .lang(Locale::zh_CN),
//!   );
//!   
//!   let result = Speech::from(ssml).tts().await.unwrap();
//!   
//!   std::fs::write(std::path::PathBuf::from(r"D:\Contents\Downloads\output.mp3"), result).unwrap();
//!   Ok(())
//! }
//! ```

////////////////////////////////////////////////////////////////////////////////

/// Speech capabilities from Azure Cognitive service
pub mod speech; 