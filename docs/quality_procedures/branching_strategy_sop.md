// Emojis: 🌳🌿🔗
// Hyperspace: [0.50, 0.70, 0.90, 0.10, 0.30, 0.50, 0.70, 0.90]
// Summary Number: 20250806

# SOP: Branching Strategy

## 1. Objective
To define a clear and consistent branching strategy for the `ragit` project, facilitating isolated development, effective collaboration, and robust integration, especially when dealing with nested submodules and complex dependency fixes.

## 2. Core Principles
- **Isolation**: All new features, bug fixes, or significant refactoring efforts should be developed on dedicated branches.
- **Hierarchy**: For work involving submodules, create branches within the submodule itself to isolate changes specific to that submodule.
- **Traceability**: Ensure all work is committed with descriptive messages and documented in relevant quality procedures or work logs.

## 3. Branch Naming Conventions
- **Feature Branches (Main Repository)**: `feature/<descriptive-name>` (e.g., `feature/dyim-embedding-pipeline`)
- **Bugfix Branches (Main Repository)**: `bugfix/<issue-id>-<descriptive-name>` (e.g., `bugfix/GH-123-fix-auth-flow`)
- **Submodule Branches**: `<submodule-name>-<descriptive-name>` (e.g., `sophia-rs-compat-fix`)

## 4. Procedure

### Step 1: Create a Main Feature/Bugfix Branch
- For any new development or bug fix in the main `ragit` repository, create a new branch from `main` (or `develop`, if applicable).
- **Command:** `git checkout -b <branch-name>`

### Step 2: Identify Submodule Involvement
- Determine if the work requires modifications within any nested Git submodules (e.g., `vendor/meta-introspector/solfunmeme-dioxus/vendor/sophia_rs/`).

### Step 3: Create a Submodule-Specific Branch (if applicable)
- If a submodule is involved, navigate into the submodule's directory.
- Create a new branch within that submodule to isolate changes specific to it.
- **Command:** `git -C <path/to/submodule> checkout -b <submodule-branch-name>`

### Step 4: Commit Changes within Submodule
- Perform all necessary modifications within the submodule.
- Stage and commit these changes *within the submodule's repository*.
- **Command (from main repo root):** `git -C <path/to/submodule> add .`
- **Command (from main repo root):** `git -C <path/to/submodule> commit -m "<descriptive-commit-message>"`

### Step 5: Update Main Repository's Submodule Reference
- After committing changes in the submodule, the main repository's reference to that submodule will be updated.
- Stage and commit this updated submodule reference in the main repository.
- **Command (from main repo root):** `git add <path/to/submodule>`
- **Command (from main repo root):** `git commit -m "Update <submodule-name> to <new-commit-hash>"`

### Step 6: Integrate Submodule Branch (Optional, for specific scenarios)
- In some cases, it might be necessary to branch off an older version of a submodule (e.g., to apply a fix to a specific release or to isolate a compatibility layer).
- **Command (from submodule directory):** `git checkout -b <new-branch-name> <older-commit-hash>`
- **Note:** This creates a detached HEAD state in the submodule. Ensure you understand the implications and manage the submodule's state carefully.

## 5. Documentation
- All significant branching decisions and their rationale should be documented in the relevant work logs (e.g., `dyim_embedding_pipeline_work_log.md`).
- New SOPs should be linked from `docs/index.md`.
