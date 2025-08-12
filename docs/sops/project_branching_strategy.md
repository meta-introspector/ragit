## SOP: Project Branching Strategy

### 1. Purpose
This Standard Operating Procedure (SOP) defines the branching strategy for the `ragit` project, ensuring a clear, consistent, and efficient workflow for development, bug fixes, and releases. It aims to maintain code quality, facilitate collaboration, and enable effective version control.

### 2. Scope
This SOP applies to all developers contributing to the `ragit` project and its submodules.

### 3. Core Principles
-   **`main` branch:** This branch represents the stable, production-ready codebase. Direct commits to `main` are strictly prohibited.
-   **Feature Branches:** All new development, features, and significant changes must be done in dedicated feature branches.
-   **Descriptive Naming:** Branch names should be clear, concise, and reflect the purpose of the branch.
-   **Regular Updates:** Feature branches should be regularly updated with changes from `main` to minimize merge conflicts.
-   **Code Review:** All changes must undergo a code review before being merged into `main`.

### 4. Branch Naming Conventions
-   **Feature Branches:** `feature/<descriptive-name>` (e.g., `feature/add-new-parser`, `feature/implement-caching`)
-   **Bugfix Branches:** `bugfix/<issue-id>-<descriptive-name>` (e.g., `bugfix/123-fix-login-error`, `bugfix/456-resolve-memory-leak`)
-   **Release Branches:** `release/<version-number>` (e.g., `release/1.0.0`, `release/1.1.0`)
-   **Hotfix Branches:** `hotfix/<issue-id>-<descriptive-name>` (e.g., `hotfix/789-critical-security-patch`)
-   **Submodule-Specific Branches:** When working on a submodule, create a branch within the submodule's repository following its own conventions, or if none exist, use `<submodule-name>-<descriptive-name>` (e.g., `coccinelleforrust-update-parser`).

### 5. Workflow Procedures

#### 5.1. Creating a New Feature/Bugfix Branch
1.  **Ensure `main` is up-to-date:**
    ```bash
    git checkout main
    git pull origin main
    ```
2.  **Create and switch to your new branch:**
    ```bash
    git checkout -b <branch-name>
    ```

#### 5.2. Working on a Branch
1.  **Commit frequently:** Make small, atomic commits with clear and descriptive messages.
2.  **Pull latest changes from `main` regularly:** To avoid large merge conflicts, periodically update your branch with changes from `main`.
    ```bash
    git pull origin main
    ```
    -   Resolve any merge conflicts that arise immediately.

#### 5.3. Updating Submodules within a Branch
If your work involves updating a submodule:
1.  **Navigate into the submodule directory:**
    ```bash
    cd path/to/your/submodule
    ```
2.  **Checkout the desired branch or commit within the submodule:**
    ```bash
    git checkout <submodule-branch-or-commit-hash>
    ```
3.  **Navigate back to the main project root:**
    ```bash
    cd -
    ```
4.  **Commit the submodule update in the main project:** This records the new submodule commit hash.
    ```bash
    git add path/to/your/submodule
    git commit -m "Update submodule <submodule-name> to <new-commit-hash>"
    ```

#### 5.4. Merging a Feature/Bugfix Branch into `main`
1.  **Ensure your branch is up-to-date with `main`:**
    ```bash
    git checkout <your-branch-name>
    git pull origin main
    ```
    -   Resolve any merge conflicts.
2.  **Ensure all tests pass and code reviews are complete.**
3.  **Switch to the `main` branch:**
    ```bash
    git checkout main
    ```
4.  **Merge your branch using `--no-ff` (no fast-forward) to preserve branch history:**
    ```bash
    git merge --no-ff <your-branch-name>
    ```
5.  **Push the `main` branch to the remote repository:**
    ```bash
    git push origin main
    ```
6.  **Delete the merged branch (optional, but recommended):**
    ```bash
    git branch -d <your-branch-name>
    git push origin --delete <your-branch-name>
    ```

### 6. Related SOPs
-   [SOP: Reviewing Changes in a Submodule](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/review_submodule_changes.md)
-   [Git Commit Message Guidelines (to be created)]
