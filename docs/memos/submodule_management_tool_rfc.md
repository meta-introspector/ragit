# RFC: Rust-based Submodule Management Tool

## 1. Title
Rust-based Submodule Management Tool for `ragit`

## 2. Abstract
This RFC proposes the development of a new Rust-based command-line tool to streamline and simplify the management of Git submodules within the `ragit` project. The tool will abstract away the complexities of nested submodule operations, providing a more intuitive and robust workflow for tasks such as updating, committing changes within, and synchronizing submodules. It will leverage existing, well-vetted Rust crates for Git interaction and file system operations.

## 3. Motivation
The `ragit` project heavily relies on Git submodules for vendoring external dependencies and managing internal components. The current manual process of navigating into nested submodules, performing operations, and then propagating those changes up the hierarchy is error-prone, cumbersome, and inefficient. This has led to repeated operational errors, as observed in recent development sessions. A dedicated Rust tool will:
*   Reduce human error in submodule management.
*   Improve developer efficiency and experience.
*   Ensure consistency in submodule operations across the project.
*   Provide a foundation for more advanced, automated submodule workflows.

## 4. Goals
*   **Simplify Common Operations**: Provide high-level commands for common submodule tasks (e.g., `update`, `commit`, `status`, `sync`).
*   **Handle Nested Submodules**: Automatically traverse and operate on nested submodules without requiring manual `cd` commands.
*   **Integrate with `ragit` Workflow**: Be designed to fit seamlessly into the existing `ragit` development and QA processes.
*   **Leverage Rust Ecosystem**: Utilize robust and performant Rust crates for Git and file system interactions.
*   **Provide Clear Feedback**: Offer clear and actionable output for all operations.

## 5. Non-Goals
*   Replacing Git itself for core version control operations.
*   Implementing a full-fledged Git client.
*   Supporting non-Git version control systems.

## 6. Proposed Design

### 6.1. Architecture
The tool will be a new Rust crate within the `ragit` workspace, potentially named `ragit-submodule-manager`. It will expose a CLI interface.

### 6.2. Core Functionality (Proposed Commands)

#### `ragit-submodule-manager update [submodule_path]`
*   **Description**: Updates the specified submodule (or all submodules if `submodule_path` is omitted) to its configured commit.
*   **Implementation**: Internally uses `git submodule update --init --recursive` or equivalent programmatic calls.

#### `ragit-submodule-manager commit [submodule_path] -m "message"`
*   **Description**: Commits changes within the specified submodule. If `submodule_path` is omitted, it will attempt to commit changes in all dirty submodules.
*   **Implementation**: 
    1.  Identifies dirty submodules.
    2.  For each dirty submodule, it performs `git add .` and `git commit -m "message"` within that submodule's repository.
    3.  Propagates the submodule commit hash up to its parent repository.

#### `ragit-submodule-manager status [submodule_path]`
*   **Description**: Shows the status of the specified submodule (or all submodules).
*   **Implementation**: Equivalent to `git submodule status --recursive` and `git status` within each submodule.

#### `ragit-submodule-manager sync [submodule_path]`
*   **Description**: Synchronizes the remote URLs for submodules.
*   **Implementation**: Internally uses `git submodule sync --recursive`.

### 6.3. Key Rust Crates to Consider
*   **`git2-rs`**: A Rust binding to libgit2, providing low-level Git operations. This will be crucial for programmatic interaction with Git repositories and submodules.
*   **`walkdir`**: For efficiently traversing directory structures to find submodules.
*   **`anyhow` / `thiserror`**: For robust error handling.
*   **`clap`**: For building the command-line interface.

## 7. Open Questions / Future Considerations
*   How to handle conflicts within submodules during automated commits.
*   Integration with `ragit`'s existing configuration system.
*   Support for specific submodule branches or tags.
*   Potential for a TUI (Text User Interface) for interactive submodule management.

## 8. Timeline
*   **Phase 1 (Initial Prototype - 1 week)**: Implement `status` and `update` commands for single-level submodules.
*   **Phase 2 (Nested Support - 2 weeks)**: Extend `status` and `update` to handle nested submodules recursively. Implement `commit` for single-level submodules.
*   **Phase 3 (Full Functionality - 2 weeks)**: Implement `commit` for nested submodules and `sync` command.
*   **Phase 4 (Integration & Testing - 1 week)**: Integrate into `ragit`'s build system and write comprehensive tests.

## 9. Conclusion
A dedicated Rust-based submodule management tool will significantly improve the developer experience and reliability of submodule operations within the `ragit` project. By leveraging the power of the Rust ecosystem, we can build a robust and efficient solution to a recurring pain point.
