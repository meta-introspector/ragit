# SOP: Git Check-in Procedure for Recursive Submodules

## 1. Purpose
This procedure outlines the steps for consistently checking in changes within a Git repository that utilizes recursive submodules, ensuring all modifications are properly committed and pushed to matching branches.

## 2. Scope
This SOP applies to all development work involving the main `ragit` repository and its submodules.

## 3. Procedure

### 3.1. Identify Modified Submodules
Before committing changes in the main repository, identify all submodules that have local modifications or new commits.

```bash
git status
```
Look for lines indicating `modified: <submodule_path> (modified content)` or `(new commits)`. Make a note of these submodule paths.

### 3.2. Check Out Matching Branches in Modified Submodules
For each modified submodule, navigate into its directory and ensure it is on a branch that logically matches the parent repository's current branch. If the submodule is on a detached HEAD or a different branch, switch to or create a matching branch.

```bash
cd <submodule_path>
git status # Identify current branch or detached HEAD
git checkout <matching_branch_name> # If not already on it, or create if necessary
git pull origin <matching_branch_name> # Ensure it's up-to-date
cd ..
```

### 3.3. Commit Changes Within Submodules
For each modified submodule identified in Step 3.1, navigate into its directory, stage all relevant changes, and commit them. Use clear and concise commit messages.

```bash
cd <submodule_path>
git add .
# Review changes before committing (optional but recommended)
git diff --staged
# Commit changes
git commit -F /data/data/com.termux/files/home/storage/github/ragit/temp_commit_message.txt # Use the standard commit message file
git push # Push submodule changes to its remote
cd ..
```
Repeat this for all modified submodules.

### 3.4. Update Parent Repository's Submodule References
After committing and pushing changes in all submodules, the parent repository will recognize that its submodule references have changed. These changes need to be staged and committed in the parent repository.

```bash
git status # Verify submodules are now listed as 'modified: <submodule_path> (new commits)'
git add <submodule_path_1> <submodule_path_2> ... # Add all modified submodules
# Review changes before committing (optional but recommended)
git diff --staged
# Commit the updated submodule references in the parent repository
git commit -F /data/data/com.termux/files/home/storage/github/ragit/temp_commit_message.txt # Use the standard commit message file
```

### 3.5. Push All Changes
Finally, push the changes from the parent repository to its remote. This will also push the updated submodule references.

```bash
git push
```
If the current branch does not have an upstream branch set, use:
```bash
git push --set-upstream origin <current_branch_name>
```

## 4. Verification
After completing the procedure, run `git status` in the main repository to ensure a clean working directory and that all changes have been pushed.

```bash
git status
```
Expected output: `nothing to commit, working tree clean` and `Your branch is up to date with 'origin/<branch_name>'`.

## 5. Related SOPs
- [SOP: Git Commit Message Guidelines](link_to_commit_message_sop_if_exists)
- [SOP: Branching Strategy](link_to_branching_strategy_sop_if_exists)
