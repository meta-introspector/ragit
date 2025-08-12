# SOP: Monadic Refactoring Procedure

## 1. Objective

To define a systematic, repeatable, and chainable procedure for porting or restoring functionality from one part of the codebase to another (e.g., from a previous commit, a different crate, or a submodule). This process is designed to be "monadic" in the sense that each step is a self-contained operation that logically follows the previous one, allowing us to chain them together like `.next().next()`.

## 2. Scope

This SOP applies to all refactoring tasks that involve moving, restoring, or integrating code from different parts of the project.

## 3. The Monadic Procedure

Each step in this procedure can be thought of as a `.next()` call in a chain of operations.

### .next(review_history)

**Action:** Start by reviewing the `git log --patch` to identify the source of the code to be ported. This could be a specific commit hash, a range of commits, or a commit from a different branch.

**Objective:** To locate the exact code changes that need to be restored or moved.

### .next(analyze_source)

**Action:** Analyze the patch or the code from the source commit to understand its components. This includes identifying:
*   CLI command definitions (e.g., in `cli.rs`).
*   Command handlers (e.g., in `handlers.rs`).
*   Core logic and data structures.
*   Associated data files (`.json`, `.toml`, etc.).
*   Ontology files (`.ttl`).
*   Documentation (`.md`).

**Objective:** To create a manifest of all the pieces that need to be moved or restored.

### .next(analyze_target)

**Action:** Analyze the target location (the new submodule, crate, or file) to understand its current structure. Identify the specific files that will need to be modified.

**Objective:** To map the source components to their new locations in the target.

### .next(read_target_files)

**Action:** Read the contents of the target files that will be modified. This is crucial for ensuring that the `replace` operations are precise and don't fail due to content mismatches.

**Objective:** To get the current state of the target files before modification.

### .next(apply_changes)

**Action:** Use the `replace` or `write_file` tool to apply the code changes from the source to the target files. This should be done in a targeted manner, replacing specific functions or blocks of code rather than entire files whenever possible.

**Objective:** To integrate the desired functionality into the target location.

### .next(restore_artifacts)

**Action:** Restore any associated artifacts that were identified in the `analyze_source` step. This includes creating or updating data files, ontologies, and documentation.

**Objective:** To ensure that all supporting files for the new functionality are in place.

### .next(document_changes)

**Action:** Update the relevant documentation (e.g., `README.md`, other SOPs) to reflect the changes. This includes adding information about the new functionality, its usage, and its current status.

**Objective:** To keep the project's documentation up-to-date and accurate.

### .next(verify)

**Action:** Compile and test the changes to ensure that they have been applied correctly and have not introduced any regressions. This may involve running `cargo build`, `cargo test`, and any other relevant checks.

**Objective:** To verify the correctness and stability of the changes.

## 4. Example

The restoration of the "multi-search" functionality to the `spinoffs/model-builder-quiz` submodule is a concrete example of this procedure in action. We followed these steps to identify the relevant code in a previous commit, analyze the source and target locations, and systematically apply the changes to restore the feature.
