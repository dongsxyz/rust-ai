//!
//! A module for manipulating additional HTTP headers especially when
//! communicating with proxy servers.

////////////////////////////////////////////////////////////////////////////////

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

const ADDITIONAL_HEADERS_VAR: &'static str = "RUST_AI_ADDITIONAL_HEADERS";

/// A type alias for (String, String), which denotes HTTP header/value pair.
type RawHeader<'a> = (&'a str, &'a str);
type Header = (String, String);

/// This is the only interface to set/get runtime level headers when sending
/// requests to endpoints such as OpenAI.
///
/// # Usage
/// 1. Call `AdditionalHeaders::default()` to initialize an instance.
/// 2. Call `set_header()` to set required headers.
/// 3. Depends on the caller:
///   - If you are using Rust-AI as an library, call `to_var()` to set related
///     environment variable.
///   - If you are calling from Rust-AI itself, you should call `provide()` to
///     turn current instance into [`HeaderMap`].
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AdditionalHeaders {
    headers: Vec<Header>,
}

impl Default for AdditionalHeaders {
    /// Create a new instance of [`AdditionalHeaders`] that doesn't contain any
    /// headers.
    fn default() -> Self {
        Self { headers: vec![] }
    }
}

impl AdditionalHeaders {
    /// Try to initialize [`AdditionalHeaders`] instance from environment
    /// variable. If no such environment variables available, then create an
    /// empty instance.
    pub fn from_var() -> Self {
        if let Ok(c) = std::env::var(ADDITIONAL_HEADERS_VAR) {
            if let Ok(deserialized) = serde_json::from_str::<Self>(&c) {
                return deserialized;
            }
        }
        Default::default()
    }
}

impl AdditionalHeaders {
    /// Set a new header pair. Header name and values should never include NUL
    /// characters (`\0`).
    pub fn set_header<'a>(&mut self, header: RawHeader<'a>) {
        let header_name = header.0.to_string();
        let header_value = header.1.to_string();
        if header_name.contains("\0") || header_value.contains("\0") {
            panic!("`\0` cannot present in any field of the header");
        }
        self.headers.push((header_name, header_value));
    }

    /// Turn the contained headers into [`HeaderMap`]. Will not consume current
    /// instance.
    pub fn provide(&self) -> HeaderMap {
        let mut hm = HeaderMap::new();
        self.headers.iter().for_each(|h| {
            hm.append(
                HeaderName::from_lowercase(h.0.to_lowercase().as_bytes()).unwrap(),
                HeaderValue::from_str(&h.1).unwrap(),
            );
        });

        hm
    }

    /// Try to serialize current instance into string form and set to specific
    /// environment variable.
    pub fn to_var(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        std::env::set_var(ADDITIONAL_HEADERS_VAR, serialized);
    }
}
