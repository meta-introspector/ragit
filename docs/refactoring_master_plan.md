# Ragit Refactoring Master Plan

**Date:** July 23, 2025

This document provides a comprehensive overview of the entire `ragit` refactoring effort. It consolidates all previous logs and outlines the final plan to achieve a stable, modular, and maintainable codebase.

---

## 1. Historical Context & Key Architectural Changes

The primary motivation for this refactoring was to resolve complex circular dependencies between the original crates. This led to a major architectural shift towards a micro-crate design.

*   **`ragit-core`:** Introduced to house fundamental traits (`Matcher`) and break dependency cycles.
*   **`ragit-types`:** Created as a "pure" crate containing only data structures (`Uid`, `FileSchema`, `ChunkSchema`, etc.) with no local dependencies.
*   **`ragit-ignore`:** Became the dedicated home for `.gitignore` parsing logic (`Ignore`, `Pattern`).
*   **`ragit-commands`:** All CLI command implementations were moved here from the main `ragit` binary, which now acts as a thin wrapper.
*   **Dependency Resolution:** The core achievement was breaking the cycles between `ragit-api`, `ragit-utils`, `ragit-types`, `ragit-uid`, and `ragit-pdl`.

---

## 2. Current Status: Errors in `ragit-commands`

A `cargo build --workspace` reveals that the refactoring has successfully stabilized the lower-level crates. However, the `ragit-commands` crate is now failing to compile with a large number of errors.

**Error Analysis:**

The errors are predictable consequences of the refactoring and fall into three categories:
1.  **"Not Found" (E0425, E0433):** The most common error. Functions, methods, and types have been moved to new crates or modules, and the `use` statements in `ragit-commands` are now incorrect.
2.  **Incorrect API Usage (E0599, E0061, E0308):** Method signatures have changed. Methods that were once part of the `Index` struct (`index.method()`) are now standalone functions (`method(&index)`), and the calls need to be updated.
3.  **Missing Trait Implementations (E0277):** Structs like `RemoveResult` no longer implement `Display`, causing `println!` macros to fail.

---

## 3. The Plan to Win: A Two-Phase Approach

This plan will first stabilize the project by fixing the existing errors and then execute a final refactoring to create a more robust and ergonomic architecture.

### **Phase 1: Stabilize and Fix (Immediate Priority)**

The goal is to get the project to compile again by methodically fixing the errors in `ragit-commands`.

1.  **Clean `ragit-utils` Warnings:** Run `cargo fix --lib -p ragit-utils` to automatically apply suggestions for unused imports and variables, reducing compiler noise.
2.  **Fix `ragit-commands` Imports:** Systematically go through each compiler error and add the correct `use` statements. The compiler's suggestions will be followed (e.g., importing `read_string` from `ragit_fs`, `PullResult` from `ragit_utils`, etc.).
3.  **Update API Calls:** Refactor all calls to methods that no longer exist on the `Index` struct. For example, `index.build()` will be changed to `build(&mut index)`, locating the new standalone function in `ragit-utils`.
4.  **Fix `Display` Errors:** Change all `println!("{}", ...)` calls for types that do not implement `Display` to use the debug formatter: `println!("{:?}", ...)`.

### **Phase 2: The Winning Refactor (Introduce `ragit-index`)**

Phase 1 will result in a working but architecturally awkward build. The final step is to restore the ergonomic `index.method()` calling convention by creating a dedicated crate for the `Index`.

**Current Status of `Index` Refactoring:** The `Index` struct and its associated methods are still being consolidated. Specifically, the `build` method (and its helpers) is currently located in `src/index/commands/build.rs` but needs to be moved to `crates/ragit-utils/src/index/index_struct.rs` to be a proper method of the `Index` struct.

**Action: Create the `ragit-index` Crate (Revised)**

1.  **Create New Crate:** Initialize a new crate at `crates/ragit-index`.
2.  **Move `Index` Struct:** Relocate the `Index` struct definition from `ragit-utils` to `crates/ragit-index/src/lib.rs`.
3.  **Consolidate Methods:** Move all the standalone index-related functions (including the `build` method and its helpers) from the `ragit-utils/src/index/` submodules and `src/index/commands/build.rs` back into `impl Index` blocks within the new `ragit-index` crate. Make these methods `pub`.
4.  **Update Dependencies:** Modify the `Cargo.toml` files for `ragit-commands` and any other relevant crates to depend on the new `ragit-index` crate for all index-related functionality.

**Benefit:** This final step will complete the refactoring. It will centralize all core index logic into a single, cohesive crate, restoring an intuitive and ergonomic API for the rest of the application to use.

### **Phase 3: Verify Command Refactoring**

After the `Index` refactoring is complete, I will verify that all command implementations have been successfully moved from the root `ragit` crate into the `ragit-commands` crate. This will involve checking `crates/ragit-commands/src/lib.rs` and the `crates/ragit-commands/src/commands/` directory.
