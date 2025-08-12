#!/bin/bash

# SOP: Git Automation - New Branch Creation
# Objective: Automate the process of ensuring the main branch is up-to-date
# and creating a new feature or bugfix branch.

# Usage:
# ./scripts/git_new_branch.sh <branch-type> <branch-name> [issue-id]
# <branch-type>: feature or bugfix
# <branch-name>: A descriptive name for the branch (e.g., add-new-parser)
# [issue-id]: Optional, required for bugfix branches (e.g., 123)

set -e

BRANCH_TYPE=$1
BRANCH_NAME=$2
ISSUE_ID=$3

if [ -z "$BRANCH_TYPE" ] || [ -z "$BRANCH_NAME" ]; then
    echo "Usage: $0 <branch-type> <branch-name> [issue-id]"
    echo "  <branch-type>: feature or bugfix"
    echo "  <branch-name>: A descriptive name for the branch (e.g., add-new-parser)"
    echo "  [issue-id]: Optional, required for bugfix branches (e.g., 123)"
    exit 1
fi

if [ "$BRANCH_TYPE" == "bugfix" ] && [ -z "$ISSUE_ID" ]; then
    echo "Error: For 'bugfix' branch type, an issue-id is required."
    echo "Usage: $0 bugfix <branch-name> <issue-id>"
    exit 1
fi

# Ensure main is up-to-date
echo "Ensuring main branch is up-to-date..."
git checkout main
git pull origin main

# Construct the new branch name
NEW_BRANCH_FULL_NAME=""
if [ "$BRANCH_TYPE" == "feature" ]; then
    NEW_BRANCH_FULL_NAME="feature/$BRANCH_NAME"
elif [ "$BRANCH_TYPE" == "bugfix" ]; then
    NEW_BRANCH_FULL_NAME="bugfix/$ISSUE_ID-$BRANCH_NAME"
else
    echo "Error: Invalid branch type. Must be 'feature' or 'bugfix'."
    exit 1
fi

# Create and switch to the new branch
echo "Creating and switching to new branch: $NEW_BRANCH_FULL_NAME..."
git checkout -b "$NEW_BRANCH_FULL_NAME"

echo "Successfully created and switched to branch: $NEW_BRANCH_FULL_NAME"
