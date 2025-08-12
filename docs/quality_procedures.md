# Quality Procedures for Ragit Development

This document outlines the quality procedures to ensure the stability, efficiency, and maintainability of the Ragit project.

## Code Search and Memory Management

**Procedure:** All code search operations within the Ragit project must utilize the internal `ragit` search capabilities. External JavaScript-based search tools are prohibited due to their propensity to cause out-of-memory errors and hinder development workflow.

**Rationale:** The `ragit` search is optimized for efficient code indexing and retrieval within large codebases, providing superior performance and stability compared to external tools. It also supports advanced search functionalities, such as matching filenames to function names, which enhances developer productivity.

**Implementation:**
*   Developers are required to use `ragit` for all code search tasks.
*   Any new search functionalities or integrations must be built upon the `ragit` search core.
*   Regular monitoring of memory usage during development and testing will be conducted to ensure adherence to this procedure.

## General Development Practices

*   **One Declaration Per File:** Adhere to the principle of "one declaration per file" for structs, enums, and functions to promote modularity, testability, and reusability.
*   **Consistent Path Handling:** Use `PathBuf` consistently for all file system operations, leveraging helper functions for conversions and path manipulation.
*   **Module Visibility and Re-exports:** Correctly declare and re-export public functions and modules to ensure proper accessibility throughout the crate.
*   **Error Trait Implementations:** Implement `From` traits for custom error enums to ensure proper error propagation.
*   **Systematic Error Resolution:** Address compilation errors systematically, one type or file at a time, and re-run `cargo build` after each set of changes.
*   **Runtime Resource Loading:** Prefer runtime loading for resources (e.g., prompts) that may have dynamic paths, especially in temporary environments.
*   **Specific Error Handling:** Implement specific error logging and granular error handling to pinpoint the exact cause of issues, particularly for file system operations.
*   **`cargo check` vs. `cargo run`:** Utilize `cargo check` for quick syntax and type checking, and `cargo run` (or `cargo test`) for catching runtime errors and environmental issues.
*   **Workspace Binary Execution:** When running a binary within a workspace, use `cargo run --package <package_name> -- <args>` to specify the correct package and pass arguments.
