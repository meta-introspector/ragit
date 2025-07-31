### **Refactoring and Build Finalization Plan**

**Objective:** Achieve a successful workspace build, prioritizing the `bootstrap` command and its self-improvement features. Other modules (muses, QA, other commands) are currently de-prioritized.

---

### **Current Status & Focus: Bootstrap Command**

The primary focus is on ensuring the `ragit bootstrap` command compiles and functions correctly, including its self-improvement loop.

**Key Progress:**
- `memory_monitor` argument correctly passed to `get_self_code`, `format_prompt`, and `handle_improved_code` in bootstrap crates.
- `ragit-index-query` updated to use `index.get_chunks()` for TF-IDF search.

**Next Steps for Bootstrap:**
1.  **Implement Compile and Test Logic:** Add logic within the `perform_self_improvement` loop to compile the improved code and run relevant tests.
2.  **Evaluate LLM Output:** Implement logic to evaluate the LLM's output and decide whether to continue the self-improvement loop.

---

### **De-prioritized Modules (Currently Broken)**

The following modules are currently broken and will be addressed in a later phase:
- **Muses (`ragit-muse`):** Compilation errors related to `ragit_prompt_management` and `tera` API usage.
- **QA Commands (`ragit-command-qa-test`, `ragit-command-qa-tune`):** Unresolved imports and method call issues.
- **Other Commands (`ragit-command-ls`, `ragit-agent-action`):** Various import and method call errors.
- **Code Understanding (`ragit-code-understanding`):** New crate created to house `tokei` integration and other code analysis tools, currently stubbed with `panic!`.

---

### **General Build & Verification**

1.  **Compile Workspace:**
    *   **Action:** Run `cargo build --workspace`.
    *   **Goal:** Achieve a successful compilation of the entire project, with a focus on the `bootstrap` command.

2.  **Iterative Debugging:**
    *   **Action:** If the build fails, analyze the compiler output, address the reported errors one by one, and re-run the build.
    *   **Goal:** Systematically eliminate all remaining compilation issues, prioritizing those affecting the `bootstrap` command.

3.  **Finalize and Commit:**
    *   **Action:** Once the build is successful, review the changes with `git status`, stage them with `git add .`, and create a comprehensive commit.
    *   **Goal:** Create a clean, stable checkpoint for the newly refactored codebase.