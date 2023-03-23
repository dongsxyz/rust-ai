//! 
//! # OpenAI capabilities
//! 
//! For text/code completion related tasks, OpenAI suggests to make use of 
//! `gpt-3.5-turbo` model for a better performance at lower cost.
//! 
//! Image capabilites make use of DALL·E model and you may not be able to 
//! control model selection.

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