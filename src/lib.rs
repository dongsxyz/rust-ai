//! 
//! # Rust AI API Collection
//! *See documents of each module*
//! 
//! ## Support List
//! 
//! ### OpenAI
//! 
//! | Category         | Variant        | Tested Models            | Stream |
//! | :--------------- | :------------- | :----------------------- | :----: |
//! | Chat completions | -              | `gpt-3.5-turbo`, `gpt-3.5-turbo-0301`, `text-davinci-003`, `text-davinci-002`, `code-davinci-002`[^note_3] | yes    |ext-davinci-002`, `code-davinci-002` | yes    |
//! | Completions      | -              | `gpt-3.5-turbo`, `gpt-3.5-turbo-0301`, `text-davinci-003`       | yes    |   | yes    |
//! | Edits            | -              | `text-davinci-edit-001`, `code-davinci-edit-001`  | -      |
//! | Images           | Generations    | `dall-e`[^note_1]        | -      |
//! | Images           | Edits          | `dall-e`[^note_1]        | -      |
//! | Images           | Variations     | `dall-e`[^note_1]        | -      |
//! | Embeddings       | -              | `text-embedding-ada-002`[^note_4] | -      |
//! | Audios           | Transcriptions | `whisper-1`              | -      |
//! | Audios           | Translation    | `whisper-1`              | -      |
//! | Moderation       | -              | `text-moderation-latest`[^note_2], `text-moderation-stable` | -      |
//! 
//! Note: 
//! - Updated on March 23rd, 2023.
//! - OpenAI's Fine Tunes endpoints are currently not supported.
//! 
//! [^note_1]: `dall-e` is an hypothetical name of the unknown model. Currently, 
//! OpenAI doesn't support manually specify model for image related tasks. So 
//! DALL·E models are not valid variants of `Model` enum.
//! 
//! [^note_2]: OpenAI's responses for moderation indicate usage of 
//! `text-moderation-004` model (March 23rd, 2023). But developers cannot use 
//! its API endpoints to specify variants other than `text-moderation-latest` 
//! and `text-moderation-stable`.
//! 
//! [^note_3]: GPT-4 series cannot be tested due to author of this crate is 
//! still in the waitlist.
//! 
//! [^note_4]: You may notice the actual model behind Embedding API to be 
//! `text-embedding-ada-002-v2` (March 23rd, 2023).

////////////////////////////////////////////////////////////////////////////////

/// OpenAI endpoint collection
pub mod openai;

/// Azure endpoint collection
pub mod azure;

/// Support functions
pub mod utils;