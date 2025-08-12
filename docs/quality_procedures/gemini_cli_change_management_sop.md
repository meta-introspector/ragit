# SOP: Using Gemini CLI with Structured Change Management

## 1. Objective
To define a clear, structured, and auditable process for using the Gemini CLI agent to perform development tasks within the `ragit` project, ensuring all actions align with our established change management procedures.

## 2. Scope
This SOP applies to all development tasks performed by the Gemini CLI agent, including but not limited to bug fixes, feature development, refactoring, and documentation updates.

## 3. Procedure

### Phase 1: Proposal and Planning

1.  **Define the Objective:** The user provides a clear, high-level goal for the task.
2.  **Consult Existing Documentation:** Before any action, I will consult the `docs/index/index.md` and relevant SOPs to understand the existing context and procedures.
3.  **Analyze the Codebase:** I will use my tools (`glob`, `search_file_content`, `read_file`) to analyze the relevant parts of the codebase, following the Hierarchical Search SOP.
4.  **Formulate a Written Plan:** I will create a detailed, step-by-step plan for achieving the objective. This plan will be presented to the user for approval before any modifications are made.

### Phase 2: Implementation

1.  **Create a Dedicated Branch:** All work will be performed on a new, dedicated branch, following the naming conventions outlined in the Branching Strategy SOP (`feature/<name>` or `bugfix/<id>-<name>`).
2.  **Execute the Plan:** I will execute the approved plan, using my tools to modify the codebase. All actions will adhere to the relevant SOPs (e.g., Dependency Management, Code Deduplication).
3.  **Save Drafts:** To prevent data loss, all drafts of new files or significant changes will be saved directly to the filesystem in the appropriate directory.

### Phase 3: Verification and Commit

1.  **Verify Changes:** After implementing the changes, I will run any relevant verification steps, such as building the code or running tests, to ensure the changes are correct and do not introduce regressions.
2.  **Stage Changes:** I will stage all new and modified files using `git add`.
3.  **Create Commit Message:** I will create a detailed commit message that clearly explains the "what" and "why" of the changes. The commit message will be saved to `temp_commit_message.txt`.
4.  **Commit Changes:** I will commit the changes using `git commit -F temp_commit_message.txt`.

## 4. Expected Outcome
*   A transparent, auditable, and consistent workflow for all Gemini CLI-driven development.
*   Improved alignment with the project's quality and change management standards.
*   Reduced risk of errors and regressions.