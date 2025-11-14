# Refactoring Progress

This document tracks the ongoing refactoring efforts within the `ragit` project.

## 1. Bootstrap Logic Modularity

**Initial Goal:** Extract shared bootstrap logic from `ragit-command-bootstrap` and `ragit-build-index-worker-single-file` into a common library.

**Decision:** Based on architectural feedback, the common code between two Layer 7 modules (application layer) should reside in Layer 6 (presentation layer).
**Action:**
- Created a new crate: `crates/layer6_presentation/ragit-bootstrap-logic`.
- Moved `build_index_logic` module from `ragit-command-bootstrap/src/bootstrap_commands/` to `ragit-bootstrap-logic/src/`.
- Updated `Cargo.toml` dependencies in `ragit-command-bootstrap` and `ragit-build-index-worker-single-file` to depend on `ragit-bootstrap-logic`.
- Removed duplicated `build_index_logic` from `ragit-build-index-worker-single-file`.
- Corrected internal `use` paths within `ragit-bootstrap-logic`.
- Removed `pub mod build_index_logic;` from `mod.rs` files in `ragit-command-bootstrap` and `ragit-build-index-worker-single-file`.
- **Status:** `ragit-bootstrap-logic` compiles successfully.

## 2. Core Type Relocation (`ragit-index-types`)

**Issue:** `ragit-index-types` was located in `crates/legacy_and_refactoring/`. This is an inappropriate location for core data structures.
**Decision:** `ragit-index-types` is a foundational type definition for indexing and belongs in `layer3_network`.
**Action:**
- Moved `crates/legacy_and_refactoring/ragit-index-types` to `crates/layer3_network/`.
- Updated all `Cargo.toml` files referencing `ragit-index-types` to reflect the new path.

## 3. Error Type Relocation (`ragit-error`, `ragit-error-conversions`)

**Issue:** `ragit-error` and `ragit-error-conversions` were located in `crates/legacy_and_refactoring/`. These are fundamental error types.
**Decision:** Error types are foundational and belong in `layer1_physical`.
**Action:**
- Moved `crates/legacy_and_refactoring/ragit-error` to `crates/layer1_physical/`.
- Moved `crates/legacy_and_refactoring/ragit-error-conversions` to `crates/layer1_physical/`.
- Updated all `Cargo.toml` files referencing these crates to reflect the new paths.

## 4. TF-IDF Relocation (`ragit-tfidf`)

**Issue:** `ragit-tfidf` was located in `crates/legacy_and_refactoring/` and was causing build failures due to its location.
**Decision:** `ragit-tfidf` is closely related to indexing logic and should reside in `layer3_network`.
**Action:**
- Moved `crates/legacy_and_refactoring/ragit-tfidf` to `crates/layer3_network/`.
- Updated all `Cargo.toml` files referencing `ragit-tfidf` to reflect its new path in `crates/layer3_network/`.

## 5. Bootstrap Command Refinement

**Issue:** The `bootstrap_index_self` function in `ragit-command-bootstrap` was hardcoding `max_iterations` and `max_memory_gb` to `Some(1)`, overriding command-line arguments.
**Action:** Removed the hardcoded `or(Some(1))` assignments for `max_iterations` and `max_memory_gb` in `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_command.rs`.

## Overall Status

**All changes implemented so far compile successfully.**

## 6. Verbose Output Evaluation

**Goal:** Review the verbose output of the `bootstrap` command to evaluate the informativeness of each line and adjust for clarity and conciseness.

**Action:**
- Replaced `println!` statements with `memory_monitor.verbose` calls in:
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/add_bootstrap_files.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/file_copy_utils.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/file_source.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/copy_prompts.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/export_chunks/export_chunks_main.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/final_reflective_query/execute_query.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/final_reflective_query/log_start.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/self_improvement/handle_improved_code.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/self_improvement/log_start.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/write_chunk_object.rs`
    - `crates/layer6_presentation/ragit-bootstrap-logic/src/build_index_logic/main_build_index.rs`
    - `crates/layer6_presentation/ragit-bootstrap-logic/src/build_index_logic/process_file/process_staged_file.rs`
    - `crates/layer6_presentation/ragit-bootstrap-logic/src/build_index_logic/process_file/add_chunk_to_index.rs`
- Cleaned up unused `verbose` variables by prefixing them with an underscore in relevant function signatures.

**Status:** DONE. The verbose output is now more concise and informative.

## 7. Performance Monitoring

**Goal:** Implement performance monitoring for verbose calls, measuring time and memory differences between calls and reporting if they exceed configurable thresholds.

**Action:**
- Added `time_threshold_ms` and `memory_threshold_bytes` fields to the `MemoryMonitor` struct in `crates/layer1_physical/ragit-memory-monitor/src/lib.rs`.
- Updated the `MemoryMonitor::new` function to accept `time_threshold_ms` and `memory_threshold_bytes`.
- Modified `capture_and_log_snapshot` in `crates/layer1_physical/ragit-memory-monitor/src/lib.rs` to check against these thresholds and log alerts if exceeded.
- Added `time_threshold_ms` and `memory_threshold_bytes` as command-line arguments to the `ragit-commands` CLI.
- Passed `time_threshold_ms` and `memory_threshold_bytes` from `main.rs` to `bootstrap_index_self`.

**Status:** DONE. Performance monitoring is now integrated and reports alerts when thresholds are exceeded.

## 5. Verbose Output Evaluation

**Goal:** Review the verbose output of the `bootstrap` command to evaluate the informativeness of each line and adjust for clarity and conciseness.

**Action:**
- Replaced `println!` statements with `memory_monitor.verbose` calls in:
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/add_bootstrap_files.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/file_copy_utils.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/file_source.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/copy_prompts.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/export_chunks/export_chunks_main.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/final_reflective_query/execute_query.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/final_reflective_query/log_start.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/self_improvement/handle_improved_code.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/self_improvement/log_start.rs`
    - `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/write_chunk_object.rs`
    - `crates/layer6_presentation/ragit-bootstrap-logic/src/build_index_logic/main_build_index.rs`
    - `crates/layer6_presentation/ragit-bootstrap-logic/src/build_index_logic/process_file/process_staged_file.rs`
    - `crates/layer6_presentation/ragit-bootstrap-logic/src/build_index_logic/process_file/add_chunk_to_index.rs`
- Cleaned up unused `verbose` variables by prefixing them with an underscore in relevant function signatures.

**Status:** DONE. The verbose output is now more concise and informative.