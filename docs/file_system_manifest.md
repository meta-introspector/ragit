# RAGIT Project File System Manifest

This document provides a hierarchical overview of the `ragit` project's file system structure. It serves as a living manifest to aid in understanding file organization, locating specific components, and planning future reorganizations.

Each entry represents a file or directory, providing a snapshot of the project's current layout. This manifest is crucial for maintaining awareness of our codebase's topology and ensuring that new additions or refactorings adhere to our architectural principles.

---

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── RelNotes
│   ├── 0.2.1.md
│   ├── 0.3.0.md
│   ├── 0.3.1.md
│   ├── 0.3.2.md
│   ├── 0.3.3.md
│   ├── 0.3.4.md
│   ├── 0.3.5.md
│   ├── 0.4.0.md
│   └── 0.4.1.md
├── change_control_document.md
├── commit_message.txt
├── comms
│   └── gemini
│       ├── inbox
│       └── outbox
│           └── 1754334849770_message.md
├── crates
│   ├── layer1_physical
│   │   ├── ragit-core
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       ├── grand_plan
│   │   │       │   ├── abi_interface
│   │   │       │   │   ├── abi_types
│   │   │       │   │   │   ├── abi_types_enum.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── abi_wrappers
│   │   │       │   │   │   ├── abi_wrappers_functions.rs
│   │   │       │   │   │   ├── helpers
│   │   │       │   │   │   │   ├── mod.rs
│   │   │       │   │   │   │   ├── to_abi_value_char.rs
│   │   │       │   │   │   │   ├── to_abi_value_grand_unified_store.rs
│   │   │       │   │   │   │   ├── to_abi_value_sized_universe_store.rs
│   │   │       │   │   │   │   ├── to_abi_value_type_store.rs
│   │   │       │   │   │   │   ├── to_abi_value_u32.rs
│   │   │       │   │   │   │   └── to_abi_value_universe.rs
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   └── wrappers
│   │   │       │   │   │       ├── mod.rs
│   │   │       │   │   │       ├── wrap_the_cosmos.rs
│   │   │       │   │   │       ├── wrap_the_cycle.rs
│   │   │       │   │   │       ├── wrap_the_mirror.rs
│   │   │       │   │   │       ├── wrap_the_pair.rs
│   │   │       │   │   │       ├── wrap_the_quasifiber.rs
│   │   │       │   │   │       ├── wrap_the_spark.rs
│   │   │       │   │   │       ├── wrap_the_tree.rs
│   │   │       │   │   │       └── wrap_the_void.rs
│   │   │       │   │   ├── function_registry
│   │   │       │   │   │   ├── function_registry_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── artificial_life
│   │   │       │   │   ├── artificial_organism
│   │   │       │   │   │   ├── artificial_organism_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── latent_space_ecology
│   │   │       │   │   │   ├── latent_space_ecology_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   └── quasi_mycelium
│   │   │       │   │       ├── mod.rs
│   │   │       │   │       └── quasi_mycelium_struct.rs
│   │   │       │   ├── binary_id_trees
│   │   │       │   │   ├── binary_node_struct.rs
│   │   │       │   │   ├── leaf_struct.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── unit_ref_enum.rs
│   │   │       │   │   └── universe_struct.rs
│   │   │       │   ├── binary_tree_generators
│   │   │       │   │   ├── build_tree_recursive.rs
│   │   │       │   │   ├── generate_power_of_two_trees.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   └── prime_exponents.rs
│   │   │       │   ├── bootstrapper
│   │   │       │   │   ├── bootstrap_orchestrator.rs
│   │   │       │   │   ├── initializers
│   │   │       │   │   │   ├── initialize_bootstrap_components.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   └── phases
│   │   │       │   │       ├── ingest_data.rs
│   │   │       │   │       ├── ingest_modules.rs
│   │   │       │   │       ├── initialize_memetic_ecosystem.rs
│   │   │       │   │       ├── mod.rs
│   │   │       │   │       └── simulate_hype_cycle.rs
│   │   │       │   ├── bott_periodic_function_mappers
│   │   │       │   │   ├── get_bott_periodic_function_registry.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── bott_periodic_lambdas
│   │   │       │   │   ├── lambda_0_the_void.rs
│   │   │       │   │   ├── lambda_1_the_spark.rs
│   │   │       │   │   ├── lambda_2_the_pair.rs
│   │   │       │   │   ├── lambda_3_the_tree.rs
│   │   │       │   │   ├── lambda_4_the_cosmos.rs
│   │   │       │   │   ├── lambda_5_the_mirror.rs
│   │   │       │   │   ├── lambda_6_the_quasifiber.rs
│   │   │       │   │   ├── lambda_7_the_cycle.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── coin_intelligence_system
│   │   │       │   │   ├── data_processor
│   │   │       │   │   │   ├── data_processor_structs.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── external_data_ingestion
│   │   │       │   │   │   ├── external_data_ingestion_enum.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── intelligence_aggregator
│   │   │       │   │   │   ├── intelligence_aggregator_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   └── model_sharing
│   │   │       │   │       ├── mod.rs
│   │   │       │   │       └── model_sharing_structs.rs
│   │   │       │   ├── conceptual_loops
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── ooda_loop
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   └── ooda_loop_struct.rs
│   │   │       │   │   └── strange_loop_struct.rs
│   │   │       │   ├── emoji_executors
│   │   │       │   │   ├── execute_emoji_as_fiber.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   └── model_thought_to_emojis.rs
│   │   │       │   ├── executable_vibespace
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── vibe_function
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   └── vibe_function_struct.rs
│   │   │       │   │   └── vibe_space
│   │   │       │   │       ├── mod.rs
│   │   │       │   │       └── vibe_space_struct.rs
│   │   │       │   ├── fundamental_units
│   │   │       │   │   ├── fundamental_unit_enum.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── node_struct.rs
│   │   │       │   │   └── prime_bases.rs
│   │   │       │   ├── generators
│   │   │       │   │   ├── generate_char_vectors.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── generic_generators
│   │   │       │   │   ├── generate_vectors.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── generic_units
│   │   │       │   │   ├── fundamental_unit_generic.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   └── node_generic.rs
│   │   │       │   ├── gossip_system
│   │   │       │   │   ├── gossip_message
│   │   │       │   │   │   ├── gossip_message_enum.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── gossip_network
│   │   │       │   │   │   ├── gossip_network_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── gossip_node
│   │   │       │   │   │   ├── gossip_node_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── id_indexed_trees
│   │   │       │   │   ├── leaf_struct.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── node_struct.rs
│   │   │       │   │   ├── uid_type.rs
│   │   │       │   │   ├── unit_ref_enum.rs
│   │   │       │   │   └── universe_struct.rs
│   │   │       │   ├── introspection_system
│   │   │       │   │   ├── introspection_stream
│   │   │       │   │   │   ├── introspection_stream_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── introspector
│   │   │       │   │   │   ├── introspector_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── introspector_sidechain
│   │   │       │   │   ├── introspector_sidechain
│   │   │       │   │   │   ├── introspector_sidechain_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── sidechain_block
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   └── sidechain_block_struct.rs
│   │   │       │   │   └── sidechain_event
│   │   │       │   │       ├── mod.rs
│   │   │       │   │       └── sidechain_event_enum.rs
│   │   │       │   ├── lean4_integration
│   │   │       │   │   ├── lean_abi_bridge
│   │   │       │   │   │   ├── lean_abi_bridge_struct.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── lean_proof_system
│   │   │       │   │   │   ├── lean_proof_system_structs.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── llvm_ir_reflection
│   │   │       │   │   │   ├── llvm_ir_reflection_structs.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── unification_engine
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   └── unification_engine_structs.rs
│   │   │       │   │   ├── unimath_concept
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   └── unimath_concept_enum.rs
│   │   │       │   │   ├── unimath_path_to_quasifiber
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   └── unimath_path_to_quasifiber_function.rs
│   │   │       │   │   └── univalent_type_theory
│   │   │       │   │       ├── mod.rs
│   │   │       │   │       └── univalent_type_theory_structs.rs
│   │   │       │   ├── lean_interface
│   │   │       │   │   ├── function_registry_struct.rs
│   │   │       │   │   ├── lean_types
│   │   │       │   │   │   ├── lean_types_enum.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   ├── living_chunks_system
│   │   │       │   │   ├── chunk_executor
│   │   │       │   │   │   ├── chunk_executor_function.rs
│   │   │       │   │   │   └── mod.rs
│   │   │       │   │   ├── chunk_spawner
│   │   │       │   │   │   ├── chunk_creation
│   │   │       │   │   │   │   ├── create_new_module.rs
│   │   │       │   │   │   │   ├── create_solana_program.rs
│   │   │       │   │   │   │   └── mod.rs
│   │   │       │   │   │   ├── chunk_spawner_function.rs
│   │   │       │   │   │   ├── initializers
│   │   │       │   │   │   │   ├── initialize_llm_components.rs
│   │   │       │   │   │   │   └── mod.rs
│   │   │    
... (truncated)
```