# API Request Structure

Previously, the primary `Request` type for LLM API calls was a `struct Request` defined in `crates/api/src/request.rs`. This struct contained comprehensive configuration for LLM interactions.

## Original `crates/api/src/request.rs` (Struct Definition)

```rust
#[derive(Clone, Debug)]
pub struct Request {
    pub messages: Vec<Message>,
    pub model: Model,
    pub temperature: Option<f6    pub frequency_penalty: Option<f64>,
    pub max_tokens: Option<usize>,
    /// milliseconds
    pub timeout: Option<u64>,
    /// It tries 1 + max_retry times.
    pub max_retry: usize,
    /// milliseconds
    pub sleep_between_retries: u64,
    pub dump_api_usage_at: Option<AuditRecordAt>,
    /// It dumps the AI conversation in pdl format. See <https://crates.io/crates/ragit-pdl> to read about pdl.
    pub dump_pdl_at: Option<String>,
    /// It's a directory, not a file. If given, it dumps `dir/request-<timestamp>.json` and `dir/response-<timestamp>.json`.
    pub dump_json_at: Option<String>,
    /// It can force LLMs to create a json output with a given schema.
    /// You have to call `send_and_validate` instead of `send` if you want
    /// to force the schema.
    pub schema: Option<Schema>,
    /// If LLMs fail to generate a valid schema `schema_max_try` times,
    /// it returns a default value. If it's 0, it wouldn't call LLM at all!
    pub schema_max_try: usize,
}
```

## Current `crates/api/src/request/mod.rs` (Enum Definition)

The functionality of the original `struct Request` has been refactored into an `enum Request` defined in `crates/api/src/request/mod.rs`. This `enum` now encapsulates different types of requests, including a comprehensive `ChatRequest` variant for LLM interactions and other variants for internal worker communication.

```rust
use std::path::PathBuf;
use crate::prelude::{Message, Model, Schema};
use ragit_types::AuditRecordAt;

#[derive(Debug)]
pub enum Request {
    /// Represents a request to build chunks from a specified file.
    BuildChunks { file: PathBuf },
    /// Represents a signal to terminate a worker process.
    Kill,
    /// Represents a chat request to an LLM, including all necessary configuration.
    ChatRequest {
        /// A vector of messages exchanged in the chat.
        messages: Vec<Message>,
        /// The model to be used for the chat request.
        model: Model,
        /// Optional: Controls the randomness of the output.
        temperature: Option<f64>,
        /// Optional: Penalizes new tokens based on their frequency in the text so far.
        frequency_penalty: Option<f64>,
        /// Optional: The maximum number of tokens to generate in the chat completion.
        max_tokens: Option<usize>,
        /// Optional: The timeout duration for the request in milliseconds.
        timeout: Option<u64>,
        /// The maximum number of retries for the request in case of failure.
        max_retry: usize,
        /// The duration to sleep between retries in milliseconds.
        sleep_between_retries: u64,
        /// Optional: Specifies where to dump API usage audit records.
        dump_api_usage_at: Option<AuditRecordAt>,
        /// Optional: Specifies a path to dump the PDL (Prompt Definition Language) used.
        dump_pdl_at: Option<String>,
        /// Optional: Specifies a path to dump the raw JSON response.
        dump_json_at: Option<String>,
        /// Optional: The schema to validate the LLM's response against.
        schema: Option<Schema>,
        /// The maximum number of attempts to validate the response against the schema.
        schema_max_try: usize,
    },
}
```

### Key Features of `ChatRequest` Variant (Migrated from Original Struct):

All the fields from the original `struct Request` have been faithfully migrated to the `ChatRequest` variant of the new `enum Request`. These include:

-   **Message Handling and Model Selection**: `messages` and `model` fields.
-   **LLM Parameters**: `temperature`, `frequency_penalty`, and `max_tokens` for fine-tuning model behavior.
-   **Retry Logic and Timeout Settings**: `timeout`, `max_retry`, and `sleep_between_retries` ensure robust communication with LLMs.
-   **Debugging and Logging Capabilities**: `dump_api_usage_at`, `dump_pdl_at`, and `dump_json_at` provide extensive options for auditing and debugging API interactions.
-   **Schema Validation for Structured Responses**: `schema` and `schema_max_try` facilitate the generation and validation of structured outputs from LLMs.

### Other `Request` Enums for Internal Worker Communication:

In addition to the `ChatRequest` variant, the `enum Request` in `crates/api/src/request/mod.rs` also includes variants like `BuildChunks` and `Kill` for internal worker communication. These internal `Request` enums are typically simpler, focusing on specific tasks for inter-thread communication. Other `Request` enums exist in specific modules for similar purposes:

-   `crates/ragit-index-effects/src/request.rs`
-   `src/index/commands/archive/create/request.rs`
-   `src/index/commands/archive/request.rs`
-   `src/index/commands/build.rs`