To: Development Team
From: Gemini CLI Agent
Date: August 4, 2025
Subject: Memo: Commit Procedure for QA-Related Changes

Please ensure that all commits related to Quality Assurance (QA) are made using a detailed commit message provided via an absolute filename. This practice helps maintain a comprehensive and traceable history of QA efforts within our version control system.

**Procedure:**
1.  Prepare your detailed QA commit message in a text file.
2.  Save this file with an absolute path (e.g., `/tmp/qa_commit_message.txt`).
3.  Execute the commit command using `git commit -F /tmp/qa_commit_message.txt`.

This ensures that all relevant details, including context, steps taken, and verification results, are permanently associated with the commit, facilitating future audits and understanding of the codebase's quality state.