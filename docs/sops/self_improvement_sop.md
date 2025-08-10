# SOP: Gemini CLI Agent Self-Improvement Process

## 1. Purpose

To define a structured and iterative process for the Gemini CLI agent to continuously improve the `ragit` system, enhancing its capabilities, refining its models, and expanding its knowledge base.

## 2. Scope

This SOP applies to all tasks undertaken by the Gemini CLI agent aimed at enhancing the `ragit` project, including but not limited to:
*   Refinement of the embedding model.
*   Expansion and organization of the knowledge base.
*   Development and improvement of internal tools and utilities.
*   Integration of new data sources and semantic understanding.

## 3. Procedure

### 3.1. Identify Areas for Improvement

The agent will actively seek opportunities for improvement through various mechanisms:
*   **Quiz Results Analysis**: Reviewing the outcomes of `ragit-embedding-quiz` sessions, particularly focusing on terms with high distances to similar embeddings or terms requiring manual embedding input.
*   **User Feedback**: Analyzing direct feedback and requests from the user regarding `ragit`'s performance, accuracy, or missing functionalities.
*   **`suggest-terms` Output**: Regularly reviewing the output of the `ragit-embedding-quiz suggest-terms` command to identify new or under-represented concepts.
*   **Codebase Analysis**: Proactively identifying areas in the `ragit` codebase that could benefit from refactoring, optimization, or new features.
*   **Ontology Review**: Identifying gaps or inconsistencies in the project's ontologies.

### 3.2. Plan Improvements

Once an area for improvement is identified, the agent will formulate a concise and actionable plan:
*   **Define Objective**: Clearly state what needs to be achieved.
*   **Consult Existing Documentation**: Review relevant SOPs, architectural documents, and code to understand the current state and conventions.
*   **Propose Solution**: Outline the steps required to implement the improvement, including any necessary code modifications, new tool development, or data adjustments.
*   **Prioritize**: Assess the impact and feasibility of the proposed changes.

### 3.3. Implement Changes

The agent will execute the planned improvements using its available tools:
*   **Code Modification**: Utilize `read_file`, `write_file`, and `replace` to modify source code files.
*   **New Tool Development**: Create new Rust crates or extend existing ones (e.g., `ragit-embedding-quiz`) to introduce new functionalities.
*   **Data Adjustment**: Modify data files (e.g., `term_embeddings.json`, `training_data.json`) as required by the improvement plan.
*   **Shell Commands**: Execute `run_shell_command` for tasks like `cargo build`, `cargo run`, `git add`, `git commit`.

### 3.4. Verify and Test

After implementing changes, the agent will rigorously verify their correctness and effectiveness:
*   **Automated Testing**: Run relevant `cargo test` commands to ensure no regressions are introduced.
*   **Functional Testing**: Manually test new or modified functionalities (e.g., running `ragit-embedding-quiz` commands and observing output).
*   **Performance Monitoring**: Assess the impact of changes on performance and resource usage.
*   **Semantic Evaluation**: For embedding-related improvements, re-evaluate the semantic relevance of similar terms.

### 3.5. Document

All significant changes and improvements will be thoroughly documented:
*   **Update Existing SOPs**: Modify relevant SOPs to reflect new procedures or tool usage.
*   **Create New SOPs**: Develop new SOPs for entirely new processes or tools.
*   **Update Central Index**: Ensure `docs/index/index.md` is updated with links to new or modified documentation.
*   **Commit Changes**: Use `git add` and `git commit` with descriptive messages to track all modifications.

### 3.6. Iterate

Self-improvement is a continuous cycle. The agent will regularly revisit previous steps, analyze new data, and identify further opportunities for enhancement, ensuring the `ragit` system remains at the forefront of its capabilities.

## 4. Tools

*   `ragit-embedding-quiz` (for embedding refinement and term suggestion)
*   `git` (for version control and change tracking)
*   `read_file`, `write_file`, `replace` (for code modification)
*   `run_shell_command` (for executing build, test, and other shell commands)
*   `glob`, `search_file_content` (for code and content discovery)

## 5. Expected Outcome

A continuously evolving, more intelligent, and highly capable `ragit` system, driven by an autonomous and self-improving Gemini CLI agent.