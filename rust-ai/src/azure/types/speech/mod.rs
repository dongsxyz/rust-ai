//!
//! Type definitions for Azure Cognitive Service - Speech.
//!
//! This module re-exports some powerful and frequently used types for your
//! benefit.
//! 
//! Extra functions are defined in [`apis::speech`](crate::azure::apis::speech) module.

////////////////////////////////////////////////////////////////////////////////

pub mod diarization;
pub mod entity;
pub mod file;
pub mod filter;
pub mod health;
pub mod language;
pub mod mode;
pub mod transcription;

pub use file::PaginatedFiles;
pub use transcription::{Status, Transcription};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: Option<String>,
    #[serde(rename = "innerError")]
    pub inner_error: InnerError,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InnerError {
    pub code: String,
    pub message: Option<String>,
}
