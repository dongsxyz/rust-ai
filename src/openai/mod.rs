//! 
//! # OpenAI
//! 
//! This module provides functionalities accessable from OpenAI's RESTful API 
//! endpoints.
//! 
//! You should have a valid API Key first. Put the key in `config.yml`, see
//! README.md for instance.
//! 
//! These types are re-exported from submodules:
//! - `Audio`: audio transcriptions and translations;
//! - `ChatCompletion`: ChatGPT like capabilities for text/code completion;
//! - `Completion`: GPT-3 based text/code completion;
//! - `Edit`: text content manipulation;
//! - `Embedding`: Ada based embedding extraction;
//! - `Moderation`: content violation detection;
//! - `Model`: an enum represents all available OpenAI public models.

////////////////////////////////////////////////////////////////////////////////

/// API endpoint definitions
pub mod endpoint;

/// Type definitions for OpenAI
pub mod types;

/// OpenAI capabilities
pub mod apis;

pub use apis::chat_completion::ChatCompletion;
pub use apis::completion::Completion;
pub use apis::edit::Edit;
pub use apis::image::Image;
pub use apis::embedding::Embedding;
pub use apis::audio::Audio;
pub use apis::moderation::Moderation;
pub use types::model::Model;