# Index of Ragit Documentation

This index provides a structured overview of the Ragit project documentation, categorizing documents by their primary themes and linking to relevant sections.

## I. Core Concepts & Overview

*   **`README.md`**
    *   **Themes**: Project introduction, core purpose, key features, modular design, platform support.
    *   **Key Takeaways**: Ragit as a git-like RAG framework, easy knowledge-base creation and sharing, unique RAG pipeline (no embeddings, TF-IDF, reranking), multi-crate architecture.

*   **`index.md`**
    *   **Themes**: High-level overview, core purpose, differentiating features, modular design, platform support, links to other documentation.
    *   **Key Takeaways**: Reinforces `README.md` themes, provides quick command examples (`init`, `add`, `build`, `query`).

*   **`intro.txt`**
    *   **Themes**: Concise project overview, categorized list of Ragit CLI commands.
    *   **Key Takeaways**: Quick reference for Ragit's functionalities (create, share, work with, get info, query, simulate pipeline, tutorial commands).

*   **`glossary.md`**
    *   **Themes**: Definitions of key terms and concepts.
    *   **Key Takeaways**: Comprehensive reference for Ragit terminology.

## II. Architecture & Refactoring

*   **`architecture.md`**
    *   **Themes**: Modular architecture, "one declaration per file" principle, crate responsibilities, inter-crate dependencies, OSI layer mapping.
    *   **Key Takeaways**: Detailed breakdown of Ragit's modular design, explanation of core architectural principles, overview of major crates and their roles.

*   **`layered_architecture.md`**
    *   **Themes**: Strategic plan for formal layered architecture, Ragit Crate OSI Model, refactoring steps.
    *   **Key Takeaways**: Rationale for layered architecture, detailed mapping of crates to OSI layers, step-by-step plan for implementation.

*   **`refactoring_history.md`**
    *   **Themes**: Detailed refactoring progress, specific issues addressed, previous status, recent progress, next immediate steps.
    *   **Key Takeaways**: Chronicles the evolution of the codebase, highlights resolved compilation errors and dependency issues.

*   **`refactoring_lessons.md`**
    *   **Themes**: Key refactoring principles, challenges encountered, solutions.
    *   **Key Takeaways**: Core principles like "One Declaration Per File" and "Always Use Prelude," iterative error fixing strategies, common Rust-specific challenges (e.g., `PathBuf` vs. `&str`, error handling interoperability).

*   **`refactoring_master_plan.md`**
    *   **Themes**: Comprehensive overview of the entire refactoring effort, historical context, current status, three-phase plan.
    *   **Key Takeaways**: Consolidates all refactoring information, outlines the final plan for a stable, modular, and maintainable codebase.

*   **`refactoring_progress.md`**
    *   **Themes**: Ongoing refactoring efforts, bootstrap logic modularity, core type relocation, error type relocation, TF-IDF relocation, bootstrap command refinement, verbose output evaluation, performance monitoring.
    *   **Key Takeaways**: Tracks the active development and improvements in various areas of the codebase.

*   **`refactoring_status.md`**
    *   **Themes**: Update on `Index` struct refactoring, progress, current state, next immediate steps.
    *   **Key Takeaways**: Provides a snapshot of the `Index` refactoring, detailing specific achievements and immediate priorities.

*   **`index_refactoring.md`**
    *   **Themes**: Proposed refactoring of the monolithic `Index` struct, specialized components (`IndexCore`, `ConfigManager`, `StorageManager`, `SearchEngine`, `BuildOrchestrator`, `RecoveryManager`), implementation strategy.
    *   **Key Takeaways**: Conceptual breakdown of the `Index` refactoring, illustrating the move towards a more modular design.

*   **`index_refactoring_detailed.md`**
    *   **Themes**: Comprehensive refactoring plan for the `Index` struct, OSI layer mapping, manager components and crate mapping.
    *   **Key Takeaways**: In-depth explanation of the `Index` refactoring, detailing responsibilities and crate assignments within the OSI model.

*   **`index_refactoring_progress.md`**
    *   **Themes**: Current state of index refactoring, lessons learned (strict crate hierarchy, monadic separation), goals.
    *   **Key Takeaways**: Tracks the ongoing progress of the `Index` refactoring, highlighting key insights and future objectives.

*   **`path_utils_refactoring_notes.md`**
    *   **Themes**: Planned and implemented refactoring for `path_utils.rs`, issues identified, changes made.
    *   **Key Takeaways**: Specific details on the refactoring of path utility functions.

## III. Commands & Workflow

*   **`bootstrap.md`**
    *   **Themes**: `ragit bootstrap` and `ragit bootstrap-new` commands, flags, workflows, debugging strategies, memory management, profiling.
    *   **Key Takeaways**: Comprehensive guide to Ragit's bootstrap processes, including how to use and debug them.

*   **`build.md`**
    *   **Themes**: Building Ragit from source, dependencies, `cargo` commands, `mupdf-rs`, testing, pre-built binaries.
    *   **Key Takeaways**: Instructions for compiling and setting up Ragit.

*   **`chunks.md`**
    *   **Themes**: Definition of chunks, data file processing (unstaged, staged, processed), content-addressable storage.
    *   **Key Takeaways**: Explains the fundamental data unit in Ragit and its lifecycle.

*   **`config.md`**
    *   **Themes**: Configuration system, global settings, `rag config` command, configurable parameters.
    *   **Key Takeaways**: How to manage and customize Ragit's behavior.

*   **`multi_turn.md`**
    *   **Themes**: Multi-turn queries, query rewriting, usage via CLI and Rust API.
    *   **Key Takeaways**: Explains a key feature for conversational interaction with Ragit.

*   **`pipeline.md`**
    *   **Themes**: Ragit's RAG pipeline, keyword extraction, TF-IDF search, reranking, RAG process.
    *   **Key Takeaways**: Detailed explanation of how Ragit processes queries and retrieves information.

*   **`quick_guide.md`**
    *   **Themes**: Step-by-step guide to creating, building, cloning, merging, configuring, and querying knowledge bases.
    *   **Key Takeaways**: Practical instructions for getting started with Ragit.

*   **`uid_query.md`**
    *   **Themes**: UIDs (Unique Identifiers), similarity to Git hashes, query methods (full, prefix, path match).
    *   **Key Takeaways**: How Ragit identifies and retrieves data using UIDs.

## IV. Development & Contribution

*   **`gemini_cli_change_management_sop.md`**
    *   **Themes**: Gemini CLI, change management, SOP.
    *   **Key Takeaways**: Defines the process for using the Gemini CLI agent within the project's structured change management workflow.

*   **`build_file_tree.md`**
    *   **Themes**: Codebase analysis, file tree construction, term extraction, term pairs, term triples, feature extraction.
    *   **Key Takeaways**: Explains the tool for analyzing file and directory names to understand linguistic patterns and support semantic search.

*   **`contribution.md`**
    *   **Themes**: Ways to contribute (file readers, prompt engineering, documentation, bug reports), running tests.
    *   **Key Takeaways**: Guide for developers and users interested in contributing to the project.

*   **`eval.md`**
    *   **Themes**: Evaluating RAG, logs, token usage, manual TF-IDF.
    *   **Key Takeaways**: Methods for assessing the performance and effectiveness of Ragit's RAG capabilities.

*   **`introspector_ideas.md`**
    *   **Themes**: Ideas for improving development tools and introspection, `log_verbose!` macro for `MemoryMonitor`.
    *   **Key Takeaways**: Proposals for enhancing Ragit's debugging and monitoring features.

*   **`models.md`**
    *   **Themes**: Configuring language models, `models.json` schema, search order for configuration files.
    *   **Key Takeaways**: How to integrate and manage different LLMs with Ragit.

*   **`pdl_format.md`**
    *   **Themes**: Prompt Description Language (PDL), syntax, Tera templating, image embedding, schema enforcement.
    *   **Key Takeaways**: Detailed explanation of Ragit's prompt definition system.

*   **`prompt_engineering.md`**
    *   **Themes**: Prompt engineering using PDL files or source code, testing prompts.
    *   **Key Takeaways**: Practical guidance on optimizing LLM interactions.

*   **`publish.md`**
    *   **Themes**: Process for publishing new versions, pre- and post-publication steps, testing, version bumping, binary creation.
    *   **Key Takeaways**: Internal documentation for releasing new Ragit versions.

## V. Conceptual & Future Vision

*   **`grand_plan.md`**
    *   **Themes**: Autonomous agent, self-improvement, software development, API utilization.
    *   **Key Takeaways**: Outlines the vision for an autonomous, self-improving development agent that can perform a wide range of software engineering tasks using its own API.

*   **`ragit_eco_9k_analogy.md`**
    *   **Themes**: Ragit, ECO-PAPER-9K, knowledge formalization.
    *   **Key Takeaways**: Explains the parallels between the `ragit` system and the ECO-PAPER-9K framework, demonstrating how `ragit` serves as a practical implementation of its principles.

*   **`numerology.md`**
    *   **Themes**: Project Numerology, Prime Lattice, Monster Group as a goal configuration.
    *   **Key Takeaways**: A unique conceptual framework for understanding Ragit's modularity and complexity.

*   **`solfunmeme_metaemojiwasm.md`**
    *   **Themes**: Integrating Gemini API, agent-callable functionality, MetaEmojiWASM module, Solfunmeme philosophy.
    *   **Key Takeaways**: Vision for advanced, portable, and semantically rich integration with AI agents.

## VI. Standard Operating Procedures (SOPs)

*   **`daemonization_sop.md`**
    *   **Themes**: Rust, daemonization, server, background process.
    *   **Key Takeaways**: How to create a robust, daemonized Rust server that can be managed through its PID file and a graceful shutdown endpoint.

*   **`quiz_server_sop.md`**
    *   **Themes**: Rust, quiz server, model sampling, weight updating.
    *   **Key Takeaways**: How to interact with the quiz server to sample the model and update its weights.

*   **`self_improvement_sop.md`**
    *   **Themes**: Gemini CLI, self-improvement, embedding refinement, knowledge base expansion, tool development.
    *   **Key Takeaways**: Defines the process for continuous self-improvement of the `ragit` system by the Gemini CLI agent.

## VI. Source Code Modules & Components

This section indexes key Rust modules and components identified during the source code review, linking them to their definitions and explaining their role within the Ragit architecture.

*   **`core_library.md`** (`src/lib.rs`)
    *   **Themes**: Main library entry point, module organization, centralized re-exports, global allocator, versioning guidelines.
    *   **Key Takeaways**: Explains the central role of `src/lib.rs` in defining the project's top-level structure and making core functionalities accessible.

### Layer 1: Physical

*   **`ragit-core`** (`crates/layer1_physical/ragit-core/src/lib.rs`)
    *   **Key Components**: `Matcher` trait.
    *   **Role**: Defines fundamental traits for core functionalities like pattern matching.

*   **`ragit-types`** (`crates/layer1_physical/ragit-types/src/lib.rs`)
    *   **Key Components**: `PdlError`, `SchemaParseError`, `Model` struct, `Image` struct, `ImageType` enum, `ImageSchema` struct, `Chunk` struct, `ChunkBuildInfo` struct, `ChunkLike` trait, `ChunkSchema` struct, `ChunkSource` enum, `MultiModalContent` enum, `RenderedChunk` struct, `Uid` struct, `UidError` enum, `UidType` enum, `UidWriteMode` enum, `JsonType` enum, `AuditRecordAt` struct, `FileSchema` struct, `AddMode` enum, `AddResult` struct, `RemoveResult` struct, `ProcessedDoc` struct, `IIStatus` enum, `Summary` struct, `SummaryMode` enum, `QueryConfig` struct, `PartialQueryConfig` struct, `ApiConfig` struct, `PartialApiConfig` struct, `BuildConfig` struct, `PartialBuildConfig` struct, `QueryResponse` struct, `QueryTurn` struct, error helper functions (`map_io_error`, `map_serde_json_error`, `map_anyhow_error`, `read_dir_to_api_error`, `read_to_string_to_api_error`), `FixedString` struct, `FixedVec` struct, `FixedChunkBuildInfo` struct, `FixedChunk` struct.
    *   **Role**: A foundational crate containing common type and data structure definitions, designed to be pure and without local dependencies.

*   **`ragit-file-error`** (`crates/layer1_physical/ragit-file-error/src/lib.rs`)
    *   **Key Components**: `FileError` struct, `FileErrorKind` enum.
    *   **Role**: Defines specific error types for file-related operations.

*   **`ragit-memory-monitor`** (`crates/layer1_physical/ragit-memory-monitor/src/lib.rs`)
    *   **Key Components**: `MemoryMonitor` struct.
    *   **Role**: Provides functionalities for monitoring and logging memory usage, including performance and memory alerts.

### Layer 2: Data Link

*   **`ragit-fs`** (`crates/layer2_data_link/fs/src/lib.rs`)
    *   **Key Components**: `WriteMode` enum, `read_bytes_offset`, `read_bytes`, `read_string`, `write_bytes`, `write_string`, `file_name`, `extension`, `basename`, `join`, `temp_dir`, `join2`, `join3`, `join4`, `join5`, `set_extension`, `is_dir`, `is_symlink`, `exists`, `exists_str`, `parent`, `try_create_dir`, `create_dir`, `create_dir_all`, `rename`, `copy_dir`, `copy_file`, `last_modified`, `file_size`, `read_dir`, `remove_file`, `remove_dir`, `remove_dir_all`, `into_abs_path`, `current_dir`, `set_current_dir`, `diff`, `get_relative_path`, `normalize`.
    *   **Role**: Encapsulates file system operations, providing a consistent and safe interface for interacting with the local filesystem.

*   **`ragit-ignore`** (`crates/layer2_data_link/ignore/src/lib.rs`)
    *   **Key Components**: `Ignore` struct, `Pattern` struct, `PatternUnit` enum.
    *   **Role**: Handles parsing and matching of ignore patterns (e.g., from `.gitignore`, `.ragignore`).

*   **`ragit-uid`** (`crates/layer2_data_link/ragit-uid/src/lib.rs`)
    *   **Key Components**: `load_from_file`, `save_to_file`.
    *   **Role**: Provides functions for loading and saving UIDs to files.

### Layer 3: Network

*   **`ragit-index`** (`crates/layer3_network/ragit-index/src/lib.rs`)
    *   **Key Components**: `load_from_file` (in `chunk_methods/io.rs`), `save_to_file` (in `chunk_methods/io.rs`), `UidQueryConfig` struct (in `query_helpers.rs`), `UidQueryResult` struct (in `query_helpers.rs`), `uid_query` (in `query_helpers.rs`).
    *   **Role**: Manages the core indexing logic, including chunk methods and query helpers.

*   **`ragit-utils`** (`crates/layer3_network/ragit-utils/src/lib.rs`)
    *   **Key Components**: `Error` enum, `ErrorKind` enum, `CliError` enum, `Span` enum, `Keywords` struct, `MultiTurnSchema` struct, `substr_edit_distance`, `get_closest_string`, `get_ragit_api_key`, `ArgParser` struct, `ArgCount` enum, `ArgType` enum, `Flag` struct, `ArgFlag` struct, `ParsedArgs` struct, `parse_pre_args`, `get_doc_content`, `find_root`, `join_paths`, `join3_paths`, `get_rag_path`, `get_uid_path`, `get_ii_path`, `get_normalized_abs_pathbuf`, `uid_new_file`, `VersionInfo` struct, `MemoryMonitor` struct (re-exported from `ragit-memory-monitor`), `print_memory_usage`, `check_memory_limit`, `print_process_list`.
    *   **Role**: Provides general-purpose utility functions, including error handling, prompts, query utilities, string utilities, API utilities, CLI types, doc utilities, project root finding, path utilities, UID helpers, version info, and memory utilities.

### Layer 4: Transport

*   **`ragit-core-utils`** (`crates/layer4_transport/ragit-core-utils/src/lib.rs`)
    *   **Key Components**: `get_relative_path` (in `path.rs`), `join_paths` (in `path.rs`), `join3_paths` (in `path.rs`), `get_normalized_abs_pathbuf` (in `path.rs`), `pathbuf_to_str` (in `path_utils.rs`), `str_to_pathbuf` (in `path_utils.rs`), `path_to_display` (in `path_utils.rs`), `str_to_path_ref` (in `path_utils.rs`), `normalize_path` (in `path_utils.rs`), `to_absolute_path` (in `path_utils.rs`), `encode_base64` (in `encoding.rs`).
    *   **Role**: Provides core utility functions, including path manipulation and encoding.

*   **`ragit-query`** (`crates/layer4_transport/ragit-query/src/lib.rs`)
    *   **Key Components**: `UidQueryConfig` struct, `UidQueryResult` struct, `uid_query`.
    *   **Role**: Dedicated to handling query logic, including UID queries.

*   **`ragit-readers`** (`crates/layer4_transport/ragit-readers/src/lib.rs`)
    *   **Key Components**: `FileReaderImpl` trait, `FileReader` struct, `new`, `can_generate_chunk`, `generate_chunk`, `next_chunk`, `fill_buffer_until_n_chunks`, `file_reader_key`, `fetch_images_from_web`, `merge_tokens`.
    *   **Role**: Consolidates all file-reading logic and provides the `FileReader` struct and `FileReaderImpl` trait.

### Layer 5: Session

*   **`ragit-config`** (`crates/layer5_session/ragit-config/src/lib.rs`)
    *   **Key Components**: `PartialBuildConfig` struct, `BuildConfig` struct, `PartialApiConfig` struct, `PartialQueryConfig` struct, `load_config_from_home`, `load_api_config_from_home`, `load_query_config_from_home`, `load_build_config_from_home`.
    *   **Role**: Manages application-wide configuration settings, including partial configurations and loading from home directory.

*   **`ragit-rate-limit`** (`crates/layer5_session/ragit-rate-limit/src/lib.rs`)
    *   **Key Components**: `merge_rate_limits` (placeholder).
    *   **Role**: Contains placeholder functionality for merging rate limits.

*   **`ragit-session-query`** (`crates/layer5_session/ragit-session-query/src/lib.rs`)
    *   **Key Components**: `select_turns_for_context`.
    *   **Role**: Manages query sessions and multi-turn conversations, including selecting turns for context.

### Layer 6: Presentation

*   **`ragit-korean`** (`crates/layer6_presentation/korean/src/lib.rs`)
    *   **Key Components**: `tokenize`.
    *   **Role**: Provides Korean text tokenization functionality.

*   **`ragit-model-query-response`** (`crates/layer6_presentation/ragit-model-query-response/src/lib.rs`)
    *   **Key Components**: `ModelQueryResponse` struct.
    *   **Role**: Encapsulates the response from a language model query.

*   **`ragit-pdl`** (`crates/layer6_presentation/pdl/src/lib.rs`)
    *   **Key Components**: `Pdl` struct, `parse_pdl_from_file`, `parse_pdl`, `escape_pdl_tokens`, `unescape_pdl_tokens`, `into_message_contents`.
    *   **Role**: Handles the Prompt Description Language (PDL), including parsing, rendering, and schema enforcement.

*   **`ragit-schema`** (`crates/layer6_presentation/ragit-schema/src/lib.rs`)
    *   **Key Components**: `Prettify` trait.
    *   **Role**: Contains the logic for handling and interpreting schemas, including the `Prettify` trait.

### Layer 7: Application

*   **`ragit-api`** (`crates/layer7_application/api/src/lib.rs`)
    *   **Key Components**: `FetchResult` struct, `load_models`, `save_models`, `list_models`, `AuditRecord` struct, `Tracker` struct, `dump_api_usage`, `get_user_usage_data_since`, `get_usage_data_since`, `calc_usage`, `dump_pdl`, `message_content_to_json`, `message_contents_to_json_array`, `message_to_json`.
    *   **Role**: Defines core API interfaces and data structures for interaction between different parts of the system, including loading and saving models.

*   **`ragit-cli`** (`crates/layer7_application/cli/src/lib.rs`)
    *   **Key Components**: `parse_pre_args`.
    *   **Role**: Provides command-line argument parsing utilities.

*   **`ragit-commands`** (`crates/layer7_application/ragit-commands/src/main.rs`)
    *   **Key Components**: `Args` struct, `Commands` enum, `main` function.
    *   **Role**: Serves as the main entry point for the Ragit CLI, parsing command-line arguments and dispatching to subcommands.

*   **`ragit-groq-data`** (`crates/layer7_application/ragit-groq-data/src/lib.rs`)
    *   **Key Components**: `ModelRaw` struct, `DEFAULT_MODELS`.
    *   **Role**: Defines `ModelRaw` and `DEFAULT_MODELS` for Groq model configurations.

*   **`ragit-model`** (`crates/layer7_application/ragit-model/src/lib.rs`)
    *   **Key Components**: `Model` struct.
    *   **Role**: Responsible for managing and interacting with different language models.

*   **`ragit-mesh-agent`** (`crates/layer7_application/ragit-mesh-agent/src/main.rs`)
    *   **Key Components**: `Settings` struct, `main` function.
    *   **Role**: Connects to a p2p network of other agents, using a private Solana validator for peer discovery.

### Legacy & Refactoring

*   **`ragit-agent`** (`crates/legacy_and_refactoring/ragit-agent/src/lib.rs`)
    *   **Key Components**: `FileTree` struct.
    *   **Role**: Provides functionalities related to agent-based operations, including file tree representation.