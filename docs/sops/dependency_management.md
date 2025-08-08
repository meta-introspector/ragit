## SOP: Dependency Management

### 1. Purpose
This Standard Operating Procedure (SOP) outlines a controlled and consistent process for managing external Rust dependencies within the `ragit` project. It prioritizes vendoring or forking crates into the `vendor/` directory to ensure reproducibility, security, and stability, aligning with ISO 9000 quality standards.

### 2. Scope
This SOP applies to all external Rust crates used in the `ragit` project, including new additions, updates, and removals.

### 3. Core Principles
-   **Control:** Maintain full control over external dependencies by vendoring or forking.
-   **Reproducibility:** Ensure consistent builds across all environments by using local copies of dependencies.
-   **Security:** Mitigate supply chain risks by vetting and managing vendored code.
-   **Stability:** Prevent unexpected breaking changes from upstream updates.
-   **Transparency:** Clearly document all vendored dependencies and their origins.

### 4. Procedure

#### 4.1. Identifying a New Dependency
1.  **Evaluate Need:** Before adding a new dependency, thoroughly evaluate if its functionality can be implemented internally or if an existing dependency can be extended.
2.  **License Compatibility:** Verify that the new dependency's license is compatible with the `ragit` project's licensing.
3.  **Security Review:** Conduct a preliminary security review of the dependency's codebase, especially for critical components.

#### 4.2. Vendoring a New Dependency
1.  **Clone/Copy to `vendor/`:** Clone or copy the external crate into the `vendor/` directory. The directory structure should typically be `vendor/<crate-name>/`.
    ```bash
    git clone <repository-url> vendor/<crate-name>
    # OR
    cp -r <path-to-crate> vendor/<crate-name>
    ```
2.  **Update `Cargo.toml`:**
    -   For the crate(s) that will use the vendored dependency, modify their `Cargo.toml` to point to the local path.
    -   Add a `[patch.crates-io]` section to the root `Cargo.toml` (or the relevant workspace `Cargo.toml`) to redirect the dependency to the local path.
    ```toml
    # Example in root Cargo.toml
    [patch.crates-io]
    <original-crate-name> = { path = "vendor/<crate-name>" }
    ```
3.  **Update `Cargo.lock`:** Run `cargo update` to ensure `Cargo.lock` reflects the vendored path. This should be done carefully to avoid unnecessary updates to other dependencies.
    ```bash
    cargo update --package <crate-name>
    ```
    *Note: User prefers to avoid `cargo update` unless absolutely necessary due to long build times. Use targeted updates when possible.* 
4.  **Commit Changes:** Commit the vendored code and the `Cargo.toml`/`Cargo.lock` modifications in a single, atomic commit.
    ```bash
    git add vendor/<crate-name> Cargo.toml Cargo.lock
    git commit -m "feat: Vendor <crate-name> for local dependency management"
    ```

#### 4.3. Updating an Existing Vendored Dependency
1.  **Pull Latest Changes:** Navigate into the vendored crate's directory and pull the latest changes from its upstream repository.
    ```bash
    cd vendor/<crate-name>
    git pull origin <branch-name>
    ```
2.  **Review Changes:** Thoroughly review the pulled changes for any breaking changes, new features, or security implications.
3.  **Update `Cargo.lock`:** If necessary, run `cargo update --package <crate-name>` from the project root.
4.  **Commit Changes:** Commit the updated vendored code and `Cargo.lock`.

#### 4.4. Removing a Dependency
1.  **Remove from `Cargo.toml`:** Remove the dependency entry from all relevant `Cargo.toml` files.
2.  **Remove from `vendor/`:** Delete the corresponding directory from the `vendor/` folder.
3.  **Update `Cargo.lock`:** Run `cargo update` to clean up `Cargo.lock`.
4.  **Commit Changes:** Commit the removal.

### 5. Verification
-   **Build Project:** Ensure the entire project builds successfully after dependency changes.
-   **Run Tests:** Execute all relevant tests to confirm no regressions were introduced.
-   **Code Review:** Have another developer review the dependency changes.

### 6. Related SOPs
-   [SOP: Project Branching Strategy](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/project_branching_strategy.md)
-   [SOP: Reviewing Changes in a Submodule](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/review_submodule_changes.md)
