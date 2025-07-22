# Refactoring and Build Fix Plan

This document outlines the steps to fix the current build errors and refactor the codebase for better maintainability, focusing on simplifying imports.

## Phase 1: Resolve Build Errors

The current build is failing primarily due to `unresolved import` errors. This suggests that items (functions, structs, enums) were moved during recent refactoring, but their import paths were not updated in the consumer crates.

**Steps:**

1.  **Fix `main` crate command imports:**
    *   The `src/main/commands/*.rs` files have incorrect import paths for `..._command` functions.
    *   **Action:** For each error, locate the correct path of the target function within the `ragit-utils` crate. It's likely that the `commands` module structure was flattened. For example, `ragit_utils::index::commands::add::add_command` might now be `ragit_utils::index::add_command`.
    *   Update the `use` statements in each command file.

2.  **Fix `lib.rs` imports:**
    *   `src/lib.rs` has unresolved imports for `Index`, `LoadMode`, `AgentAction`, etc.
    *   **Action:** Locate the correct paths for these items in `ragit-utils` and update the `pub use` statements. For example, `ragit_utils::index::Index` is likely now `ragit_utils::index::index_struct::Index`.

3.  **Fix `ragit_uid` imports:**
    *   Several files try to `use ragit_uid::...`, which is incorrect.
    *   **Action:** Replace `use ragit_uid::...` with `use ragit_utils::...` as suggested by the compiler.

4.  **Address `main.rs` as a library module:**
    *   The compiler warns `a binary crate cannot be used as library` because `src/lib.rs` contains `pub mod main;`.
    *   **Action:** Remove `pub mod main;` from `src/lib.rs`. The `main` module is the entry point for the binary and should not be part of the library's public API.

5.  **Handle other miscellaneous errors:**
    *   Fix the `Prompt` import in `src/chunk.rs`.
    *   Fix the `INDEX_DIR_NAME` and `MODEL_FILE_NAME` imports in `src/imports.rs`.

## Phase 2: Simplify Imports with Prelude

Once the build is successful, we will simplify the import statements as requested.

**Steps:**

1.  **Review `src/prelude.rs`:** Ensure it exports all commonly used items. Add any missing items that would help reduce verbosity in other files.

2.  **Replace verbose imports:**
    *   **Action:** In each file within the `src` directory, replace blocks of `use` statements with a single `use crate::prelude::*;` where possible.

3.  **Run `cargo fix`:**
    *   **Action:** Run `cargo fix --all` to automatically clean up remaining warnings, such as unused imports that might result from the prelude refactoring.

## Phase 3: Clean Up Warnings

After the build is stable and imports are simplified, address the remaining compiler warnings.

**Steps:**

1.  **Fix unused variables:** Prefix unused variables with an underscore (e.g., `_variable_name`).
2.  **Fix dead code:** Remove unused functions and constants, or determine if they should be used.
3.  **Fix unexpected `cfg`:** Investigate the `korean` feature flag warning in `ragit-utils` and correct the `Cargo.toml` or the `cfg` attribute.
