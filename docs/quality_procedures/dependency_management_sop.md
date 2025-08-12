// Emojis: 📦🍴🔒
// Hyperspace: [0.70, 0.90, 0.10, 0.30, 0.50, 0.70, 0.90, 0.10]
// Summary Number: 20250808

# SOP: Dependency Management - Vendored & Forked Crates

## 1. Objective
To establish a controlled and reproducible process for managing external code dependencies within the `ragit` project, prioritizing the use of vendored and forked crates to ensure stability, security, and long-term maintainability, in compliance with ISO 9000 principles.

## 2. Scope
This SOP applies to all external Rust crates and libraries integrated into the `ragit` project, particularly those managed via `Cargo.toml` and `git` submodules.

## 3. Core Principles
- **Control**: Maintain direct control over dependency source code to mitigate risks associated with upstream changes or disappearances.
- **Reproducibility**: Ensure that project builds are consistently reproducible across different environments and over time.
- **Security**: Facilitate internal security audits and the application of custom patches without reliance on external maintainers.
- **Stability**: Minimize unexpected breaking changes from upstream updates by managing versions locally.
- **Transparency**: Document the origin and modification history of all vendored and forked dependencies.

## 4. Procedure

### Step 1: Dependency Identification and Evaluation
1.  **Identify New Dependencies**: When a new external crate is required, identify its purpose, license, and upstream repository.
2.  **Evaluate Suitability**: Assess the dependency's stability, maintenance status, and community support.
3.  **Prioritize Vendoring/Forking**:
    -   **Mandatory**: For critical or foundational dependencies (e.g., core libraries, security-sensitive components), vendoring or forking is mandatory.
    -   **Preferred**: For other dependencies, vendoring or forking is strongly preferred if it enhances control, stability, or allows for necessary modifications.
    -   **Exception**: Direct `crates.io` dependencies may be used only if vendoring/forking is impractical, introduces undue complexity, or the dependency is trivial and highly stable. All exceptions must be documented and approved by a project maintainer.

### Step 2: Vendoring or Forking the Dependency

#### Option A: Vendoring (Copying Source Code)
1.  **Copy Source**: Copy the exact version of the dependency's source code into the `vendor/` directory (e.g., `vendor/<crate_name>/`).
    ```bash
    cp -r ~/.cargo/registry/src/<source_hash>/<crate_name>-<version> vendor/<crate_name>
    ```
2.  **Update Root `Cargo.toml`**: Add a `[patch.crates-io]` entry in the root `Cargo.toml` to redirect the dependency to the local path.
    ```toml
    [patch.crates-io]
    <crate_name> = { path = "vendor/<crate_name>" }
    ```
3.  **Commit Vendored Code**: Add and commit the vendored source code to the `ragit` repository.

#### Option B: Forking (Using Git Submodule)
1.  **Fork Repository**: Create a fork of the upstream repository on GitHub (or equivalent).
2.  **Add as Submodule**: Add the forked repository as a Git submodule into the `vendor/` directory (e.g., `vendor/<crate_name>/`).
    ```bash
    git submodule add <your_fork_url> vendor/<crate_name>
    ```
3.  **Pin Version**: Ensure the submodule is pinned to a specific commit hash to guarantee reproducibility.
    ```bash
    git -C vendor/<crate_name> checkout <commit_hash>
    ```
4.  **Update Root `Cargo.toml`**: Add a `[patch.crates-io]` entry in the root `Cargo.toml` to redirect the dependency to the local path.
    ```toml
    [patch.crates-io]
    <crate_name> = { path = "vendor/<crate_name>" }
    ```
5.  **Commit Submodule**: Add and commit the submodule reference in the main `ragit` repository.

### Step 3: Applying Modifications (if necessary)
1.  **Isolate Changes**: If modifications are required (e.g., API fixes, feature additions), create a dedicated branch within the vendored/forked repository.
2.  **Implement Fixes**: Apply the necessary code changes.
3.  **Document Changes**: Clearly document all modifications within the vendored/forked repository's commit history or a dedicated `CHANGELOG.md`.
4.  **Commit Changes**: Commit the changes within the vendored/forked repository.
5.  **Update Main Repository Reference**: If using a submodule, update the main repository's reference to the new commit in the submodule.

### Step 4: Integration and Verification
1.  **Update `Cargo.toml`**: Ensure all `Cargo.toml` files (root and dependent crates) correctly reference the vendored/forked dependency.
2.  **Resolve Conflicts**: Address any compilation errors or conflicts arising from the integration.
3.  **Build and Test**: Perform a full project build (`cargo build`) and run all relevant tests (`cargo test`) to verify successful integration and functionality.
4.  **Memory Monitoring**: Continuously monitor memory usage and performance, especially for new dependencies, to ensure they align with project goals.

## 5. Documentation
- All vendored/forked dependencies must have their origin, version, and any applied modifications clearly documented.
- This SOP will be linked from `docs/index.md`.

## 6. Review and Audit
This SOP will be reviewed annually and audited as part of the project's overall quality management system to ensure its effectiveness and compliance.
