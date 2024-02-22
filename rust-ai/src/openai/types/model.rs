//!
//! Note: all contents copied from OpenAI documentation on November 7, 2023.
//!
//! Source: <https://platform.openai.com/docs/models/overview>
//!
//! # Models
//!
//! | MODELS       | DESCRIPTION |
//! | :----------- | :---------- |
//! | GPT-4        | A set of models that improve on GPT-3.5 and can understand as well as generate natural language or code |
//! | GPT-3.5      | A set of models that improve on GPT-3 and can understand as well as generate natural language or code |
//! | DALL·E       | A model that can generate and edit images given a natural language prompt |
//! | Whisper      | A model that can convert audio into text |
//! | Embeddings   | A set of models that can convert text into a numerical form |
//! | CodexLimited | A set of models that can understand and generate code, including translating natural language to code |
//! | Moderation   | A fine-tuned model that can detect whether text may be sensitive or unsafe |
//! | GPT-3	       | A set of models that can understand and generate natural language |
//! | TTS          | A set of models that can convert text into natural sounding spoken audio |

////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

/// An enum of OpenAI models
///
/// Note: GPT-4 are not publicly availble yet (Mar 22, 2023).
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Model {
    /// **GPT-4 Turbo**
    /// 
    /// The latest GPT-4 model intended to reduce cases of “laziness” where the
    /// model doesn’t complete a task. Returns a maximum of 4,096 output 
    /// tokens. [Learn more](https://openai.com/blog/new-embedding-models-and-api-updates).
    ///
    /// | MAX TOKENS    | TRAINING DATA  | SERIES |
    /// | :------------ | :------------- | :----- |
    /// | 128000 tokens | Up to Dec 2023 | GPT-4  |
    #[serde(rename = "gpt-4-0125-preview")]
    GPT_4_0125_PREVIEW,

    /// Currently points to gpt-4-0125-preview.
    ///
    /// | MAX TOKENS    | TRAINING DATA  | SERIES |
    /// | :------------ | :------------- | :----- |
    /// | 128000 tokens | Up to Dec 2023 | GPT-4  |
    #[serde(rename = "gpt-4-turbo-preview")]
    GPT_4_TURBO_PREVIEW,

    /// GPT-4 Turbo model featuring improved instruction following, JSON mode,
    /// reproducible outputs, parallel function calling, and more. Returns a 
    /// maximum of 4,096 output tokens. This is a preview model. [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday).
    ///
    /// | MAX TOKENS    | TRAINING DATA  | SERIES |
    /// | :------------ | :------------- | :----- |
    /// | 128000 tokens | Up to Apr 2023 | GPT-4  |
    #[serde(rename = "gpt-4-1106-preview")]
    GPT_4_1106_PREVIEW,

    /// Ability to understand images, in addition to all other GPT-4 Turbo
    /// capabilties. Returns a maximum of 4,096 output tokens. This is a
    /// preview model version and not suited yet for production traffic.
    /// [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday).
    ///
    /// | MAX TOKENS    | TRAINING DATA  | SERIES |
    /// | :------------ | :------------- | :----- |
    /// | 128000 tokens | Up to Apr 2023 | GPT-4  |
    #[serde(rename = "gpt-4-vision-preview")]
    GPT_4_TURBO_WITH_VISION,

    /// GPT-4 with the ability to understand images, in addition to all other
    /// GPT-4 Turbo capabilities. Returns a maximum of 4,096 output tokens. 
    /// This is a preview model version. [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday).
    ///
    /// | MAX TOKENS    | TRAINING DATA  | SERIES |
    /// | :------------ | :------------- | :----- |
    /// | 128000 tokens | Up to Apr 2023 | GPT-4  |
    #[serde(rename = "gpt-4-1106-vision-preview")]
    GPT_4_TURBO_1106_WITH_VISION,

    /// More capable than any GPT-3.5 model, able to do more complex tasks, and
    /// optimized for chat. Will be updated with our latest model iteration.
    ///
    /// Note: on June 27th, 2023, `gpt-4` will be updated to point from
    /// `gpt-4-0314` to `gpt-4-0613`, the latest model iteration.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 8192 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4")]
    GPT_4,

    /// Snapshot of `gpt-4` from March 14th 2023. Unlike `gpt-4`, this model
    /// will not receive updates, and will only be supported for a three month
    /// period ending on June 13th 2023.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 8192 tokens | Up to Sep 2021 | GPT-4  |
    #[deprecated(note = "Discontinuation date 2024-06-13, use `gpt-4-0613` instead")]
    #[serde(rename = "gpt-4-0314")]
    GPT_4_0314,

    /// Snapshot of `gpt-4` from June 13th 2023 with function calling data.
    /// Unlike `gpt-4`, this model will not receive updates, and will be
    /// deprecated 3 months after a new version is released.
    ///
    /// | MAX TOKENS   | TRAINING DATA  | SERIES |
    /// | :----------- | :------------- | :----- |
    /// | 8,192 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4-0613")]
    GPT_4_0613,

    /// Same capabilities as the base `gpt-4` mode but with 4x the context
    /// length. Will be updated with our latest model iteration.
    ///
    /// | MAX TOKENS   | TRAINING DATA  | SERIES |
    /// | :----------- | :------------- | :----- |
    /// | 32768 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4-32k")]
    GPT_4_32K,

    /// Snapshot of `gpt-4-32` from June 13th 2023. Unlike `gpt-4-32k`, this
    /// model will not receive updates, and will be deprecated 3 months after a
    /// new version is released.
    ///
    /// | MAX TOKENS    | TRAINING DATA  | SERIES |
    /// | :------------ | :------------- | :----- |
    /// | 32,768 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4-0613")]
    GPT_4_32K_0613,

    /// Snapshot of `gpt-4-32` from March 14th 2023. Unlike `gpt-4-32k`, this
    /// model will not receive updates, and will be deprecated 3 months after a
    /// new version is released.
    ///
    /// | MAX TOKENS   | TRAINING DATA  | SERIES |
    /// | :----------- | :------------- | :----- |
    /// | 32768 tokens | Up to Sep 2021 | GPT-4  |
    #[deprecated(note = "Discontinuation date 2023-09-13, use `gpt-4-32k-0613` instead")]
    #[serde(rename = "gpt-4-32k-0314")]
    GPT_4_32K_0314,

    /// **Updated GPT 3.5 Turbo**
    /// 
    /// The latest GPT-3.5 Turbo model with higher accuracy at responding in 
    /// requested formats and a fix for a bug which caused a text encoding 
    /// issue for non-English language function calls. Returns a maximum of 
    /// 4,096 output tokens. [Learn more](https://openai.com/blog/new-embedding-models-and-api-updates#:~:text=Other%20new%20models%20and%20lower%20pricing)
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 16,385 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "gpt-3.5-turbo-0125")]
    GPT_3_5_TURBO_0125,

    /// Most capable GPT-3.5 model and optimized for chat at 1/10th the cost of
    /// `text-davinci-003`. Will be updated with our latest model iteration.
    ///
    /// Note: on June 27th, 2023, `gpt-3.5-turbo` will be updated to point from
    /// `gpt-3.5-turbo-0301` to `gpt-3.5-turbo-0613`.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 4096 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "gpt-3.5-turbo")]
    GPT_3_5_TURBO,

    /// The latest GPT-3.5 Turbo model with improved instruction following,
    /// JSON mode, reproducible outputs, parallel function calling, and more.
    ///  Returns a maximum of 4,096 output tokens. [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday).
    ///
    /// | MAX TOKENS   | TRAINING DATA  | SERIES  |
    /// | :----------- | :------------- | :------ |
    /// | 16385 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "gpt-3.5-turbo-1106")]
    GPT_3_5_TURBO_1106,

    /// Same capabilities as the standard `gpt-3.5-turbo` model but with 4
    /// times the context.
    ///
    /// Note: currently points to gpt-3.5-turbo-16k-0613
    /// | MAX TOKENS    | TRAINING DATA  | SERIES  |
    /// | :------------ | :------------- | :------ |
    /// | 16,385 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "gpt-3.5-turbo-16k")]
    GPT_3_5_TURBO_16K,

    /// Snapshot of `gpt-3.5-turbo-16k` from June 13th 2023. Unlike
    /// `gpt-3.5-turbo-16k`, this model will not receive updates, and will be
    /// deprecated 3 months after a new version is released.
    /// 
    /// Note: Will be deprecated on June 13, 2024.
    ///
    /// | MAX TOKENS    | TRAINING DATA  | SERIES  |
    /// | :------------ | :------------- | :------ |
    /// | 16,385 tokens | Up to Sep 2021 | GPT-3.5 |
    #[deprecated(note = "Discontinuation date 2024-06-13")]
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    GPT_3_5_TURBO_16K_0613,

    /// Snapshot of `gpt-3.5-turbo` from June 13th 2023 with function calling
    /// data. Unlike `gpt-3.5-turbo`, this model will not receive updates, and
    /// will be deprecated 3 months after a new version is released.
    /// 
    /// Note: Will be deprecated on June 13, 2024.
    ///
    /// | MAX TOKENS   | TRAINING DATA  | SERIES  |
    /// | :----------- | :------------- | :------ |
    /// | 4,096 tokens | Up to Sep 2021 | GPT-3.5 |
    #[deprecated(note = "Discontinuation date 2024-06-13")]
    #[serde(rename = "gpt-3.5-turbo-0613")]
    GPT_3_5_TURBO_0613,

    /// Snapshot of `gpt-3.5-turbo` from March 1st 2023. Unlike
    /// `gpt-3.5-turbo`, this model will not receive updates, and will only be
    /// supported for a three month period ending on June 1st 2023.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 4096 tokens | Up to Sep 2021 | GPT-3.5 |
    #[deprecated(note = "Discontinuation date 2023-09-13, use `gpt-3.5-turbo-0613` instead")]
    #[serde(rename = "gpt-3.5-turbo-0301")]
    GPT_3_5_TURBO_0301,

    /// Can do any language task with better quality, longer output, and
    /// consistent instruction-following than the curie, babbage, or ada
    /// models. Also supports some additional features such as [inserting text](https://platform.openai.com/docs/guides/gpt/inserting-text).
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 4097 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "text-davinci-003")]
    TEXT_DAVINCI_003,

    /// Similar capabilities to `text-davinci-003` but trained with supervised
    /// fine-tuning instead of reinforcement learning
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 4097 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "text-davinci-002")]
    TEXT_DAVINCI_002,
    #[serde(rename = "text-davinci-edit-001")]
    TEXT_DAVINCI_EDIT_001,

    /// Optimized for code-completion tasks
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 8001 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "code-davinci-edit-001")]
    CODE_DAVINCI_EDIT_001,

    #[serde(rename = "whisper-1")]
    WHISPER_1,

    #[serde(rename = "text-embedding-ada-002")]
    TEXT_EMBEDDING_ADA_002,

    #[serde(rename = "text-embedding-ada-002-v2")]
    TEXT_EMBEDDING_ADA_002_v2,

    #[serde(rename = "text-search-ada-doc-001")]
    TEXT_SEARCH_ADA_DOC_001,

    /// Most capable Codex model. Particularly good at translating natural
    /// language to code. In addition to completing code, also supports
    /// [inserting](https://platform.openai.com/docs/guides/code/inserting-code) completions within code.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 8001 tokens | Up to Jun 2021 | Codex  |
    #[deprecated(note = "The Codex models are now deprecated.")]
    #[serde(rename = "code-davinci-002")]
    CODE_DAVINCI_002,

    /// Earlier version of `code-davinci-002`
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 8001 tokens | Up to Jun 2021 | Codex  |
    #[deprecated(note = "The Codex models are now deprecated.")]
    #[serde(rename = "code-davinci-001")]
    CODE_DAVINCI_001,

    /// Almost as capable as Davinci Codex, but slightly faster. This speed
    /// advantage may make it preferable for real-time applications.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2048 tokens | -              | Codex  |
    #[deprecated(note = "The Codex models are now deprecated.")]
    #[serde(rename = "code-cushman-002")]
    CODE_CUSHMAN_002,

    /// Earlier version of `code-cushman-002`
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2048 tokens | -              | Codex  |
    #[deprecated(note = "The Codex models are now deprecated.")]
    #[serde(rename = "code-cushman-001")]
    CODE_CUSHMAN_001,

    /// Most capable moderation model. Accuracy will be slighlty higher than
    /// the stable model
    ///
    /// Series: Moderation
    #[serde(rename = "text-moderation-latest")]
    TEXT_MODERATION_LATEST,
    #[serde(rename = "text-moderation-004")]
    TEXT_MODERATION_004,
    #[serde(rename = "text-moderation-003")]
    TEXT_MODERATION_003,
    #[serde(rename = "text-moderation-002")]
    TEXT_MODERATION_002,
    #[serde(rename = "text-moderation-001")]
    TEXT_MODERATION_001,

    /// Almost as capable as the latest model, but slightly older.
    ///
    /// Series: Moderation
    #[serde(rename = "text-moderation-stable")]
    TEXT_MODERATION_STABLE,

    /// Very capable, faster and lower cost than Davinci.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2049 tokens | Up to Oct 2019 | GPT-3  |
    #[serde(rename = "text-curie-001")]
    TEXT_CURIE_001,

    /// Capable of straightforward tasks, very fast, and lower cost.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2049 tokens | Up to Oct 2019 | GPT-3  |
    #[serde(rename = "text-babbage-001")]
    TEXT_BABBAGE_001,

    /// Capable of very simple tasks, usually the fastest model in the GPT-3
    /// series, and lowest cost.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2049 tokens | Up to Oct 2019 | GPT-3  |
    #[serde(rename = "text-ada-001")]
    TEXT_ADA_001,

    /// Most capable GPT-3 model. Can do any task the other models can do,
    /// often with higher quality.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2049 tokens | Up to Oct 2019 | GPT-3  |
    #[serde(rename = "davinci")]
    DAVINCI,

    /// Very capable, but faster and lower cost than Davinci.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2049 tokens | Up to Oct 2019 | GPT-3  |
    #[serde(rename = "curie")]
    CURIE,

    /// Capable of straightforward tasks, very fast, and lower cost.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2049 tokens | Up to Oct 2019 | GPT-3  |
    #[serde(rename = "babbage")]
    BABBAGE,

    /// Capable of very simple tasks, usually the fastest model in the GPT-3 series, and lowest cost.
    ///
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2049 tokens | Up to Oct 2019 | GPT-3  |
    #[serde(rename = "ada")]
    ADA,

    /// The latest DALL·E model released in Nov 2023. [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday).
    #[serde(rename = "dall-e-3")]
    DALL_E_3,

    /// The previous DALL·E model released in Nov 2022. The 2nd iteration of
    /// DALL·E with more realistic, accurate, and 4x greater resolution images
    /// than the original model.
    #[serde(rename = "dall-e-2")]
    DALL_E_2,

    /// The latest text to speech model, optimized for speed.
    #[serde(rename = "tts-1")]
    TTS_1,

    /// The latest text to speech model, optimized for quality.
    #[serde(rename = "tts-1-hd")]
    TTS_1_HD,

    #[serde(other)]
    UNKNOWN,
}

#[allow(deprecated)]
impl Into<&'static str> for Model {
    fn into(self) -> &'static str {
        match self {
            Model::GPT_4 => "gpt-4",
            Model::GPT_4_0613 => "gpt-4-0613",
            Model::GPT_4_0314 => "gpt-4-0314",
            Model::GPT_4_32K => "gpt-4-32k",
            Model::GPT_4_32K_0613 => "gpt-4-32k-0613",
            Model::GPT_4_32K_0314 => "gpt-4-32k-0314",
            Model::GPT_3_5_TURBO => "gpt-3.5-turbo",
            Model::GPT_3_5_TURBO_16K => "gpt-3.5-turbo-16k",
            Model::GPT_3_5_TURBO_16K_0613 => "gpt-3.5-turbo-16k-0613",
            Model::GPT_3_5_TURBO_0613 => "gpt-3.5-turbo-0613",
            Model::GPT_3_5_TURBO_0301 => "gpt-3.5-turbo-0301",
            Model::TEXT_DAVINCI_003 => "text-davinci-003",
            Model::TEXT_DAVINCI_002 => "text-davinci-002",
            Model::TEXT_DAVINCI_EDIT_001 => "text-davinci-edit-001",
            Model::CODE_DAVINCI_EDIT_001 => "code-davinci-edit-001",
            Model::WHISPER_1 => "whisper-1",
            Model::TEXT_EMBEDDING_ADA_002 => "text-embedding-ada-002",
            Model::TEXT_EMBEDDING_ADA_002_v2 => "text-embedding-ada-002-v2",
            Model::TEXT_SEARCH_ADA_DOC_001 => "text-search-ada-doc-001",
            Model::CODE_DAVINCI_002 => "code-davinci-002",
            Model::CODE_DAVINCI_001 => "code-davinci-001",
            Model::CODE_CUSHMAN_002 => "code-cushman-002",
            Model::CODE_CUSHMAN_001 => "code-cushman-001",
            Model::TEXT_MODERATION_LATEST => "text-moderation-latest",
            Model::TEXT_MODERATION_STABLE => "text-moderation-stable",
            Model::TEXT_CURIE_001 => "text-curie-001",
            Model::TEXT_BABBAGE_001 => "text-babbage-001",
            Model::TEXT_ADA_001 => "text-ada-001",
            Model::DAVINCI => "davinci",
            Model::CURIE => "curie",
            Model::BABBAGE => "babbage",
            Model::ADA => "ada",
            Model::TEXT_MODERATION_001 => "text-moderation-001",
            Model::TEXT_MODERATION_002 => "text-moderation-002",
            Model::TEXT_MODERATION_003 => "text-moderation-003",
            Model::TEXT_MODERATION_004 => "text-moderation-004",
            Model::UNKNOWN => "unknown",
            Model::GPT_4_TURBO_WITH_VISION => "gpt-4-vision-preview",
            Model::GPT_3_5_TURBO_1106 => "gpt-3.5-turbo-1106",
            Model::DALL_E_3 => "dall-e-3",
            Model::DALL_E_2 => "dall-e-2",
            Model::TTS_1 => "tts-1",
            Model::TTS_1_HD => "tts-1-hd",
            Model::GPT_4_0125_PREVIEW => "gpt-4-0125-preview",
            Model::GPT_4_TURBO_PREVIEW => "gpt-4-turbo-preview",
            Model::GPT_4_1106_PREVIEW => "gpt-4-1106-preview",
            Model::GPT_4_TURBO_1106_WITH_VISION => "gpt-4-1106-vision-preview",
            Model::GPT_3_5_TURBO_0125 => "gpt-3.5-turbo-0125",
        }
    }
}
