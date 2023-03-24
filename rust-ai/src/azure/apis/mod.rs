//! 
//! Azure APIs in Rust like structs.
//! 
//! This is not just a simple wrapping binding of Azure's services, you can 
//! chain parameters/modifiers with sequential method calls.
//! 
//! # Example 
//! 
//! ```rust
//! use rust_ai::azure::{MicrosoftOutputFormat, Speech, VoiceName};
//! 
//! let result = Speech::default()
//! .voice(VoiceName::zh_CN_YunfengNeural)
//! .format(MicrosoftOutputFormat::Audio_24khz_48kbitrate_Mono_Mp3)
//! .tts(r#"美丽中国，大好河山欢迎您！"#.into())
//! .await?;
//! 
//! std::fs::write(PathBuf::from(r"D:\Contents\Downloads\output.mp3"), result).unwrap();
//! ```

////////////////////////////////////////////////////////////////////////////////

/// Speech capabilities from Azure Cognitive service
pub mod speech; 