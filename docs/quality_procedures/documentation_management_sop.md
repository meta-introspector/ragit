# SOP: Documentation Management

## 1. Objective
To establish a clear and consistent process for creating, storing, and maintaining documentation within the `ragit` project, ensuring that all documentation is accurate, up-to-date, and easily accessible.

## 2. Scope
This SOP applies to all documentation within the `ragit` project, including but not limited to:

*   Architectural diagrams
*   Code documentation
*   Design documents
*   Process and procedure documents (SOPs)
*   User guides and tutorials

## 3. Procedure

### Phase 1: Creation and Storage
1.  **Consult the Index:** Before creating any new documentation, consult the `docs/index/index.md` file to ensure that similar documentation does not already exist.
2.  **Create New Documentation:** Create new documentation in Markdown format.
3.  **Store in Appropriate Location:** Store the new documentation in the appropriate subdirectory within the `docs/` directory. If a suitable subdirectory does not exist, create one.
4.  **Update the Index:** After creating the new documentation, add a link to it in the `docs/index/index.md` file.

### Phase 2: Maintenance and Review
1.  **Regular Review:** All documentation should be reviewed on a regular basis to ensure that it is accurate and up-to-date.
2.  **Update as Needed:** If any documentation is found to be inaccurate or out-of-date, it should be updated immediately.
3.  **Version Control:** All changes to documentation should be committed to the `ragit` Git repository with clear and descriptive commit messages.

## 4. Tools
*   `ragit` CLI
*   Git
*   Markdown editor

## 5. Key Locations and Procedures

To effectively manage documentation, it is crucial to be aware of the following key locations and procedures:

*   **Main Documentation Index:** `docs/index/index.md`
    *   **Purpose:** This is the central hub for all project documentation. Always consult this file before creating new documents to avoid duplication.
*   **Standard Operating Procedures (SOPs):** `docs/sops/`
    *   **Purpose:** This directory contains detailed instructions for common tasks and workflows. Familiarize yourself with these SOPs to ensure consistency and quality.
*   **Quality Assurance (QA) Procedures:** `docs/quality_procedures/`
    *   **Purpose:** This directory contains documents related to our quality assurance processes, including branching strategies, dependency management, and meta-programs.

### Core Principles:

*   **Hierarchical Search:** Always start with the highest-level information (e.g., the documentation index) and progressively narrow your search.
*   **One Declaration Per File:** This principle applies to code but also informs our documentation strategy, which is to have clear, focused documents for each topic.
*   **Vendor/Fork Dependencies:** We prioritize local control over external dependencies to ensure stability and security.
*   **Branching Strategy:** We follow a structured branching strategy to isolate work and ensure stability:
    *   **Feature Branches:** `feature/<descriptive-name>`
    *   **Bugfix Branches:** `bugfix/<issue-id>-<descriptive-name>`
    *   **Submodule Branches:** `<submodule-name>-<descriptive-name>`
*   **Meta-Programs:** We use meta-programs like "KitKat" and "GM" to manage our workflow and recover from interruptions.

## 6. Expected Outcome
*   A comprehensive and well-maintained set of documentation for the `ragit` project.
*   Improved knowledge sharing and collaboration among project members.
*   Reduced onboarding time for new project members.