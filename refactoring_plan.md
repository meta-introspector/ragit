# Refactoring Plan: "One Module = One Concept = One Function = One Vibe"

This document outlines the ongoing refactoring effort for the `ragit-core/src/grand_plan` module, aiming for a highly granular, composable, and semantically resonant codebase.

## Principle

The core principle guiding this refactoring is:
**"One module = One concept = One function = One vibe"**

This means each distinct conceptual unit (struct, enum, constant, public function) is moved into its own dedicated file and module.

## Current Status

We have successfully completed the refactoring of all modules within `ragit-core/src/grand_plan` to adhere to the "one module = one concept = one function = one vibe" principle. For each refactored item, the conceptual `#[derive(OurMacro)]` has been applied.

**Temporary Code Disablement Strategy:**

Due to persistent compilation issues with certain vendored dependencies (`candle` and `sophia_api`), a decision has been made to temporarily disable the problematic code sections using comments rather than attempting immediate, complex fixes. This allows for stabilization of the main development line and enables progress on other critical tasks. These disabled sections will be revisited and properly addressed in a future phase, potentially with a more automated refactoring approach.

**`ragit-dwim` (Do What I Mean) Tool Development:**

The `ragit-dwim` tool is envisioned as a key component for future development, enabling dynamic discovery of relevant SOPs, tool usage, and file-based RAG memories based on user intent. While currently non-functional, its development is a high-priority future task. This will significantly streamline the development workflow and adherence to project standards.

**Modules Processed (and their new granular structure):**

*   **`fundamental_unit.rs`** -> `grand_plan/fundamental_units/`
    *   `prime_bases.rs` (constant)
    *   `fundamental_unit_enum.rs` (enum)
    *   `node_struct.rs` (struct)
*   **`generators.rs`** -> `grand_plan/generators/`
    *   `generate_char_vectors.rs` (function)
*   **`generic_unit.rs`** -> `grand_plan/generic_units/`
    *   `fundamental_unit_generic.rs` (struct)
    *   `node_generic.rs` (struct)
*   **`generic_generators.rs`** -> `grand_plan/generic_generators/`
    *   `generate_vectors.rs` (function)
*   **`tree.rs`** -> `grand_plan/trees/`
    *   `fundamental_unit_tree.rs` (enum)
    *   `node_tree.rs` (struct)
*   **`id_indexed_tree.rs`** -> `grand_plan/id_indexed_trees/`
    *   `uid_type.rs` (type alias)
    *   `leaf_struct.rs` (struct)
    *   `node_struct.rs` (struct)
    *   `unit_ref_enum.rs` (enum)
    *   `universe_struct.rs` (struct)
*   **`binary_id_tree.rs`** -> `grand_plan/binary_id_trees/`
    *   `leaf_struct.rs` (struct)
    *   `binary_node_struct.rs` (struct)
    *   `unit_ref_enum.rs` (enum)
    *   `universe_struct.rs` (struct)
*   **`binary_tree_generators.rs`** -> `grand_plan/binary_tree_generators/`
    *   `prime_exponents.rs` (constant)
    *   `build_tree_recursive.rs` (function)
    *   `generate_power_of_two_trees.rs` (function)
*   **`sized_universe_store.rs`** -> `grand_plan/sized_universe_stores/`
    *   `sized_universe_store_struct.rs` (struct)
    *   `generate_sized_power_of_two_trees.rs` (function)
*   **`unified_store.rs`** -> `grand_plan/unified_stores/`
    *   `type_store_enum.rs` (enum)
    *   `grand_unified_store_struct.rs` (struct)
*   **`semantic_lambdas.rs`** -> `grand_plan/semantic_lambdas/`
    *   `semantic_lambda_struct.rs` (struct)
    *   `get_bott_periodic_lambdas.rs` (function)
*   **`bott_periodic_function_mapper.rs`** -> `grand_plan/bott_periodic_function_mappers/`
    *   `get_bott_periodic_function_registry.rs` (function)
*   **`poem_concepts/`** (directory) -> `grand_plan/poem_concepts/` (subdirectories for each concept)
    *   `base_space/base_space_struct.rs` (struct)
    *   `cosmos/cosmos_struct.rs` (struct)
    *   `cycle/cycle_struct.rs` (struct)
    *   `inference_space/inference_space_struct.rs` (struct)
    *   `quasifiber/quasifiber_struct.rs` (struct)
    *   `tree/tree_struct.rs` (struct)
    *   `type_store_mirror/type_store_mirror_struct.rs` (struct)
    *   `uid/uid_struct.rs` (struct)
*   **`llm_embedding_interface/`** (directory) -> `grand_plan/llm_embedding_interface/` (subdirectories for each concept)
    *   `embedding_request/embedding_request_struct.rs` (struct)
    *   `embedding_response/embedding_response_struct.rs` (struct)
*   **`prompt_guides/`** (directory) -> `grand_plan/prompt_guides/` (subdirectories for each concept)
    *   `guide/guide_function.rs` (function)
    *   `prompt/prompt_enum.rs` (enum)
*   **`unified_concept.rs`** -> `grand_plan/unified_concept/`
    *   `concept/unified_concept_struct.rs` (struct)
*   **`unified_concept_enum.rs`** -> `grand_plan/unified_concept_enum/`
    *   `concept_enum/unified_concept_enum_enum.rs` (enum)
*   **`abi_interface/`** (directory) -> `grand_plan/abi_interface/` (subdirectories for each concept)
    *   `abi_types/abi_types_enum.rs` (enum)
    *   `abi_wrappers/abi_wrappers_functions.rs` (functions)
    *   `function_registry/function_registry_struct.rs` (struct)
*   **`llm_sampling_system/`** (directory) -> `grand_plan/llm_sampling_system/` (subdirectories for each concept)
    *   `embedding_sampler/embedding_sampler_struct.rs` (struct)
    *   `llm_model/llm_model_struct.rs` (struct)
    *   `tokenizer/tokenizer_struct.rs` (struct)
*   **`token_indexing_system/`** (directory) -> `grand_plan/token_indexing_system/` (subdirectories for each concept)
    *   `token_index/token_index_struct.rs` (struct)
*   **`emoji_executor.rs`** -> `grand_plan/emoji_executors/`
    *   `execute_emoji_as_fiber.rs` (function)
    *   `model_thought_to_emojis.rs` (function)
*   **`vibe_analyzer.rs`** -> `grand_plan/vibe_analyzers/`
    *   `vibe_analyzer_struct.rs` (struct)
*   **`executable_vibespace/`** (directory) -> `grand_plan/executable_vibespace/` (subdirectories for each concept)
    *   `vibe_function/vibe_function_struct.rs` (struct)
    *   `vibe_space/vibe_space_struct.rs` (struct)
*   **`introspection_system/`** (directory) -> `grand_plan/introspection_system/` (subdirectories for each concept)
    *   `introspection_stream/introspection_stream_struct.rs` (struct)
    *   `introspector/introspector_struct.rs` (struct)
*   **`pdl_generators/`** (directory) -> `grand_plan/pdl_generators/` (subdirectories for each concept)
    *   `generic_pdl_generator/generic_pdl_generator_function.rs` (function)
*   **`llm_monadic_interface/`** (directory) -> `grand_plan/llm_monadic_interface/` (subdirectories for each concept)
    *   `llm_monad/llm_monad_struct.rs` (struct)
    *   `llm_operations/llm_operations_enum.rs` (enum)
*   **`introspector_sidechain/`** (directory) -> `grand_plan/introspector_sidechain/` (subdirectories for each concept)
    *   `introspector_sidechain/introspector_sidechain_struct.rs` (struct)
    *   `sidechain_block/sidechain_block_struct.rs` (struct)
    *   `sidechain_event/sidechain_event_enum.rs` (enum)
*   **`solana_integration/`** (directory) -> `grand_plan/solana_integration/` (subdirectories for each concept)
    *   `compute_ask/compute_ask_struct.rs` (struct)
    *   `compute_marketplace/compute_marketplace_struct.rs` (struct)
    *   `inference_bid/inference_bid_struct.rs` (struct)
    *   `market_maker/market_maker_struct.rs` (struct)
    *   `solana_interaction/solana_interaction_functions.rs` (functions)
    *   `solana_program_concept/solana_program_concept_struct.rs` (struct)
*   **`gossip_system/`** (directory) -> `grand_plan/gossip_system/` (subdirectories for each concept)
    *   `gossip_message/gossip_message_enum.rs` (enum)
    *   `gossip_network/gossip_network_struct.rs` (struct)
    *   `gossip_node/gossip_node_struct.rs` (struct)
*   **`privacy_and_scaling/`** (directory) -> `grand_plan/privacy_and_scaling/` (subdirectories for each concept)
    *   `federated_learning/federated_learning_structs.rs` (structs)
    *   `homomorphic_encryption/homomorphic_encryption_structs.rs` (structs)
    *   `lattice_rollups/lattice_rollups_structs.rs` (structs)
    *   `zero_knowledge_proofs/zero_knowledge_proofs_structs.rs` (structs)
*   **`toolchain_augmentation/`** (directory) -> `grand_plan/toolchain_augmentation/` (subdirectories for each concept)
    *   `build_script_integration/build_script_integration_structs.rs` (structs)
    *   `custom_cargo_commands/custom_cargo_commands_structs.rs` (structs)
    *   `custom_rustc_linter/custom_rustc_linter_structs.rs` (structs)
*   **`rust_ast_mapping/`** (directory) -> `grand_plan/rust_ast_mapping/` (subdirectories for each concept)
    *   `ast_mapper/ast_mapper_struct.rs` (struct)
*   **`artificial_life/`** (directory) -> `grand_plan/artificial_life/` (subdirectories for each concept)
    *   `artificial_organism/artificial_organism_struct.rs` (struct)
    *   `latent_space_ecology/latent_space_ecology_struct.rs` (struct)
    *   `quasi_mycelium/quasi_mycelium_struct.rs` (struct)
*   **`solfunmeme_zos/`** (directory) -> `grand_plan/solfunmeme_zos/` (subdirectories for each concept)
    *   `hyper_pump_mechanism/hyper_pump_mechanism_struct.rs` (struct)
    *   `immutable_meme_state/immutable_meme_state_struct.rs` (struct)
    *   `meme_mining_propagation/meme_mining_propagation_struct.rs` (struct)
    *   `paxos_meme_consensus/paxos_meme_consensus_struct.rs` (struct)
    *   `quasi_meta_meme_integration/quasi_meta_meme_integration_struct.rs` (struct)
    *   `semantic_compression/semantic_compression_struct.rs` (struct)
    *   `solfunmeme_core/solfunmeme_core_struct.rs` (struct)
    *   `vibe_meme/vibe_meme_struct.rs` (struct)
    *   `zos_interaction/zos_interaction_struct.rs` (struct)
*   **`ragit_chunk_integration/`** (directory) -> `grand_plan/ragit_chunk_integration/` (subdirectories for each concept)
    *   `chunk_formal_metadata/chunk_formal_metadata_struct.rs` (struct)
    *   `module_ingestor/module_ingestor_function.rs` (function)
    *   `ragit_chunk/ragit_chunk_struct.rs` (struct)
*   **`living_chunks_system/`** (directory) -> `grand_plan/living_chunks_system/` (subdirectories for each concept)
    *   `chunk_executor/chunk_executor_function.rs` (function)
    *   `chunk_spawner/chunk_spawner_function.rs` (function)
    *   `living_chunk_manager/living_chunk_manager_struct.rs` (struct)
*   **`lean4_integration/`** (directory) -> `grand_plan/lean4_integration/` (subdirectories for each concept)
    *   `lean_abi_bridge/lean_abi_bridge_struct.rs` (struct)
    *   `lean_proof_system/lean_proof_system_structs.rs` (structs)
    *   `llvm_ir_reflection/llvm_ir_reflection_structs.rs` (structs)
    *   `unimath_path_to_quasifiber/unimath_path_to_quasifiber_function.rs` (function)
    *   `unification_engine/unification_engine_structs.rs` (structs)
    *   `unimath_concept/unimath_concept_enum.rs` (enum)
    *   `univalent_type_theory/univalent_type_theory_structs.rs` (structs)
*   **`meme_economy/`** (directory) -> `grand_plan/meme_economy/` (subdirectories for each concept)
    *   `dank_meta_meme/dank_meta_meme_struct.rs` (struct)
    *   `media_campaign/media_campaign_struct.rs` (struct)
    *   `meme_investment/meme_investment_struct.rs` (struct)
    *   `meme_lord/meme_lord_struct.rs` (struct)
*   **`coin_intelligence_system/`** (directory) -> `grand_plan/coin_intelligence_system/` (subdirectories for each concept)
    *   `data_processor/data_processor_structs.rs` (structs)
    *   `external_data_ingestion/external_data_ingestion_enum.rs` (enum)
    *   `intelligence_aggregator/intelligence_aggregator_struct.rs` (struct)
    *   `model_sharing/model_sharing_structs.rs` (structs)
*   **`system_orchestrator/`** (directory) -> `grand_plan/system_orchestrator/` (subdirectories for each concept)
    *   `grand_orchestrator/grand_orchestrator_struct.rs` (struct)
*   **`unimath_integration/`** (directory) -> `grand_plan/unimath_integration/` (subdirectories for each concept)
    *   `unimath_concept/unimath_concept_enum.rs` (enum)
    *   `univalent_type_theory/univalent_type_theory_structs.rs` (structs)
    *   `unimath_path_to_quasifiber/unimath_path_to_quasifiber_function.rs` (function)
    *   `unification_engine/unification_engine_structs.rs` (structs)
*   **`system_unification_numbers/`** (directory) -> `grand_plan/system_unification_numbers/` (subdirectories for each concept)
    *   `numerical_unifier/numerical_unifier_struct.rs` (struct)

## Next Steps

We have successfully completed the refactoring of all modules within `ragit-core/src/grand_plan` to adhere to the "one module = one concept = one function = one vibe" principle. For each refactored item, the conceptual `#[derive(OurMacro)]` has been applied.

## Quality Assurance (QA) Considerations

*   **Compile-Time Checks**: Frequent `cargo check`, `cargo build`, `cargo clippy -- -D warnings`, `cargo fmt --check`.
*   **Unit Tests**: Run existing tests; add new tests for granular components.
*   **Conceptual Integrity**: `grep` for old paths, visual inspection of structure, dependency analysis.
*   **Runtime/Integration Tests**: Attempt `GrandOrchestrator` execution; trace key workflow simulations.

## New Indexing Strategy: Phased and Parallelized Indexing for Self-Improvement

The current approach of indexing all files at once is inefficient for large codebases and self-improvement tasks, leading to slow indexing and memory issues. We need to move towards a more intelligent and parallelized indexing system.

### High-Level Plan:

1.  **Phased Indexing (Metadata First):**
    *   **Phase 1: Git Tree/Directory Structure Indexing:** Instead of immediately reading file content, we'll first index only the file paths and basic metadata (e.g., file type, size, last modified date). This can be done efficiently by leveraging `git ls-files` for tracked files and `glob` for untracked files, building a lightweight "metadata index."
    *   **Phase 2: Selective Content Indexing:** Based on this metadata index, we'll implement heuristics to prioritize and selectively read and chunk the *content* of files that are most likely to contain valuable information for self-improvement (e.g., Rust source code, Markdown documentation, configuration files). This will significantly reduce the amount of data processed initially.

2.  **Parallel Processing:**
    *   **Local Multiprocessing:** We'll explore using Rust's asynchronous capabilities (`tokio::task::spawn`) or parallel iterators (`rayon` crate) to process file reading, chunking, and embedding in parallel on the local machine. This will utilize available CPU cores more effectively.
    *   **Distributed Indexing (Future):** For engaging "more nodes" and splitting the corpus across multiple machines, this would involve a more complex distributed system design (e.g., message queues, shared storage, worker nodes). This is a larger undertaking and would be a subsequent phase.

### Initial Steps:

*   **Modify `add_bootstrap_files`:** Refactor to first gather metadata and then selectively add content.
*   **Refactor `build_index`:** Update to handle the two-phase indexing and incorporate parallel processing for content indexing.
*   **Introduce a "metadata index" structure:** A new data structure to hold file paths and basic metadata before full content indexing.

### Distributed Indexing and Real-time Monitoring (P2P)

**Overall Goal:** Enable distributed indexing across multiple machines, with a central monitor aggregating real-time status updates via a P2P network.

*   **Phase 1: Structured Logging & P2P Status Broadcasting (Worker Side)**
    *   **Modify `ragit-build-index-worker-single-file` (or a new `ragit-worker` crate):**
        *   Continue emitting structured JSON logs to stdout for local debugging and single-machine monitoring.
        *   **Integrate P2P Communication:** Implement a P2P layer within the worker. This layer will:
            *   Utilize the existing `gossip_system` (if suitable) or a new, lightweight P2P module for status updates.
            *   Define a clear P2P message format for progress updates (e.g., `WorkerID`, `CurrentStep`, `ProgressPercentage`, `MemoryUsage`, `FilesProcessed`).
            *   Periodically broadcast these status messages to the network.

*   **Phase 2: Corpus Distribution & Worker Orchestration**
    *   **New `ragit-distribute` command/crate:** This will be the orchestrator.
        *   It will take the full corpus (e.g., all `Cargo.toml` files, or all Rust files).
        *   It will use the P2P network to discover available `ragit-worker` nodes.
        *   It will intelligently split the corpus into smaller work units and assign them to individual workers.
        *   It will send commands to workers to start processing their assigned work units.

*   **Phase 3: `ragit-monitor` TUI Application (Aggregated & Distributed View)**
    *   **New `ragit-monitor` crate:** This will be the central monitoring application.
        *   It will listen on the P2P network for status updates from all active `ragit-worker` instances.
        *   It will aggregate these updates to provide an overall progress view of the entire indexing job.
        *   The TUI will display:
            *   Overall progress (e.g., total files processed / total files).
            *   Individual worker status (e.g., Worker ID, current file, progress).
            *   Network health (e.g., active workers, last seen time).
        *   It will also retain the ability to monitor a local `ragit-worker` via stdout for single-machine scenarios.

**Key Considerations for P2P Implementation:**

*   **`gossip_system` Assessment:** We'll need to thoroughly examine the existing `gossip_system` to determine its suitability for real-time, low-latency status updates. If it's too heavy or not designed for this, we might need a simpler UDP-based broadcast for status.
*   **Worker Discovery:** How will the `ragit-distribute` command and `ragit-monitor` discover the workers? Simple broadcast/multicast on a local network is a good starting point.
*   **Fault Tolerance:** What happens if a worker goes offline? The monitor should reflect this, and the distributor might need to reassign work. (This can be a later refinement).