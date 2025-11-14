# Glossary of Ragit Documentation

This glossary defines key terms and concepts found within the Ragit project documentation.

## A

**`add` command**: A Ragit CLI command used to stage files for processing, similar to `git add`. It respects the `.ragignore` file.
**`AddMode`**: An enum defined in `crates/layer1_physical/ragit-types/src/add_mode.rs` that specifies how files should be added (Auto, Manual, Reject).
**`AddResult`**: A struct defined in `crates/layer1_physical/ragit-types/src/add_mode.rs` that contains the number of added files and chunks.
**`ApiConfig`**: A struct defined in `crates/layer5_session/ragit-config/src/lib.rs` that holds API-related configuration settings, such as model, retry settings, and logging.
**`ApiError`**: A canonical error type used across the Ragit project for consistent error handling. It is defined in `crates/layer1_physical/ragit-types/src/api_error.rs` and can be cloned and converted from other error types.
**`api_request_structure.md`**: Documentation detailing the evolution and structure of API requests within Ragit, particularly the `Request` enum and its `ChatRequest` variant.
**`api_utils.rs`**: A module in `crates/layer3_network/ragit-utils/src/api_utils.rs` that provides utility functions related to API interactions, such as `get_ragit_api_key`.
**`architecture.md`**: Documentation outlining the modular architecture of Ragit, emphasizing the "one declaration per file" principle and the responsibilities of various crates.
**`ArgCount`**: An enum defined in `crates/layer3_network/ragit-utils/src/cli_types.rs` that specifies the expected number of arguments for a CLI command.
**`ArgFlag`**: A struct defined in `crates/layer3_network/ragit-utils/src/cli_types.rs` that represents a command-line argument flag with an associated value.
**`ArgParser`**: A struct defined in `crates/layer3_network/ragit-utils/src/cli_types.rs` that provides methods for parsing command-line arguments and flags.
**`ArgType`**: An enum defined in `crates/layer3_network/ragit-utils/src/cli_types.rs` that specifies the expected type of a command-line argument.
**`AuditRecord`**: A struct defined in `crates/layer7_application/api/src/audit.rs` that records token counts and costs for LLM API usage.
**`AuditRecordAt`**: A struct defined in `crates/layer1_physical/ragit-types/src/audit_record_at.rs` used to specify where to dump API usage audit records.

## B

**`BuildConfig`**: A struct defined in `crates/layer5_session/ragit-config/src/lib.rs` that holds configuration settings related to the build process, such as chunk size, image size, and summary lengths.
**`bootstrap` command**: A Ragit CLI command (`cargo run --package ragit-commands -- bootstrap`) that builds an index and uses it to analyze and improve the Ragit codebase. It has various flags for configuration and debugging.
**`bootstrap-new` command**: A newer, more focused Ragit CLI command (`cargo run --package ragit-commands -- bootstrap-new`) for building an index with fixed-size chunking, executing `ragit-build-index-worker-single-file` as a separate process.
**`build` command**: A Ragit CLI command that processes staged files, creates chunks, and marks files as "processed." It can be paused and resumed.
**`build.md`**: Documentation on how to build Ragit from source, including dependencies, `cargo` commands, and specific instructions for `mupdf-rs`.
**`BuildOrchestrator`**: A proposed component in the refactored `Index` responsible for coordinating the complex process of building the index.

## C

**`cargo`**: The Rust package manager and build tool, used to compile and manage Ragit's multi-crate architecture.
**`ChatRequest`**: A variant of the `Request` enum in `crates/api/src/request/mod.rs` that encapsulates all necessary configuration for chat requests to LLMs.
**Chunk**: A basic building block of a Ragit knowledge-base. Defined in `crates/layer1_physical/ragit-types/src/chunk/chunk_struct.rs`, data files are split into chunks, and each chunk is assigned a title and summary. Chunks are stored in a content-addressable manner.
**`ChunkBuildInfo`**: A struct defined in `crates/layer1_physical/ragit-types/src/chunk/chunk_struct.rs` that stores information about how a chunk was built, including file reader key, prompt hash, and model.
**`ChunkLike` trait**: A trait defined in `crates/layer1_physical/ragit-types/src/chunk/chunk_trait.rs` that provides common methods for chunk-like objects, such as `uid`, `char_len`, `image_count`, and `render_source`.
**`ChunkSchema`**: A struct defined in `crates/layer1_physical/ragit-types/src/chunk/chunk_schema.rs` that represents the schema for a chunk, containing its title and summary.
**`ChunkSource`**: An enum defined in `crates/layer1_physical/ragit-types/src/chunk/chunk_source.rs` that indicates the origin of a chunk (e.g., File, Dummy, Merge).
**`chunks.md`**: Documentation explaining what chunks are, how Ragit processes data files into chunks, and how chunks are stored.
**`cli_types.rs`**: A module in `crates/layer3_network/ragit-utils/src/cli_types.rs` that defines types and functions for command-line argument parsing.
**`CliError`**: An enum defined in `crates/layer3_network/ragit-utils/src/error.rs` that represents errors specific to the command-line interface.
**`clone` command**: A Ragit CLI command used to download knowledge-bases from the internet.
**`config` command**: A Ragit CLI command used to read and write Ragit's configuration settings.
**`config.md`**: Documentation detailing Ragit's configuration system, including global settings, the `rag config` command, and configurable parameters.
**`ConfigManager`**: A proposed component in the refactored `Index` responsible for handling all configuration-related logic.
**Content-addressable storage**: A storage method used for chunks in Ragit, similar to Git's object files, where the location of a chunk is determined by its UID (SHA-1 hash).
**`constant.rs`**: A module in `crates/layer3_network/ragit-utils/src/constant.rs` that defines various string constants used throughout the Ragit project, including file names, directory names, and error messages.
**`contribution.md`**: Documentation outlining ways to contribute to Ragit, including adding file readers, prompt engineering, writing documentation, and issuing bug reports.
**Crate**: A compilation unit in Rust, used to organize Ragit's modular architecture into distinct, specialized components.

## D

**`doc_utils.rs`**: A module in `crates/layer3_network/ragit-utils/src/doc_utils.rs` that provides utility functions for documentation, such as `get_doc_content`.
**`dump_api_usage`**: A configuration option that, when enabled, records token counts and estimated costs of LLM API usage.
**`dump_log`**: A configuration option that, when enabled, records all LLM API calls (including failed ones) to `.ragit/logs`.

## E

**`encoding.rs`**: A module in `crates/layer4_transport/ragit-core-utils/src/encoding.rs` that provides functions for encoding data, such as `encode_base64`.
**`Error` (ragit-utils)**: An enum defined in `crates/layer3_network/ragit-utils/src/error.rs` that represents various error types within the `ragit-utils` crate.
**`ErrorKind`**: An enum defined in `crates/layer3_network/ragit-utils/src/error.rs` that categorizes different types of errors within `ragit-utils`.
**`error_helpers.rs`**: A module in `crates/layer1_physical/ragit-types/src/error_helpers.rs` that defines helper functions for mapping various error types to `ApiError`.
**`eval.md`**: Documentation describing how to evaluate RAG (Retrieval Augmented Generation) using logs, token usage, and manual TF-IDF.
**`extract-keywords` command**: A Ragit CLI command used to simulate Step 2 of the RAG pipeline, where an LLM extracts search-keywords from a query.

## F

**`FileError`**: A struct defined in `crates/layer1_physical/ragit-file-error/src/lib.rs` that represents file-related errors in Ragit.
**`FileErrorKind`**: An enum defined in `crates/layer1_physical/ragit-file-error/src/lib.rs` that categorizes different types of file errors.
**File readers**: Components in Ragit responsible for processing different file types (e.g., `.md`, `.txt`, `.pdf`) into chunks.
**`FileReader`**: A struct defined in `crates/layer4_transport/ragit-readers/src/lib.rs` that handles reading files and generating chunks.
**`FileReaderImpl` trait**: A trait defined in `crates/layer4_transport/ragit-readers/src/lib.rs` that provides an interface for different file reader implementations.
**`FileSchema`**: A struct defined in `crates/layer1_physical/ragit-types/src/file_schema.rs` that represents the schema for a file, including its path, processing status, and UID.
**`FileTree`**: A struct defined in `crates/legacy_and_refactoring/ragit-agent/src/file_tree.rs` for representing and manipulating file system structures.
**`FixedChunk`**: A struct defined in `crates/layer1_physical/ragit-types/src/fixed_types/fixed_chunk_struct.rs` that represents a chunk with fixed-size string and vector fields for memory optimization.
**`FixedChunkBuildInfo`**: A struct defined in `crates/layer1_physical/ragit-types/src/fixed_types/fixed_chunk_struct.rs` that stores build information for `FixedChunk` with fixed-size strings.
**`FixedString`**: A struct defined in `crates/layer1_physical/ragit-types/src/fixed_types/fixed_string_struct.rs` that represents a fixed-size string for memory optimization.
**`FixedVec`**: A struct defined in `crates/layer1_physical/ragit-types/src/fixed_types/fixed_vec.rs` that represents a fixed-size vector for memory optimization.
**Flag**: A struct defined in `crates/layer3_network/ragit-utils/src/cli_types.rs` that represents a command-line flag.
**`flush()`**: A method requiring the `std::io::Write` trait to be in scope, used for ensuring buffered output is written.

## G

**Gemini API**: Google's API for large language models, planned for integration into Ragit for metadata generation and embeddings.
**`gemini_embedding`**: An asynchronous Rust function for generating embeddings using the Gemini API.
**`gemini_metadata`**: An asynchronous Rust function for generating titles and summaries using the Gemini API.
**`git-like` workflow**: Ragit's design philosophy, mimicking Git's commands and concepts (e.g., `add`, `build`, content-addressable storage).
**Glossary**: A list of terms with their definitions, compiled from the Ragit documentation.

## I

**`Ignore`**: A struct defined in `crates/layer2_data_link/ignore/src/ignore/ignore_struct.rs` that handles parsing and matching of ignore patterns (e.g., from `.gitignore`, `.ragignore`).
**`Image`**: A struct defined in `crates/layer1_physical/ragit-types/src/image/image_struct.rs` that represents an image, including its UID, type, and bytes.
**`ImageSchema`**: A struct defined in `crates/layer1_physical/ragit-types/src/image/schema.rs` that represents the schema for an image, including its UID, extracted text, and size.
**`ImageType`**: An enum defined in `crates/layer1_physical/ragit-types/src/pdl_types.rs` (and duplicated in `crates/layer1_physical/ragit-types/src/image/image_struct.rs`) that specifies the type of an image (e.g., Png, Jpeg, Svg).
**`Index` struct**: The core data structure in Ragit that manages the knowledge-base. It is undergoing refactoring from a monolithic structure to a facade coordinating specialized manager objects.
**`index.md`**: Documentation providing a high-level overview of Ragit, its core purpose, key differentiating features, modular design, platform support, and links to other documentation.
**`IndexCore`**: A proposed component in the refactored `Index` responsible for managing the fundamental state and metadata of the index.
**`index_refactoring_detailed.md`**: Documentation outlining the modular refactoring of the `Index` struct, mapping components to OSI layers and illustrating dependencies.
**`index_refactoring.md`**: Documentation outlining the proposed refactoring of the monolithic `Index` struct into several specialized components.
**`index_refactoring_progress.md`**: Documentation detailing the current status of the index refactoring, lessons learned, and future goals.
**`init` command**: A Ragit CLI command used to initialize a new Ragit repository, creating a `.ragit/` directory.
**`introspector_ideas.md`**: Documentation proposing a `log_verbose!` macro for improved logging and performance within the `MemoryMonitor`.
**Inverted Index**: A data structure that makes full-text searching much faster in Ragit. It can be built using `rag ii-build`.
**`IIStatus`**: An enum defined in `crates/layer1_physical/ragit-types/src/ii_status.rs` that represents the status of the inverted index (None, Outdated, Complete, Ongoing).
**`ii-build` command**: A Ragit CLI command used to build an inverted index.
**`ii-status` command**: A Ragit CLI command used to check the status of the inverted index.

## J

**`jemalloc`**: A general-purpose memory allocator that can be used for memory profiling in Ragit.
**`jeprof`**: A Perl script used to analyze `.heap` files generated by `jemalloc` for memory profiling.
**`JsonType`**: An enum defined in `crates/layer1_physical/ragit-types/src/json_type.rs` that represents different JSON data types.

## K

**`Keywords`**: A struct defined in `crates/layer3_network/ragit-utils/src/query/keyword.rs` that represents a collection of keywords extracted from a query.
**`Kill`**: A variant of the `Request` enum used for internal worker communication, representing a signal to terminate a worker process.
**Knowledge-base**: A collection of processed data files (chunks) in Ragit, which can be queried.
**`ragit-korean`**: A crate (`crates/layer6_presentation/korean/src/lib.rs`) that provides Korean text tokenization functionality.

## L

**`layered_architecture.md`**: Documentation outlining the plan to refactor Ragit into a formal layered architecture based on the "Ragit Crate OSI Model."
**`lessons.md`**: Documentation summarizing key lessons learned during refactoring, focusing on consistent path handling, module visibility, error trait implementations, dependency version conflicts, systematic error resolution, resource loading, and debugging practices.
**`log_verbose!` macro**: A proposed macro for simplified and efficient verbose logging within the `MemoryMonitor`.
**`ls-models` command**: A Ragit CLI command used to list available language models.

## M

**`Matcher` trait**: A fundamental trait defined in `ragit-core` (`crates/layer1_physical/ragit-core/src/lib.rs`) with a single method `is_match(&self, path: &Path) -> bool`, used for pattern matching against file paths.
**`max_iterations`**: A flag for bootstrap commands that allows graceful exit after a certain number of iterations, primarily for debugging.
**`max_memory_gb`**: A flag for bootstrap commands that sets a maximum memory limit in gigabytes, designed for graceful exit if exceeded.
**`MemoryMonitor`**: A component in Ragit (`crates/layer1_physical/ragit-memory-monitor/src/lib.rs`) used for monitoring and logging memory usage, including performance and memory alerts.
**`memory_utils.rs`**: A module in `crates/layer3_network/ragit-utils/src/memory_utils.rs` that provides functions for printing memory usage, checking memory limits, and listing processes.
**`merge` command**: A Ragit CLI command used to extend an existing knowledge-base by merging another knowledge-base into it.
**Message**: A struct defined in `crates/layer1_physical/ragit-types/src/pdl_types.rs` representing a message in a PDL conversation, containing a `Role` and `MessageContent`.
**`MessageContent`**: An enum defined in `crates/layer1_physical/ragit-types/src/pdl_types.rs` representing the content of a message, which can be a `String` or an `Image`.
**MetaEmojiWASM**: A proposed WASM module for Ragit that allows agents to load and execute Ragit's core logic directly within their sandboxed environment, using a semantic layer for function calling based on "meta-emojis."
**`metaprogram_kitkat_plan.md`**: Documentation detailing a meta-program for pausing work, planning, documenting, and committing, along with current status, new critical path, and generalized lessons learned.
**`Model`**: A struct defined in `crates/layer7_application/ragit-model/src/model/model_struct.rs` representing a language model, including its API URL, test model status, name, API name, image reading capability, API provider, pricing, timeout, explanation, API key, environment variable, rate limits, quality expectations, expected response time, initial score, and current key index.
**`ModelQueryResponse`**: A struct defined in `crates/layer6_presentation/ragit-model-query-response/src/lib.rs` that encapsulates the response from a language model query, including multi-turn schema, retrieved chunks, and the response message.
**`ModelRaw`**: A struct defined in `crates/layer7_application/ragit-groq-data/src/lib.rs` that represents a raw model configuration, including name, API provider, pricing, and rate limits.
**`models.json`**: A configuration file used to define and manage language models in Ragit.
**`models.md`**: Documentation explaining how to configure language models by modifying `models.json`.
**Monster Group**: In Ragit's "Project Numerology," the prime factorization of the Monster Group's order serves as the target state for the project's conceptual prime factorization.
**`multi_turn.md`**: Documentation explaining what multi-turn queries are in Ragit, how they work through query rewriting, and how to use them.
**Multi-turn query**: A Ragit feature that allows users to ask follow-up questions, where the system rephrases the query to include context from previous turns.
**`MultiModalContent`**: An enum defined in `crates/layer1_physical/ragit-types/src/chunk/rendered_chunk.rs` representing content that can be either `Text` or an `Image` (identified by its UID).
**`MultiTurnSchema`**: A struct defined in `crates/layer3_network/ragit-utils/src/query/multi_turn_schema.rs` that represents the schema for multi-turn queries, indicating if it's a query, in-context, and the rephrased query.
**`mupdf-rs`**: A dependency for Ragit's `pdf` feature, used for handling PDF files.

## N

**`numerology.md`**: Documentation introducing the concept of "Project Numerology" in Ragit, where components are assigned prime numbers and their combinations are products of these primes, forming a "prime lattice."

## O

**"One Declaration Per File" principle**: A core architectural principle in Ragit where each struct, enum, and function is ideally placed in its own dedicated file, promoting modularity and reusability.
**OSI Model**: A conceptual framework used to map Ragit's crates to different layers, illustrating their roles and dependencies.

## P

**`ParsedArgs`**: A struct defined in `crates/layer3_network/ragit-utils/src/cli_types.rs` that stores the parsed command-line arguments and flags.
**`PartialApiConfig`**: A struct defined in `crates/layer5_session/ragit-config/src/lib.rs` that holds optional API-related configuration settings for partial updates.
**`PartialBuildConfig`**: A struct defined in `crates/layer5_session/ragit-config/src/lib.rs` that holds optional build-related configuration settings for partial updates.
**`PartialQueryConfig`**: A struct defined in `crates/layer5_session/ragit-config/src/lib.rs` that holds optional query-related configuration settings for partial updates.
**`path_utils.rs` (ragit-core-utils)**: A module in `crates/layer4_transport/ragit-core-utils/src/path_utils.rs` that provides utility functions for path manipulation, including conversions, joining, and normalization.
**`path_utils_refactoring_notes.md`**: Documentation detailing planned and implemented refactoring for `path_utils.rs`.
**`Pattern`**: A struct defined in `crates/layer2_data_link/ignore/src/ignore/pattern_struct.rs` that represents a parsed ignore pattern.
**`PatternUnit`**: An enum defined in `crates/layer2_data_link/ignore/src/ignore/pattern_unit_enum.rs` that represents a single unit within an ignore pattern (e.g., DoubleAster, Regex, Fixed).
**PDL (Prompt Description Language)**: A special file format used by Ragit to represent prompts, allowing for pragmatic prompts using Tera template language, embedded images, and forced JSON output with designated schemas.
**`pdl_format.md`**: Documentation explaining the Prompt Description Language (PDL) used in Ragit.
**`PdlError`**: An enum defined in `crates/layer1_physical/ragit-types/src/pdl_error.rs` that represents errors encountered during PDL processing, including invalid PDL format, I/O errors, and JSON errors.
**`PdlRole`**: An enum defined in `crates/layer1_physical/ragit-types/src/pdl_types.rs` that specifies the role of a turn in a PDL conversation (e.g., User, Assistant, System, Reasoning, Schema).
**`pdl_types.rs`**: A module within `ragit-types` (`crates/layer1_physical/ragit-types/src/pdl_types.rs`) that defines types related to PDL, such as `MessageContent`, `Role`, `PdlRole`, `Message`, and `ImageType`.
**`pipeline.md`**: Documentation explaining Ragit's RAG pipeline, detailing how it uses LLMs to extract keywords, performs TF-IDF search, reranks chunks, and then uses the most relevant chunks for RAG.
**`prelude.rs` (ragit-types)**: A module in `crates/layer1_physical/ragit-types/src/prelude.rs` that re-exports commonly used types and traits, simplifying `use` statements in other modules.
**`prelude.rs` (ragit-utils)**: A module in `crates/layer3_network/ragit-utils/src/prelude.rs` that re-exports various types and functions from `ragit-types`, `ragit-utils` itself, and `anyhow`.
**`Prettify` trait**: A trait defined in `crates/layer6_presentation/ragit-schema/src/prettify.rs` that provides a method to format data structures into a more human-readable JSON representation.
**Prime lattice**: In Ragit's "Project Numerology," a multi-dimensional lattice where prime factors represent fundamental properties of a component, and composite numbers represent complex modules.
**`ProcessedDoc`**: A struct defined in `crates/layer1_physical/ragit-types/src/processed_doc.rs` that represents a processed document, containing its UID and tokens.
**"Project Numerology"**: A conceptual framework in Ragit that uses prime factorization to understand and organize modularity, complexity, and interdependencies.
**`project_root.rs`**: A module in `crates/layer3_network/ragit-utils/src/project_root.rs` that defines `find_root` for locating the Ragit project root directory.
**Prompt engineering**: The process of designing and refining prompts for LLMs, which can be done in Ragit by editing PDL files or modifying source code.
**`prompt_engineering.md`**: Documentation explaining how to perform prompt engineering in Ragit.
**`prompts.rs`**: A module in `crates/layer3_network/ragit-utils/src/prompts.rs` that is intentionally left empty, as prompts are now loaded at runtime by the `Index` struct.
**`publish.md`**: Documentation detailing the process for publishing new versions of Ragit.

## Q

**`query` command**: A Ragit CLI command used to ask questions on a knowledge-base. It can be run in interactive mode (`--interactive`).
**`QueryConfig`**: A struct defined in `crates/layer1_physical/ragit-types/src/query.rs` that holds configuration settings related to queries, such as enabling RAG, inverted index, and maximum summaries/retrievals.
**`QueryResponse`**: A struct defined in `crates/layer1_physical/ragit-types/src/query_turn.rs` that represents the response part of a query turn.
**`QueryTurn`**: A struct defined in `crates/layer1_physical/ragit-types/src/query_turn.rs` that represents a single turn in a multi-turn query conversation, including the query and its response.
**`quick_guide.md`**: Documentation providing a step-by-step guide on creating, building, cloning, merging, configuring, and querying Ragit knowledge bases.

## R

**RAG (Retrieval Augmented Generation)**: A technique used by Ragit where an LLM's response is augmented with retrieved information from a knowledge-base.
**`ragit-api`**: A Ragit crate (`crates/layer7_application/api/src/lib.rs`) that defines core API interfaces and data structures for interaction between different parts of the system, including loading and saving models.
**`ragit-build-index-worker-single-file`**: A binary executed by the `bootstrap-new` command for building an index with fixed-size chunking.
**`ragit-cli`**: A Ragit crate (`crates/layer7_application/cli/src/lib.rs`) that provides command-line argument parsing utilities.
**`ragit-commands`**: A Ragit crate (`crates/layer7_application/ragit-commands/src/main.rs`) that serves as the main entry point for the Ragit CLI, parsing command-line arguments and dispatching to subcommands.
**`ragit-config`**: A Ragit crate (`crates/layer5_session/ragit-config/src/lib.rs`) that manages application-wide configuration settings, including partial configurations and loading from home directory.
**`ragit-core`**: A Ragit crate (`crates/layer1_physical/ragit-core/src/lib.rs`) that houses fundamental traits like `Matcher`.
**`ragit-core-utils`**: A Ragit crate (`crates/layer4_transport/ragit-core-utils/src/lib.rs`) that provides core utility functions, including path manipulation and encoding.
**`ragit-error`**: A Ragit crate that defines a centralized error handling mechanism.
**`ragit-file-error`**: A Ragit crate (`crates/layer1_physical/ragit-file-error/src/lib.rs`) that defines `FileError` and `FileErrorKind` for file-related errors.
**`ragit-fs`**: A Ragit crate (`crates/layer2_data_link/fs/src/lib.rs`) that encapsulates file system operations, including reading, writing, and path manipulation.
**`ragit-groq-data`**: A Ragit crate (`crates/layer7_application/ragit-groq-data/src/lib.rs`) that defines `ModelRaw` and `DEFAULT_MODELS` for Groq model configurations.
**`ragit-ignore`**: A Ragit crate (`crates/layer2_data_link/ignore/src/ignore/ignore_struct.rs`) dedicated to handling `.gitignore` parsing logic and pattern matching.
**`ragit-index`**: A Ragit crate (`crates/layer3_network/ragit-index/src/lib.rs`) that manages the core indexing logic, including chunk methods and query helpers.
**`ragit-index-build`**: A proposed new crate responsible for coordinating the complex process of building the index.
**`ragit-index-core`**: A proposed new crate that will house core index logic, including recovery.
**`ragit-index-effects`**: A new Ragit crate created to house all monadic operations (I/O, error handling, external API calls).
**`ragit-index-io`**: A Ragit crate that handles input/output operations specifically related to the index.
**`ragit-index-query`**: A Ragit crate that contains logic for querying the index.
**`ragit-index-storage`**: A proposed new crate responsible for managing all file system interactions specific to the index's internal structure.
**`ragit-index-types`**: A Ragit crate that contains the core `Index` struct and related data structures, designed to be pure and data-only.
**`ragit-korean`**: A crate (`crates/layer6_presentation/korean/src/lib.rs`) that provides Korean text tokenization functionality.
**`ragit-memory-monitor`**: A crate (`crates/layer1_physical/ragit-memory-monitor/src/lib.rs`) that provides memory monitoring and reporting functionalities.
**`ragit-model`**: A Ragit crate (`crates/layer7_application/ragit-model/src/lib.rs`) responsible for managing and interacting with different language models.
**`ragit-model-provider`**: A Ragit crate responsible for managing and interacting with different language model providers.
**`ragit-model-query-response`**: A struct defined in `crates/layer6_presentation/ragit-model-query-response/src/lib.rs` that encapsulates the response from a language model query.
**`ragit-pdl`**: A Ragit crate (`crates/layer6_presentation/pdl/src/lib.rs`) that handles the Prompt Description Language (PDL), including parsing, rendering, and schema enforcement.
**`ragit-path_utils.rs`**: A module in `crates/layer3_network/ragit-utils/src/ragit_path_utils.rs` that defines utility functions for path manipulation specific to Ragit.
**`ragit-query`**: A Ragit crate (`crates/layer4_transport/ragit-query/src/lib.rs`) dedicated to handling query logic, including UID queries.
**`ragit-rate-limit`**: A Ragit crate (`crates/layer5_session/ragit-rate-limit/src/lib.rs`) that contains placeholder functionality for merging rate limits.
**`ragit-readers`**: A Ragit crate (`crates/layer4_transport/ragit-readers/src/lib.rs`) that consolidates all file-reading logic and provides the `FileReader` struct and `FileReaderImpl` trait.
**`ragit-schema`**: A Ragit crate (`crates/layer6_presentation/ragit-schema/src/lib.rs`) that contains the logic for handling and interpreting schemas, including the `Prettify` trait.
**`ragit-session-query`**: A Ragit crate (`crates/layer5_session/ragit-session-query/src/lib.rs`) that manages query sessions and multi-turn conversations, including selecting turns for context.
**`ragit-tfidf`**: A Ragit crate that implements TF-IDF calculations for text analysis.
**`ragit-types`**: A foundational Ragit crate (`crates/layer1_physical/ragit-types/src/lib.rs`) that contains common type and data structure definitions, with no local dependencies.
**`ragit-uid`**: A Ragit crate (`crates/layer2_data_link/ragit-uid/src/lib.rs`) that manages Unique Identifiers (UIDs), including loading and saving from files.
**`ragit-utils`**: A Ragit crate (`crates/layer3_network/ragit-utils/src/lib.rs`) that provides general-purpose utility functions, including error handling, prompts, query utilities, string utilities, API utilities, CLI types, doc utilities, project root finding, path utilities, UID helpers, version info, and memory utilities.
**`README.md`**: The main README file for the Ragit project, providing an overview and introduction.
**`refactoring_history.md`**: Documentation chronicling the detailed refactoring progress and specific issues addressed during Ragit's development.
**`refactoring_lessons.md`**: Documentation summarizing key refactoring principles and challenges encountered during Ragit's development.
**`refactoring_master_plan.md`**: Documentation providing a comprehensive overview of the entire `ragit` refactoring effort, including historical context, current status, and a three-phase plan.
**`refactoring_progress.md`**: Documentation tracking the ongoing refactoring efforts within the `ragit` project.
**`refactoring_status.md`**: Documentation providing an update on the `Index` struct refactoring, detailing progress, current state, and next immediate steps.
**`RemoveResult`**: A struct defined in `crates/layer1_physical/ragit-types/src/remove_result.rs` that contains the number of removed files and chunks.
**`RenderedChunk`**: A struct defined in `crates/layer1_physical/ragit-types/src/chunk/rendered_chunk.rs` that represents a chunk after it has been rendered, including PDL data, human-readable data, and raw multimodal content.
**`Request` enum**: An enum in `crates/api/src/request/mod.rs` that encapsulates different types of requests, including `ChatRequest`, `BuildChunks`, and `Kill`.
**`retrieve-chunks` command**: A Ragit CLI command used to simulate Steps 2, 3, and 4 of the RAG pipeline (keyword extraction, TF-IDF search, and reranking).
**`RecoveryManager`**: A proposed component in the refactored `Index` responsible for handling error recovery, validation, and integrity checking.
**Reranking**: The process in Ragit's RAG pipeline where an LLM reviews titles and summaries of retrieved chunks and selects the most relevant ones.
**`Role`**: An enum defined in `crates/layer1_physical/ragit-types/src/pdl_types.rs` that specifies the role of a message sender in a PDL conversation (e.g., User, Assistant, System, Reasoning).

## S

**`SchemaParseError`**: An enum defined in `crates/layer1_physical/ragit-types/src/pdl_error.rs` that represents errors encountered during schema parsing.
**Schema**: In PDL, a way to force LLMs to output JSON with a designated structure, including constraints.
**`SearchEngine`**: A proposed component in the refactored `Index` responsible for encapsulating all search functionality.
**Self-improvement**: A phase in the `ragit bootstrap` command where Ragit analyzes and improves code by reading its own source, generating a prompt, executing a query, and writing improved code to a file.
**`solfunmeme_metaemojiwasm.md`**: Documentation detailing the integration of Gemini API and agent-callable functionality into Ragit, introducing the MetaEmojiWASM module and the Solfunmeme philosophy.
**Solfunmeme**: A philosophy in Ragit that bridges formal code structures with intuitive, semantic representations, particularly through the MetaEmojiWASM module.
**`Span`**: An enum defined in `crates/layer3_network/ragit-utils/src/error.rs` that represents a location within a command-line argument string, used for error reporting.
**`std::io::Write`**: A Rust trait that provides methods for writing to a stream, including `flush()`.
**`StorageManager`**: A proposed component in the refactored `Index` responsible for managing all file system interactions specific to the index's internal structure.
**`string_utils.rs`**: A module in `crates/layer3_network/ragit-utils/src/string_utils.rs` that provides utility functions for string manipulation, such as calculating edit distance and finding the closest string.
**`strum`**: A Rust crate that provides macros for working with enums, which can sometimes cause dependency version conflicts.
**Summary**: A brief overview generated by AI for each chunk in a Ragit knowledge-base.
**`Summary`**: A struct defined in `crates/layer1_physical/ragit-types/src/summary.rs` that represents a summary, containing its content.
**`SummaryMode`**: An enum defined in `crates/layer1_physical/ragit-types/src/summary.rs` that specifies the mode for generating summaries (Simple, Rerank).

## T

**Tera**: A template engine used in PDL for writing pragmatic prompts.
**TF-IDF (Term Frequency-Inverse Document Frequency)**: A numerical statistic used in Ragit to reflect how important a word is to a document in a collection or corpus, used for searching and scoring chunks.
**`tfidf` command**: A Ragit CLI command used to test Ragit's TF-IDF engine.
**Tokenization**: The process of breaking down text into smaller units (tokens), used in Ragit's chunking process.
**`Tracker`**: A struct defined in `crates/layer7_application/api/src/audit.rs` that tracks API usage records.

## U

**UID (Unique Identifier)**: A 64-character hexadecimal string based on SHA3-256 hash, used in Ragit to identify chunks, files, and images. It includes metadata. Defined in `crates/layer1_physical/ragit-types/src/uid/mod.rs`.
**`uid_helpers.rs`**: A module in `crates/layer3_network/ragit-utils/src/uid_helpers.rs` that provides helper functions for UID generation, such as `uid_new_file`.
**`uid_query.md`**: Documentation explaining UIDs in Ragit, their similarity to Git hashes, how to query them using full or prefix matches, and path matches.
**`UidQueryConfig`**: A struct defined in `crates/layer4_transport/ragit-query/src/query_helpers.rs` that configures UID queries, specifying what types of UIDs to search for.
**`UidQueryResult`**: A struct defined in `crates/layer4_transport/ragit-query/src/query_helpers.rs` that represents the result of a UID query, containing lists of matching chunks, images, processed files, and staged files.
**`UidError`**: An enum defined in `crates/layer1_physical/ragit-types/src/uid/mod.rs` that represents errors related to UID operations.
**`UidType`**: An enum defined in `crates/layer1_physical/ragit-types/src/uid/mod.rs` that specifies the type of a UID (e.g., Chunk, Image, File).
**`UidWriteMode`**: An enum defined in `crates/layer1_physical/ragit-types/src/uid/mod.rs` that specifies the write mode for UIDs (Naive, Compact).
**Unstaged files**: Data files that have not yet been added to the staging area in Ragit.

## V

**`VersionInfo`**: A struct defined in `crates/layer3_network/ragit-utils/src/version_info.rs` that holds version information for Ragit.
**`--verbose` flag**: A flag for bootstrap commands that enables verbose output, useful for debugging.

## W

**WASM (WebAssembly)**: A binary instruction format for a stack-based virtual machine, proposed for use in Ragit's MetaEmojiWASM module for portable and efficient execution of core logic.
**`WriteMode`**: An enum defined in `crates/layer2_data_link/fs/src/lib.rs` that specifies how files should be written, including options for appending, creating, truncating, and atomic writes.