# AI in Rust

Provide a collection of third-party APIs that give public access to AI capabilities.

**WARNING**: this crate is **NOT PRODUCTION** ready.

## Support List

### OpenAI

| Category         | Variant        | Tested Models            | Stream |
| :--------------- | :------------- | :----------------------- | :----: |
| Chat completions | -              | `gpt-3.5-turbo`, `gpt-3.5-turbo-0301`, `text-davinci-003`, `text-davinci-002`, `code-davinci-002`[^note_3] | yes    |
| Completions      | -              | `gpt-3.5-turbo`, `gpt-3.5-turbo-0301`, `text-davinci-003`       | yes    |
| Edits            | -              | `text-davinci-edit-001`, `code-davinci-edit-001`  | -      |
| Images           | Generations    | `dall-e`[^note_1]        | -      |
| Images           | Edits          | `dall-e`[^note_1]        | -      |
| Images           | Variations     | `dall-e`[^note_1]        | -      |
| Embeddings       | -              | `text-embedding-ada-002`[^note_4] | -      |
| Audios           | Transcriptions | `whisper-1`              | -      |
| Audios           | Translation    | `whisper-1`              | -      |
| Moderation       | -              | `text-moderation-latest`[^note_2], `text-moderation-stable` | -      |

Note:
- Updated on March 24rd, 2023
- OpenAI's Fine Tunes endpoints are currently not supported.

### Azure

| Category          | Capability | Endpoint       |
| :---------------- | :--------- | :------------- |
| Cognitive service | Speech     | Text-to-Speech, Voice-List, Speech-to-Text (Batch) |

Note: 
- Azure CN is not supported by this repo yet.

## Usage

Create a `config.yml` in your working directory, or root of this crate. Contents like this:

```yaml
openai:
  api_key: sk-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
azure:
  speech:
    key: 4c7eXXXXXXXXXXXXXXXXXXXXXXX54c32
    region: westus
```

[^note_1]: `dall-e` is an hypothetical name of the unknown model. Currently, OpenAI doesn't support manually specify model for image related tasks. So DALL·E models are not valid variants of `Model` enum.

[^note_2]: OpenAI's responses for moderation indicate usage of `text-moderation-004` model (March 23rd, 2023). But developers cannot use its API endpoints to specify variants other than `text-moderation-latest` and `text-moderation-stable`.

[^note_3]: GPT-4 series cannot be tested due to author of this crate is still in the waitlist.

[^note_4]: You may notice the actual model behind Embedding API to be `text-embedding-ada-002-v2` (March 23rd, 2023).