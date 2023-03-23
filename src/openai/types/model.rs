//! 
//! Note: all contents copied from OpenAI documentation on Mar 22, 2023.
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

////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};


/// An enum of OpenAI models
///
/// Note: GPT-4 are not publicly availble yet (Mar 22, 2023).
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum Model {
    /// More capable than any GPT-3.5 model, able to do more complex tasks, and 
    /// optimized for chat. Will be updated with our latest model iteration.
    /// 
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 8192 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4")]
    GPT_4,

    /// Snapshot of `gpt-4` from March 14th 2023. Unlike `gpt-4`, this model 
    /// will not receive updates, and will only be supported for a three month 
    /// period ending on June 14th 2023.
    /// 
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 8192 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4-0314")]
    GPT_4_0314,
    
    /// Same capabilities as the base `gpt-4` mode but with 4x the context 
    /// length. Will be updated with our latest model iteration.
    /// 
    /// | MAX TOKENS   | TRAINING DATA  | SERIES |
    /// | :----------- | :------------- | :----- |
    /// | 32768 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4-32k")]
    GPT_4_32K,
    
    /// Snapshot of `gpt-4-32` from March 14th 2023. Unlike `gpt-4-32k`, this 
    /// model will not receive updates, and will only be supported for a three 
    /// month period ending on June 14th 2023.
    /// 
    /// | MAX TOKENS   | TRAINING DATA  | SERIES |
    /// | :----------- | :------------- | :----- |
    /// | 32768 tokens | Up to Sep 2021 | GPT-4  |
    #[serde(rename = "gpt-4-32k-0314")]
    GPT_4_32K_0314,

    /// Most capable GPT-3.5 model and optimized for chat at 1/10th the cost of 
    /// `text-davinci-003`. Will be updated with our latest model iteration.
    /// 
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 4096 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "gpt-3.5-turbo")]
    GPT_3_5_TURBO,

    /// Snapshot of `gpt-3.5-turbo` from March 1st 2023. Unlike 
    /// `gpt-3.5-turbo`, this model will not receive updates, and will only be 
    /// supported for a three month period ending on June 1st 2023.
    /// 
    /// | MAX TOKENS  | TRAINING DATA  | SERIES  |
    /// | :---------- | :------------- | :------ |
    /// | 4096 tokens | Up to Sep 2021 | GPT-3.5 |
    #[serde(rename = "gpt-3.5-turbo-0301")]
    GPT_3_5_TURBO_0301,

    /// Can do any language task with better quality, longer output, and 
    /// consistent instruction-following than the curie, babbage, or ada models.
    ///  Also supports [inserting](https://platform.openai.com/docs/guides/completion/inserting-text) completions within text.
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
    #[serde(rename = "code-davinci-002")]
    CODE_DAVINCI_002,

    /// Almost as capable as Davinci Codex, but slightly faster. This speed 
    /// advantage may make it preferable for real-time applications.
    /// 
    /// | MAX TOKENS  | TRAINING DATA  | SERIES |
    /// | :---------- | :------------- | :----- |
    /// | 2048 tokens | -              | Codex  |
    #[serde(rename = "code-cushman-001")]
    CODE_CUSHMAN_001,

    /// Most capable moderation model. Accuracy will be slighlty higher than 
    /// the stable model
    /// 
    /// Series: Moderation
    #[serde(rename = "text-moderation-latest")]
    TEXT_MODERATION_LATEST,

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
}
