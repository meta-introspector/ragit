# SOP: Documentation Storage and Management

## 1. Objective
To establish a standardized procedure for storing, organizing, and managing all project documentation, ensuring consistency, discoverability, and adherence to the "one declaration per file" principle for documentation.

## 2. Scope
This SOP applies to all documentation files within the `ragit` project, including but not limited to Markdown files (`.md`), ontology files (`.ttl`), and any other relevant textual or visual documentation.

## 3. Principles

*   **One Declaration Per File (Documentation Edition):** Each distinct concept, SOP, QA procedure, glossary term, or architectural component should ideally reside in its own dedicated Markdown file. This promotes modularity and ease of maintenance.
*   **Hierarchical Organization:** Documentation will be organized into a logical directory structure that mirrors the project's functional or conceptual hierarchy.
*   **Discoverability:** All documentation should be easily discoverable through a central index and consistent naming conventions.
*   **Version Control:** All documentation will be managed under Git version control, ensuring a complete history of changes.
*   **Clarity and Conciseness:** Documentation should be clear, concise, and to the point, avoiding unnecessary jargon.

## 4. Procedure

### 4.1. File Naming Conventions

*   **Lowercase and Hyphenated:** All file names should be lowercase and use hyphens (`-`) as word separators (e.g., `my-new-sop.md`, `glossary-term-example.md`).
*   **Descriptive:** File names should be descriptive of their content.
*   **File Extensions:** Use appropriate file extensions (e.g., `.md` for Markdown, `.ttl` for Turtle ontologies).

### 4.2. Directory Structure

All documentation will reside within the `docs/` directory at the project root. Subdirectories will be created to categorize documentation logically.

*   **`docs/index/`**: Contains the main documentation index (`index.md`) and a comprehensive glossary (`glossary.md`), along with individual glossary term files (`glossary_terms/`).
*   **`docs/sops/`**: Contains all Standard Operating Procedures.
*   **`docs/quality_procedures/`**: Contains all Quality Assurance procedures.
*   **`docs/architecture/`**: Contains architectural overviews and design documents.
*   **`docs/commands/`**: Documentation specific to CLI commands.
*   **`docs/issues/`**: Documentation related to specific issues or bug fixes.
*   **`docs/memos/`**: Internal memos or informal notes.
*   **`docs/rust_code/`**: Documentation specific to Rust code components or crates.
*   **`docs/submodules/`**: Documentation related to vendored or submoduled projects.
*   **`docs/chats/`**: Transcripts or summaries of important chat conversations.
*   **`docs/change_log/`**: Records of significant changes.

### 4.3. Central Indexing

*   **`docs/index/index.md`**: This file serves as the central entry point for all documentation. It will contain a hierarchical list of all major documentation sections and links to individual documents.
*   **`docs/index/glossary.md`**: This file will list all defined glossary terms and link to their individual definitions in `docs/index/glossary_terms/`.

### 4.4. Content Guidelines

*   **Markdown Format:** All textual documentation will be written in GitHub-flavored Markdown.
*   **Clear Headings:** Use clear and consistent headings to structure content.
*   **Internal Linking:** Use relative links to connect related documents within the `docs/` directory.
*   **Metadata (Optional but Recommended):** For certain documents (e.g., SOPs, QA procedures), consider including a small metadata block at the top with information like objective, scope, and version.

## 5. Maintenance

*   **Regular Review:** Documentation should be regularly reviewed and updated to reflect changes in the codebase or procedures.
*   **Version Control:** All changes to documentation must be committed to Git.
*   **Automated Checks (Future):** Explore automated tools for link validation and consistency checks.

## 6. Expected Outcome
*   A well-organized and easily navigable documentation system.
*   Improved consistency and quality of project documentation.
*   Reduced effort in finding and understanding project information.
*   Enhanced adherence to project standards and principles.
