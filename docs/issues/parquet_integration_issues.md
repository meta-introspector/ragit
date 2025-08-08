# Parquet Integration Issues in `term_quiz_master`

## Date: 2025-08-07

## Objective:
Integrate Parquet file generation into the `term_quiz_master` crate using the `hf-dataset-validator-rust` vendored module.

## Issues Encountered:

### 1. Persistent Dependency Version Conflicts
**Problem:** Repeated compilation failures due to conflicting `chrono`, `arrow`, and `parquet` versions.
**Details:**
- `hf-dataset-validator-rust` initially required `chrono` versions between `0.4.34` and `0.4.40`.
- The main `ragit` workspace uses `chrono 0.4.41`.
- Attempts to update `hf-dataset-validator-rust`'s `Cargo.toml` to use `chrono = "0.4.41"` and `arrow = "56.0.0"`, `parquet = "56.0.0"` (versions that compiled successfully in isolation) did not fully resolve the issue.
- `cargo update` was used to synchronize `Cargo.lock`, but transitive dependencies continued to cause conflicts, particularly with `arrow-arith`'s `ChronoDateExt` and `Datelike` traits.

### 2. `hf-dataset-validator-rust` as a Library Crate
**Problem:** Initial attempts to use `hf-dataset-validator-rust` as a dependency failed because it was a binary crate, not a library.
**Resolution Attempt:**
- Modified `hf-dataset-validator-rust/Cargo.toml` to include a `[lib]` section and moved `main.rs` content to `lib.rs`.
- This introduced new compilation errors related to `hf_dataset_converter` functions not being found or having incorrect signatures.

### 3. `async`/`await` Integration and Data Flow
**Problem:** The `term_quiz_master`'s `run_quiz` function was synchronous, while `hf_dataset_validator`'s generation functions are `async`.
**Details:**
- Changed `run_quiz` to `async` and added `#[tokio::main]` to `main.rs`.
- The `create_huggingface_dataset` function in `hf_dataset_converter` was initially designed to read from a file system path, but the `term_quiz_master` had `AugmentedTerms` data in memory.
- Refactored `hf_dataset_converter` to accept `AugmentedTerms` directly, introducing `create_huggingface_dataset_from_augmented_terms`.
- This led to further compilation errors related to field access (`id`, `character_group`) and type mismatches (`usize` vs. `String` for `numerical_address`).

### 4. Refactoring Tool Limitations
**Problem:** Attempts to automate code modifications using a custom Rust refactoring tool (and `coccinelleforrust`) proved challenging.
**Details:**
- `coccinelleforrust` failed to parse the semantic patch, indicating strict syntax requirements or dialect differences.
- The custom Rust refactoring tool, while more flexible, also required significant debugging and precise AST manipulation, becoming a task in itself.

## Current Status:
Code changes related to Parquet integration are currently commented out in `crates/term_quiz_master/src/quiz_logic.rs` and `crates/term_quiz_master/src/main.rs` to allow the project to compile.

## Next Steps:
1.  Investigate deeper into `arrow` and `parquet` dependency trees to find a truly compatible version set with `chrono 0.4.41`.
2.  Re-evaluate the approach to integrating `hf-dataset-validator-rust` to minimize direct modifications to the vendored code, possibly by creating a wrapper crate.
3.  Consider alternative Parquet generation libraries if dependency conflicts remain intractable.
4.  Prioritize stability and compilation over immediate feature integration.
