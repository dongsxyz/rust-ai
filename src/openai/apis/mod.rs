//! 
//! OpenAI capabilities

////////////////////////////////////////////////////////////////////////////////
 
/// Chat completion using GPT-3.5 (ChatGPT) and GPT-4
pub mod chat_completion;

/// Propmpt completion using GPT-3 series
pub mod completion;

/// Text and code edits
pub mod edit;

/// Image generation and manipulation
pub mod image;

/// Embedding and extraction
pub mod embedding;

/// Audio transcription and translation
pub mod audio;

/// Moderation endpoint
pub mod moderation;