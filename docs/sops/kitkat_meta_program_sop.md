# SOP: "Have a KitKat" Meta-Program

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the process and principles of the "Have a KitKat" meta-program within the `ragit` project. Its purpose is to provide a structured workflow for pausing current development, defining and documenting a new strategic plan, committing the current state, and conceptually "rebooting" the development cycle to focus on the new plan. This ensures systematic progress, clear documentation of strategic shifts, and adherence to quality standards.

## 2. Scope
This SOP applies to all development phases and strategic planning activities within the `ragit` project where a significant shift in focus or a new strategic direction is required. It encompasses the documentation of current status, definition of new critical paths, and generalization of learning.

## 3. Procedure

### Phase 1: Pause and Assess Current State
**Objective:** To halt current development and thoroughly document the project's status.

**Steps:**
1.  **Pause Development:** Cease all active coding and refactoring tasks.
2.  **Document Current Status:**
    *   Detail all implemented features and bug fixes since the last "KitKat" or major strategic shift.
    *   List any known issues, compilation errors, or runtime panics.
    *   Specify the exact state of the codebase, including any temporary changes or workarounds.
    *   Reference any relevant commit hashes or branches.
    *   Example: "Implemented `--verbose` flag. Traced build failure to `PromptMissing("summarize")` error. `bootstrap_index_self` now copies `prompts` to temporary directory. Root cause of `PromptMissing` identified: `Index` struct's `prompts` field not populated correctly because prompts were copied *after* index initialization."

### Phase 2: Define New Strategic Plan
**Objective:** To clearly articulate the next critical path and strategic goals.

**Steps:**
1.  **Identify New Critical Path:** Define the immediate, most crucial objectives to be achieved.
2.  **Outline Steps:** Break down the critical path into actionable, measurable steps.
3.  **Set Success Criteria:** Define what constitutes successful completion of the new critical path.
4.  **Example:** "New Critical Path: Successfully run `bootstrap` command without compilation errors or runtime panics. This involves: 1. Verifying all compilation errors are resolved. 2. Confirming graceful exit behavior with `max_iterations`. 3. Ensuring `PromptMissing` error is resolved by correctly populating the `Index`'s `prompts` field."

### Phase 3: Generalize Learning and Document Best Practices
**Objective:** To extract reusable knowledge and refine development methodologies.

**Steps:**
1.  **Analyze Challenges and Solutions:** Review the problems encountered and the solutions implemented during the previous development phase.
2.  **Formulate Generalizations:** Abstract specific solutions into general principles or best practices.
3.  **Document Learnings:** Add these generalized learnings to relevant documentation (e.g., `docs/lessons.md`, `docs/refactoring_lessons.md`).
4.  **Example:** "Generalization of Learning: 'One Declaration Per File' Principle significantly improves modularity. Macro peculiarities in Rust require `format_args!` for constants. Asynchronous operations demand careful ownership handling. Systematic debugging is crucial. `std::io::Write` is needed for `flush()`. Error handling with `anyhow` and `thiserror` requires attention to `Clone` and `From` implementations."

### Phase 4: Commit and Reboot
**Objective:** To create a stable checkpoint and formally initiate the new development cycle.

**Steps:**
1.  **Stage All Changes:** Ensure all relevant files reflecting the current state and new plan are staged for commit.
2.  **Create Commit Message:** Use a detailed commit message that summarizes the "KitKat" event, referencing the documented status, new plan, and generalized learnings.
3.  **Commit Changes:** Execute the `git commit` command.
4.  **Conceptual Reboot:** Mentally shift focus to the new critical path, treating it as the primary objective.

## 4. Tools
*   `git` (for version control and committing)
*   Markdown editor (for documenting SOPs and plans)
*   `ragit` CLI (for project-specific analysis and indexing, if applicable)
*   Rust programming language (for implementing and verifying changes)

## 5. Expected Outcome
*   A clear, documented strategic plan for the next development phase.
*   A stable, committed codebase reflecting the current state and strategic shift.
*   Refined development practices and generalized learnings.
*   Improved project maintainability, clarity, and adherence to quality standards (ISO 9000, GMP, Six Sigma, C4 Model, UML, ITIL, HIPAA, and DevSecOps best practices).
