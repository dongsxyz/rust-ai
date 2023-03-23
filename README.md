# AI in Rust

Provide a collection of third-party APIs that give public access to AI capabilities.

**WARNING**: this crate is **NOT PRODUCTION** ready.

## Support List

### OpenAI

Note: *Updated on March 23rd, 2023*.

| Category         | Variant        | Tested Models            | Stream |
| :--------------- | :------------- | :----------------------- | :----: |
| Chat completions | -              | `gpt-3.5-turbo`          | yes    |
| Completions      | -              | `text-davinci-003`       | yes    |
| Edits            | -              | `text-davinci-edit-001`  | -      |
| Images           | Generations    | -                        | -      |
| Images           | Edits          | -                        | -      |
| Images           | Variations     | -                        | -      |
| Embeddings       | -              | `text-embedding-ada-002` | -      |
| Audios           | Transcriptions | -                        | -      |
| Audios           | Translation    | -                        | -      |

## Usage

Create a `config.yml` in your working directory, or root of this crate. Contents like this:

```yaml
openai:
  api_key: sk-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

And run examples, e.g.:

```bash
cargo run --example chat-completion
```