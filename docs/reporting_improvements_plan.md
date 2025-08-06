# Reporting Improvements Plan for `term_quiz_master`

## 1. Current Problem

The per-commit output in the `term_quiz_master --update` command often displays repetitive or uninformative data, particularly in the "Top Term" and "Interesting Terms" sections. This leads to wasted screen space and reduces the immediate value of the progress report.

**Specific Issues:**
*   **Repetitive "Top Term":** Frequently reports common stopwords (e.g., "data", "path") as the top term, even when they are not semantically interesting.
*   **Sparse "Interesting Terms":** When a commit introduces few or no new terms, the "Interesting Terms" section remains empty or contains very few terms, leaving significant whitespace on the line.
*   **Lack of Global Uniqueness for "Interesting Terms":** While terms are unique per commit, there's no mechanism to ensure that a term reported as "interesting" on one line isn't reported again on a subsequent line, even if it's still "new" to that specific commit.

## 2. Goal

To maximize the information density and relevance of each per-commit output line, ensuring it provides unique and insightful terms, even when new terms are scarce. The output should be concise and fill the 80-character window effectively.

## 3. Proposed Solution (High-Level)

Implement a more sophisticated term selection strategy for the "Interesting Terms" and "Top Term" sections.

*   **Prioritize Non-Stopwords for "Top Term":** The "Top Term" should be the most frequent non-stopword term in the current commit. If all terms are stopwords, report "N/A".
*   **Dynamic "Interesting Terms" Selection:**
    1.  **Prioritize Truly New Terms:** First, fill the "Interesting Terms" section with terms that are *new to the entire run* (i.e., never seen before in any commit processed so far).
    2.  **Fallback to Unreported Existing Terms:** If the line is not yet full and there are no more *new* terms in the current commit, then select previously *unreported* terms from the overall `terms_map` (the global pool of all terms discovered so far) to fill the remaining space.
    3.  **Global Tracking of Reported Terms:** Maintain a `HashSet` of all terms that have ever been displayed in the "Interesting Terms" section to ensure they are not repeated in subsequent reports.
    4.  **Fill to Line Limit:** Continue adding terms until the 80-character line limit is approached, truncating if necessary.

## 4. Detailed Steps (Technical)

### 4.1. `crates/term_quiz_master/src/update_logic/run_update_command.rs`

*   Initialize `all_reported_terms: HashSet<String>` at the top level of `run_update_command`.
*   Pass `&mut all_reported_terms` to `repo_processor::process_repo_wrapper` (and subsequently to `process_commit` and `commit_reporter`).

### 4.2. `crates/term_quiz_master/src/update_logic/repo_processor.rs`

*   Update `process_repo_wrapper` signature to accept `all_reported_terms: &mut HashSet<String>`.
*   Pass `all_reported_terms` to `commit_reporter::report_commit_progress`.

### 4.3. `crates/term_quiz_master/src/update_logic/process_commit.rs`

*   Update `process_commit` signature to accept `all_reported_terms: &mut HashSet<String>`.
*   The function will return `(current_commit_term_counts, all_new_terms_in_commit)`. The selection of "interesting" terms will now happen in `commit_reporter`.

### 4.4. `crates/term_quiz_master/src/update_logic/commit_reporter.rs`

*   Update `report_commit_progress` signature to accept `all_reported_terms: &mut HashSet<String>`.
*   **Modify `top_term_info` logic:**
    *   Iterate through `current_commit_term_counts`.
    *   Find the term with the highest count that is *not* in `stopwords`.
    *   If no such term exists, set `top_term_info` to "Top: N/A".
*   **Modify "Interesting Terms" selection and formatting:**
    *   Initialize `interesting_terms_for_report: Vec<String>`.
    *   **Phase 1: New Terms:** Iterate through `all_new_terms_in_commit`. If a term is not in `all_reported_terms`, add it to `interesting_terms_for_report` and `all_reported_terms`. Stop when the line limit is approached or all new terms are processed.
    *   **Phase 2: Unreported Existing Terms (Fallback):** If `interesting_terms_for_report` is not yet full, iterate through `terms_map` (the global pool of all terms). If a term is not in `all_reported_terms`, add it to `interesting_terms_for_report` and `all_reported_terms`. Continue until the line limit is approached or no more unreported terms are available.
    *   Format `interesting_terms_for_report` into a string, ensuring it fits the remaining space on the `output_line`.

## 5. Expected Outcome

Each per-commit output line will be more informative, displaying:
*   Progress (commits processed, percentage).
*   Count of truly new terms in that commit.
*   The most frequent *non-stopword* term in that commit.
*   A selection of unique, previously unreported terms (prioritizing new terms from the current commit, then falling back to other unreported terms from the project), filling the line as much as possible.
