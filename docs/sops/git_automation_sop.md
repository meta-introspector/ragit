# SOP: Git Automation and Scripting

## 1. Objective
To define principles and procedures for automating repetitive Git workflows within the `ragit` project, enhancing efficiency, consistency, and adherence to established branching and check-in strategies.

## 2. Scope
This SOP applies to all Git-related automation scripts and tools developed for the `ragit` project.

## 3. Principles of Git Automation

*   **Augment, Not Replace:** Automation should streamline workflows and reduce manual errors, but not replace critical human decision-making (e.g., complex merge conflict resolution).
*   **Safety First:** Scripts must be designed to be safe, with clear prompts and error handling to prevent unintended data loss or repository corruption.
*   **Consistency:** Automated processes must strictly adhere to the project's defined branching strategy, naming conventions, and commit procedures.
*   **Idempotence:** Where possible, scripts should be idempotent, meaning running them multiple times produces the same result as running them once.
*   **Documentation:** All automation scripts must be well-documented, explaining their purpose, usage, and any prerequisites.

## 4. Automated Workflows

### 4.1. Creating a New Feature/Bugfix Branch

This is a common, repetitive task that can be safely automated. The `scripts/git_new_branch.sh` script automates the process of ensuring the `main` branch is up-to-date and creating a new feature or bugfix branch.

**Script:** `scripts/git_new_branch.sh`

**Usage:**
```bash
./scripts/git_new_branch.sh <branch-type> <branch-name> [issue-id]
```
*   `<branch-type>`: `feature` or `bugfix`
*   `<branch-name>`: A descriptive name for the branch (e.g., `add-new-parser`)
*   `[issue-id]`: Optional, required for `bugfix` branches (e.g., `123`)

**Example:**
```bash
./scripts/git_new_branch.sh feature implement-caching
./scripts/git_new_branch.sh bugfix fix-login-error 123
```

### 4.2. Other Potential Automation Areas (Future Considerations)

*   **Regular Branch Updates:** While full automation of merge conflict resolution is complex, a script could automate `git pull origin main` and provide clear instructions for manual conflict resolution if needed.
*   **Submodule Update Workflow:** A script could guide the user through the process of updating submodules, prompting for commits within submodules and then updating the parent repository's reference.
*   **Pre-commit Hooks:** Implement Git pre-commit hooks to enforce code style, run linters, or execute basic tests before a commit is allowed.

## 5. Script Development Guidelines

*   **Shell Scripting:** Prefer simple shell scripts (`bash`) for straightforward automation tasks.
*   **Error Handling:** Include robust error handling (e.g., `set -e`, `exit 1` on failure).
*   **User Feedback:** Provide clear messages to the user about what the script is doing and any actions they need to take.
*   **Dependencies:** Clearly state any external dependencies required by the script.

## 6. Related SOPs
*   [SOP: Project Branching Strategy](./project_branching_strategy.md)
*   [SOP: Git Check-in Procedure for Recursive Submodules](./git_checkin_procedure.md)
