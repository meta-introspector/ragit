pub const BOOTSTRAP_PACKAGE_NAME: &str = "ragit-command-bootstrap";
pub const INDEX_FILE_NAME: &str = "index.json";
pub const PROMPTS_DIR_NAME: &str = "prompts";
pub const SELF_LIB_PATH: &str = "crates/layer7_application/commands/ragit-command-bootstrap/src/lib.rs";
pub const TEMP_DIR_NAME: &str = "tmp_bootstrap";
pub const RAGIT_DIR_NAME: &str = ".ragit";
pub const SELF_IMPROVEMENT_PROMPT_PREFIX: &str = "The following is the Rust source code for a function that bootstraps a RAGIT index and then attempts to improve itself. Please review the code and provide an improved version. The improved version should be more robust, efficient, and clear. The function should also contain a query to reflect on its own functionality. Only output the complete, raw rust code for the file, without any explanations or markdown formatting.\n\n```rust\n{}
```";
pub const LOG_SUCCESS_WRITE_IMPROVED_SELF: &str = "bootstrap_index_self: Successfully wrote improved self to {:?}";
pub const LOG_EMPTY_SELF_IMPROVEMENT_RESPONSE: &str = "bootstrap_index_self: Self-improvement returned empty response, skipping.";
pub const LOG_RUNNING_FINAL_REFLECTIVE_QUERY: &str = "bootstrap_index_self: Running final reflective query";
pub const LOG_BEFORE_FINAL_REFLECTIVE_QUERY: &str = "bootstrap_index_self: Before final reflective query";
pub const LOG_AFTER_FINAL_REFLECTIVE_QUERY: &str = "bootstrap_index_self: After final reflective query";
pub const FINAL_REFLECTIVE_QUERY_PROMPT: &str = "Explain the bootstrap_index_self function";
pub const IMPROVED_LIB_FILE_NAME: &str = "improved_lib.rs";