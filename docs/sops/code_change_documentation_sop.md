# SOP: Code Change Documentation

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the process for documenting code changes within the `ragit` project. Consistent and thorough documentation ensures maintainability, facilitates collaboration, and provides a historical record of modifications, adhering to QA best practices.

## 2. Scope
This SOP applies to all code modifications within the `ragit` project, including new features, bug fixes, refactoring, and dependency management.

## 3. Procedure

### 3.1. Identify Changed Files
Before documenting, identify all files that have been modified, added, or deleted as part of the code change. Use `git status` and `git diff` for this purpose.

### 3.2. Create/Update Relevant SOPs
For significant changes (e.g., dependency management, major refactoring, new architectural components), create a new dedicated SOP document (e.g., `docs/sops/my_new_feature_sop.md`). For minor changes or updates to existing components, update the relevant existing SOPs.

### 3.3. Document Changes within the SOP
Within the relevant SOP, include the following details:

*   **Problem Identification**: Clearly state the problem or objective that the code change addresses.
*   **Analysis**: Describe any analysis performed to understand the problem or existing codebase.
*   **Procedure/Modifications**: Detail the specific steps taken to implement the change. This should include:
    *   File paths of modified files.
    *   Specific code snippets (before and after, if applicable) for critical changes.
    *   Rationale for the chosen approach.
*   **Tools Used**: List the tools utilized during the process (e.g., `read_file`, `replace`, `run_shell_command`, `git`).
*   **Expected Outcome**: Describe the anticipated result of the changes.
*   **Verification**: Outline how the changes were or will be verified (e.g., `cargo build`, `cargo test`, specific command outputs).

### 3.4. Commit Message Formulation
Craft a clear, concise, and informative commit message that summarizes the changes. The commit message should reference the relevant SOP document(s).

### 3.5. Branching Strategy
For significant changes, create a new feature or bugfix branch from the appropriate base branch (e.g., `main`, `develop`). Follow the project's branching strategy SOP for naming conventions and workflow.

### 3.6. QA Review
Ensure that the documented changes and the code itself undergo a thorough QA review process, adhering to the project's QA SOPs.

## 4. Tools
*   `git status`
*   `git diff`
*   `read_file`
*   `write_file`
*   `replace`
*   `run_shell_command`

## 5. Expected Outcome
*   All code changes are thoroughly documented in relevant SOPs.
*   A clear and traceable history of code modifications is maintained.
*   Improved collaboration and understanding of the codebase.
*   Adherence to project QA standards.
