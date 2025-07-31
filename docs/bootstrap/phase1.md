# Bootstrap Phase 1: Environment Setup, File Staging, and Placeholder Index Building

This document outlines the current state and key components of the `ragit` bootstrap process, focusing on the initial phase of environment setup, file staging, and the placeholder index building. The primary goal of this phase is to establish a robust framework for the self-improving bootstrap, with a strong emphasis on modularity and observability, particularly regarding memory usage.

## Key Components and Workflow:

The `bootstrap` command's execution flow is orchestrated within `crates/layer7_application/ragit-build-index-worker-single-file/src/main.rs`. It follows a sequential process, with each major step encapsulated in its own function, adhering to the "one declaration per file" principle for enhanced modularity and maintainability.

1.  **Environment Setup (`setup_environment`):**
    *   Initializes the system for memory monitoring using `sysinfo`.
    *   Creates a temporary directory (`tmp_bootstrap`) where all intermediate files (like copied prompts and the generated index) will reside.
    *   Determines the actual root directory of the project.
    *   An `Index` struct is initialized within this function.

2.  **Prompt Copying (`copy_prompts`):**
    *   Copies all necessary prompt definition language (`.pdl`) files from the project's `prompts/` directory to the temporary `tmp_bootstrap/prompts/` directory. This ensures that the bootstrap process has access to the required prompts for operations like summarization.

3.  **File Staging (`add_bootstrap_files`):**
    *   Identifies and stages files that will be included in the index. This process leverages the `FileSource` trait defined in `crates/layer7_application/ragit-build-index-worker-single-file/src/bootstrap_commands/file_source.rs`.
    *   **`FileSource` Implementations:**
        *   `StaticFileSource`: For providing a predefined, static list of files.
        *   `CargoPackageFileSource`: Designed to automatically discover and stage all `.rs` source files within a specified Cargo package (e.g., the `ragit-command-bootstrap` package itself, enabling self-indexing).
        *   `GlobFileSource`: For staging files based on a given glob pattern.
    *   The staged files are added to the `Index` struct.

4.  **Placeholder Index Building (`build_index`):**
    *   Located in `crates/layer7_application/ragit-build-index-worker-single-file/src/bootstrap_commands/build_index.rs`.
    *   **Current State:** This function is currently a **placeholder**. It iterates through the `index.staged_files` but does not yet implement the actual logic for chunking, embedding, or adding content to the index. Its primary role in this phase is to simulate the build process and integrate with memory profiling.
    *   It includes checks for the `summarize.pdl` prompt in the temporary directory, although its absence is currently only a warning, not an error, due to the placeholder nature.

5.  **Cleanup:**
    *   After the bootstrap process, the temporary directory and its contents are removed to ensure a clean state.

## Observability and Memory Profiling:

A significant enhancement in this phase is the integration of comprehensive memory profiling. The `main.rs` utilizes `sysinfo` and custom `memory_profiler` utilities to capture and log memory snapshots at critical junctures of the bootstrap process. This provides detailed insights into memory consumption, aiding in debugging and optimization, especially when dealing with large codebases or memory-intensive operations.

*   Memory snapshots are captured and logged before and after `setup_environment`, `copy_prompts`, `add_bootstrap_files`, and `build_index`.
*   Constants defined in `crates/layer7_application/ragit-build-index-worker-single-file/src/bootstrap_commands/constants.rs` are used for consistent logging messages related to memory usage.

## Current Discrepancy: `Index` Mutability

A notable inconsistency exists in how the `Index` struct is handled:
*   In `crates/layer7_application/ragit-build-index-worker-single-file/src/main.rs`, the `&mut index` argument is commented out when calling `add_bootstrap_files` and `build_index`.
*   However, the `build_index` function (and likely `add_bootstrap_files` internally) still expects a mutable `Index` (`index: &mut Index`).

This discrepancy needs to be resolved to ensure the `Index` struct is correctly modified and persisted throughout the bootstrap process when the actual indexing logic is implemented. The current setup suggests that the `Index` is created in `setup_environment` but its subsequent modification is not consistently reflected in the function calls in `main.rs`.

## Comparison: Old `ragit-commands` Bootstrap vs. New Single-File Bootstrap

This section outlines the key differences and architectural shifts between the original `bootstrap` command found in `ragit-commands` and the new, synchronous, single-threaded implementation located in `crates/layer7_application/ragit-build-index-worker-single-file/`.

### Old `ragit-commands` Bootstrap (as per `docs/bootstrap.md`)

*   **Location:** Primarily within `crates/layer7_application/commands/ragit-command-bootstrap/`.
*   **Concurrency:** Implied to be potentially multi-threaded (e.g., `--workers` flag), though `docs/bootstrap.md` notes it defaults to 1 worker due to ongoing refactoring.
*   **Purpose:** A comprehensive, self-improving tool that not only builds an index but also uses it to analyze and improve the `ragit` codebase itself. It includes steps for self-improvement and final reflective queries.
*   **Workflow:**
    1.  Setup Environment (creates temp dir, initializes repo/index)
    2.  Copy Prompts (copies and loads prompts into index)
    3.  Add Bootstrap Files (identifies and copies files, adds to index)
    4.  Build Index (builds index from source)
    5.  Write Chunks to Markdown
    6.  Self-Improvement (analyzes and improves code)
    7.  Final Reflective Query
*   **Memory Management:** Mentions flags like `--max-memory-gb` for graceful exit and debugging OOM issues, along with `jemalloc` profiling.
*   **Modularity:** While functions are separated, the overall structure is a single command crate.

### New Single-File Bootstrap (`ragit-build-index-worker-single-file`)

*   **Location:** `crates/layer7_application/ragit-build-index-worker-single-file/`.
*   **Concurrency:** Explicitly designed as synchronous and single-threaded, focusing on a controlled, step-by-step execution.
*   **Purpose:** Focused on the core task of building an index from source files, with robust memory profiling and environment setup. It currently acts as a foundational step for future self-improvement logic, but does not yet include the self-improvement or reflective query steps.
*   **Workflow (Current Implementation):**
    1.  `setup_environment`: Initializes system, creates temp directory, determines root, initializes `Index` struct.
    2.  `copy_prompts`: Copies prompts to the temporary directory.
    3.  `add_bootstrap_files`: Identifies and stages files using the `FileSource` trait (Static, CargoPackage, Glob).
    4.  `build_index`: **Placeholder** for actual index building logic; currently iterates staged files.
    5.  Cleanup: Removes the temporary directory.
*   **Memory Management:** Integrates comprehensive memory profiling using `sysinfo` and custom utilities, capturing snapshots at each major step. This is a core feature for observability and debugging.
*   **Modularity:** Adheres strictly to the "one declaration per file" principle, leading to a highly modular and granular codebase. This makes individual components easier to understand, test, and maintain.
*   **`Index` Mutability Discrepancy:** As noted previously, there's an inconsistency where `main.rs` comments out the mutable `Index` argument for `add_bootstrap_files` and `build_index`, while these functions still expect it. This needs resolution for proper index modification.

### Key Architectural Shifts

1.  **Focus:** The new bootstrap shifts focus from a full self-improvement loop to a more granular, robust, and observable index-building foundation. The self-improvement and reflective query aspects are currently omitted, implying they will be built on top of this stable base.
2.  **Modularity and Granularity:** The "one declaration per file" principle in the new bootstrap significantly increases modularity, making the codebase more maintainable and easier to reason about.
3.  **Explicit Memory Profiling:** The new bootstrap integrates memory profiling as a first-class citizen, providing detailed insights into memory usage at each stage, which is crucial for optimizing performance and preventing OOM errors in a controlled manner.
4.  **Synchronous Execution:** By being synchronous and single-threaded, the new bootstrap aims for predictable behavior and easier debugging, contrasting with the potential complexities of multi-threaded execution in the old approach.
5.  **`FileSource` Abstraction:** The introduction of the `FileSource` trait provides a flexible and extensible way to define how files are identified and staged for indexing, allowing for different input sources (static, Cargo packages, glob patterns).