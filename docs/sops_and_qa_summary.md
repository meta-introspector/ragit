# SOPs and QA Procedures Summary

This document summarizes the key Standard Operating Procedures (SOPs) and Quality Assurance (QA) procedures for the ragit project.

## SOPs (`docs/sops/`)

*   **`code_archaeology.md`**: Before writing new code, search for existing functionality using a hierarchical approach (manifests, then targeted searches). Document findings and update the central index.
*   **`code_deduplication_sop.md`**: Use a three-phase approach: ingest all code into a `ragit` knowledge base, perform topological analysis to build a dependency graph, and then use this graph to identify and prioritize refactoring opportunities.
*   **`ECO-PAPER-9K.md`**: This is a philosophical framework for rigorous quality management, not a separate system. It integrates principles from ISO 9001, GMP, Six Sigma, C4, and ITIL into the existing workflow.
*   **`embedding_to_ontology_pipeline.md`**: A defined process for adding new terms to the ontology with semantic vector representations using `solfunmeme_embedding`, `solfunmeme_clifford`, and `solfunmeme_ontology_vibe`.
*   **`hierarchical_search_sop.md`**: A structured search strategy, starting with the highest-level information (tree, manifests) and progressively moving to more granular searches (docs, ontologies, source code).
*   **`sophia_integration.md`**: The `sophia` RDF library is abstracted behind the `solfunmeme_rdf_utils` crate to simplify its use and manage its complexity.

## Quality Procedures (`docs/quality_procedures/`)

*   **`branching_strategy_sop.md`**: Use feature branches for new work. For submodules, create branches within the submodule itself to isolate changes.
*   **`dependency_management_sop.md`**: Prioritize vendoring or forking external dependencies to ensure control, reproducibility, and security.
*   **`dwim_embedding_pipeline_work_log.md`**: A work log for the DYIM embedding pipeline, documenting progress, challenges, and decisions.
*   **`gemini_session_log.md`**: A log of Gemini CLI agent sessions.
*   **`gm_meta_program_sop.md`**: A recovery procedure after a reboot: stay on the critical path, review memories, and check recent commits.
*   **`kitkat_meta_program_sop.md`**: A strategic pause to document the current state, define a new plan, commit the work, and conceptually reboot.