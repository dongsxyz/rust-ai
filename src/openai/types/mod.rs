//! 
//! # Type collection of OpenAI APIs
//! 
//! You may find the `Model` struct mostly used. Types ending `*Response` are 
//! wrapped return values from OpenAI endpoints.

////////////////////////////////////////////////////////////////////////////////

/// Common types for OpenAI
pub mod common;

/// OpenAI models
pub mod model;

/// OpenAI Chat Completions
pub mod chat_completion;

/// OpenAI Completions
pub mod completion;

/// OpenAI Edits
pub mod edit;

/// OpenAI Images
pub mod image;

/// OpenAI Embeddings
pub mod embedding;

/// OpenAI Audios
pub mod audio;

/// OpenAI Moderations
pub mod moderation;