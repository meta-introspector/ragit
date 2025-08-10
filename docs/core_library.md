# Core Project Library (`src/lib.rs`)

## Overview

The `src/lib.rs` file located at the root of the `ragit` project serves as the main library entry point. It defines the top-level module structure for the entire project, making core functionalities and data structures accessible to other parts of the system and external consumers.

## Purpose

This file plays a crucial role in:
*   **Module Organization:** Declaring and organizing the primary modules that constitute the `ragit` project.
*   **Centralized Re-exports:** Providing a convenient single point for re-exporting essential types, traits, and functions from various internal `ragit` crates (e.g., `ragit-types`, `ragit-utils`, `ragit-model-query-response`). This simplifies imports for other crates and applications that depend on the `ragit` library.
*   **Global Allocator Configuration:** (If `jemalloc` feature is enabled) Configures `jemalloc` as the global memory allocator, which can improve memory performance.
*   **Versioning Guidelines:** Contains internal comments outlining the versioning rules for the project, ensuring consistency in development and published versions.

## Structure

The file primarily consists of:
*   `#[cfg(feature = "jemalloc")]` block: Conditional compilation for `jemalloc` allocator.
*   `pub mod` declarations: Exposing internal modules like `constant`, `error_reporting`, `imports`, `prelude`, `prompts`, `query`, and `tree`.
*   `pub use` statements: Re-exporting key components from other `ragit` crates, such as `FileTree` from `ragit_agent`, `ApiConfig` from `ragit_types`, `Keywords`, `MultiTurnSchema`, `QueryConfig` from `ragit_utils`, `ModelQueryResponse`, `QueryTurn` from `ragit_model_query_response`, `Uid` from `ragit_types`, and `UidQueryConfig`, `UidQueryResult` from `ragit_index`.
*   Comments: Providing internal guidelines, particularly for version numbering.

## Importance

`src/lib.rs` acts as the central hub for the `ragit` library, orchestrating the visibility and accessibility of its various components. Any application or crate that depends on the main `ragit` library will interact with it through this file.

---
