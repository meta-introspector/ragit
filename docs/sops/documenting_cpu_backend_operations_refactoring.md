# SOP: Documenting CPU Backend Operations Refactoring

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the process for documenting the refactoring of CPU backend operations in `candle-core` into individual files, adhering to the "one declaration per file" principle. It also specifies the required metadata header for each new file, ensuring consistent documentation and semantic tagging.

## 2. Scope
This SOP applies to all refactoring efforts within `vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/` and any similar directories where a large module is split into smaller, single-purpose files.

## 3. Procedure

### 3.1. Identify Operation for Refactoring
Select a specific CPU backend operation (e.g., `Affine`, `AvgPool2D`) from `candle-core/src/cpu_backend/mod.rs` that needs to be extracted into its own file.

### 3.2. Create New File
Create a new `.rs` file in the `vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/` directory (or the appropriate `ops` subdirectory) with a name corresponding to the operation (e.g., `affine.rs`, `avg_pool_2d.rs`).

### 3.3. Extract Code
Move the `struct` definition and its corresponding `impl Map1 for` (or `Map2`, `Map1Any`, etc.) block from `candle-core/src/cpu_backend/mod.rs` into the newly created file.

### 3.4. Add Metadata Header
At the top of each new file, add a metadata header following this format:

```
// term: <OperationName>
// count: 1
// int64: 0
// category: "CPU Backend Operation"
// significance: "<Brief description of the operation's purpose>"
// vibe: "<Conceptual vibe/theme of the operation>"
// action_suggestion: "None"
// emoji_representation: "<Relevant Emoji>"
// semantic_names: ["<semantic_name_1>", "<semantic_name_2>"]
// osi_layer: "Layer 1 - Physical"
// prime_factor: 17
// is_power_of_two: false
// numerical_address: <Unique Numerical ID>
// embedding_vectors: []
// versions: ["0.1.0"]
// first_seen_timestamp: "<YYYY-MM-DD>"
// last_seen_timestamp: "<YYYY-MM-DD>"
```

*   **`term`**: The name of the operation (e.g., `Affine`, `AvgPool2D`).
*   **`count`**: Always `1` for a single declaration.
*   **`int64`**: Always `0` for now.
*   **`category`**: "CPU Backend Operation".
*   **`significance`**: A concise explanation of what the operation does.
*   **`vibe`**: A conceptual theme or feeling associated with the operation (e.g., "Linear Transformation", "Downsampling").
*   **`action_suggestion`**: "None" for now.
*   **`emoji_representation`**: A single emoji that visually represents the operation.
*   **`semantic_names`**: A list of semantic names or keywords related to the operation.
*   **`osi_layer`**: "Layer 1 - Physical" for CPU backend operations.
*   **`prime_factor`**: `17` (as per the project's numerology for "Abstraction, Schema, Interface, Command Execution").
*   **`is_power_of_two`**: `false` for now.
*   **`numerical_address`**: A unique numerical ID for each operation, incrementing sequentially within the `ops` directory.
*   **`embedding_vectors`**: `[]` for now.
*   **`versions`**: `["0.1.0"]` for initial version.
*   **`first_seen_timestamp`**: The date the file was created.
*   **`last_seen_timestamp`**: The date the file was last modified.

### 3.5. Update `mod.rs`
In `candle-core/src/cpu_backend/mod.rs`, replace the extracted code with a `pub mod <operation_name>;` declaration (e.g., `pub mod affine;`, `pub mod avg_pool_2d;`).

### 3.6. Update Imports
Adjust `use` statements in the new file to correctly import necessary traits and modules (e.g., `use super::utils::{binary_map, unary_map, Map1};`).

### 3.7. Verification
*   Run `cargo build` to ensure no compilation errors.
*   Run relevant tests to verify the functionality of the refactored operation.

## 4. Tools
*   `read_file`
*   `write_file`
*   `replace`
*   `run_shell_command` (for `cargo build`, `cargo test`)
*   `git status`, `git diff`

## 5. Expected Outcome
*   CPU backend operations are modularized into single files.
*   Each operation file contains a consistent metadata header.
*   The codebase is more organized, readable, and maintainable.
*   The refactored code compiles and functions correctly.
