## SOP: Reviewing Changes in a Submodule

### 1. Purpose
This Standard Operating Procedure (SOP) outlines the steps for effectively reviewing changes within a Git submodule, ensuring adherence to project standards, identifying potential impacts, and maintaining code quality.

### 2. Scope
This SOP applies to all submodule updates and modifications within the `ragit` project.

### 3. Procedure

#### Phase 1: Initial Documentation Review
Before diving into the code, review the project's central documentation to understand the submodule's role and any high-level changes.

1.  **Review `docs/index/index.md`:** Check for any updates or notes related to the submodule in the main project index.
2.  **Review `docs/index/glossary.md`:** Look for new or modified terms that might be introduced by the submodule update.
3.  **Check Submodule-Specific Documentation:** If the submodule has its own `docs/` directory or `README.md` at its root, review these for specific change logs, migration guides, or updated usage instructions.

#### Phase 2: Codebase Analysis
Once the documentation review is complete, proceed to analyze the code changes within the submodule.

1.  **Navigate to the Submodule Directory:**
    ```bash
    cd <path/to/submodule>
    ```
2.  **Inspect Git History:** Review the `git log` within the submodule to understand the commit history and the nature of the changes.
    ```bash
    git log --patch -3 --all # Review last 3 commits across all branches
    ```
3.  **Review `Cargo.toml` (if Rust project):** Check for dependency changes, new features, or breaking changes.
4.  **Review `src/` directory:** Examine modified source files, paying attention to:
    -   Adherence to 'one declaration per file' principle.
    -   Changes in core logic, APIs, or data structures.
    -   Consistency with existing code patterns and conventions.
    -   Potential impact on dependent modules in the main project.
5.  **Review Test Files:** Check for updated or new tests that validate the changes.

#### Phase 3: Impact Assessment
Assess the impact of the submodule changes on the main project.

1.  **Identify Dependent Modules:** Determine which parts of the main `ragit` project depend on the updated submodule.
2.  **Run Relevant Tests:** Execute unit, integration, and system tests in the main project that are affected by the submodule changes.
3.  **Perform Build and Lint Checks:** Ensure the main project still builds successfully and passes all linting and type-checking rules.

### 4. Reporting
Document the review findings, including:
-   Summary of changes in the submodule.
-   Identified impacts on the main project.
-   Any issues found and proposed resolutions.
-   Confirmation of adherence to project standards.

### 5. Related SOPs
-   [GM Meta-Program SOP](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/gm_meta_program.md)
-   [KitKat Meta-Program SOP](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/kitkat_meta_program.md)
-   [Project Branching Strategy SOP](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/project_branching_strategy.md)
-   [Dependency Management SOP](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/dependency_management.md)
