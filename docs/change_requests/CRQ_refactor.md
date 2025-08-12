This is a well-thought-out architecture for a Rust project refactorer, leveraging a columnar index in Parquet for efficient querying and atomic transformations. Below is a concise spec capturing the essence of your blueprint, streamlined for clarity while preserving the core components, schemas, and considerations.

---

### Rust Refactorer Specification

**Goal**: Build an automated, safe, stepwise refactorer for Rust projects/workspaces, driven by a Parquet-based index of code metadata, enabling semantic transformations with validation and rollback.

#### 1. Architecture Overview

- **Indexer**: Generates Parquet tables from Rust source, ASTs, symbols, types, MIR/LLVM, Cargo metadata, and Git history. Runs once initially, then incrementally.
- **Query & Planner**: Uses DuckDB/Arrow to query Parquet tables and produce an ordered list of semantic operations (refactor plan).
- **Patch Generator**: Converts operations into AST-level or textual edits, encoded as semantic patches (JSON-based DSL).
- **Executor**: Applies patches in an isolated Git branch, validates with `cargo check/test`, commits on success, and logs outcomes in Parquet.
- **Rollback & Replay**: Uses Git commits and `operation_log.parquet` for traceability, replay, or rollback.

#### 2. Parquet Schemas (Simplified)

- **files.parquet**:
  - `file_id: int64, path: string, content_hash: string, text: string, size: int64, last_modified_commit: string`
- **ast_nodes.parquet**:
  - `node_id: int64, file_id: int64, kind: string, start_byte: int64, end_byte: int64, parent_id: int64, token_text: string, node_json: string`
- **symbols.parquet**:
  - `symbol_id: int64, name: string, kind: string, def_node_id: int64, def_file_id: int64, type_signature: string, visibility: string, crate_name: string`
- **references.parquet**:
  - `ref_id: int64, symbol_id: int64, file_id: int64, start_byte: int64, end_byte: int64, context_node_id: int64, ref_kind: string`
- **types.parquet**:
  - `type_id: int64, symbol_id: int64, canonical: string, type_json: string, inferred_by: string`
- **git_commits.parquet**:
  - `commit_hash: string, parent_hashes: list<string>, timestamp: timestamp, modified_files: list<string>`
- **cargo_metadata.parquet**:
  - `crate_name: string, manifest_path: string, version: string, dependencies: list<struct>, features: list<string>, edition: string`
- **operation_log.parquet**:
  - `op_id: uuid, patch_id: string, description: string, precondition_passed: bool, applied_commit: string, timestamp: timestamp, result: string`

#### 3. Index Building

- **Sources**:
  - Parse Rust source with `syn` or `tree-sitter` for ASTs with spans.
  - Extract symbols/types from `rustc` (`--emit=mir`, `--emit=llvm-ir`), `rustdoc` (JSON), or `rust-analyzer` (LSP diagnostics).
  - Collect Git history via `git log` and file snapshots.
  - Extract Cargo metadata from `Cargo.toml` and `cargo metadata`.
- **Process**: Convert structured data to Parquet using `pyarrow` or Rust’s `arrow` crate. Use DuckDB for ad-hoc queries.

#### 4. Planning Refactor Operations

1. **Discovery**: Query `symbols` and `references` to identify definitions and usages (e.g., `SELECT * FROM references WHERE symbol_id = (SELECT symbol_id FROM symbols WHERE name = 'foo')`).
2. **Risk Scoring**: Assign risk based on context (e.g., macros, public APIs, `unsafe`).
3. **Dependency Graph**: Order operations (e.g., rename type before its impls).
4. **Preconditions**: Define checks (e.g., `cargo check`, relevant tests pass).
5. **Output**: Ordered list of operations with node IDs, file spans, and preconditions.

#### 5. Semantic Patch Format (JSON)

```json
{
  "id": "rename::foo->bar",
  "description": "Rename function `foo` to `bar`",
  "patterns": [
    {"match": {"node": "Function", "ident": "foo"}, "action": {"rename": "bar"}},
    {"match": {"node": "Path", "ident": "foo"}, "action": {"rename": "bar"}}
  ],
  "constraints": {"exclude_crates": ["third_party"], "only_if_no_conflicts": true},
  "postchecks": ["cargo check", "cargo test --test relevant_tests"]
}
```

- Apply edits in descending `start_byte` order to avoid offset issues.
- Use `syn` for AST edits, reserialize with `rustfmt`.

#### 6. Execution Loop

1. Create branch `refactor/<op_id>/<timestamp>`.
2. Apply AST edits using `syn` or text edits via spans.
3. Run `rustfmt` and `cargo check`. If fails, collect diagnostics and abort.
4. Run targeted tests (from `tests.parquet` or `cargo test --test <names>`).
5. On success, commit with metadata (`op_id`, `patch_id`). Log result in `operation_log.parquet`.
6. Open PR or push commit.

#### 7. Validation

- **Fast**: `cargo check`, targeted `cargo test`.
- **Deep**: Full CI, semver checks for public APIs, fuzz tests for critical code.
- **Regression**: Binary diffing or end-to-end tests.

#### 8. Gotchas

- **Macros/Proc-macros**: Map expanded macro spans back to source using `rustc` expansion or `rust-analyzer`.
- **Conditional Compilation (`cfg`)**: Index per `cfg` variant or track feature flags.
- **Build.rs/Codegen**: Materialize generated code before indexing.
- **Unsafe/FFI**: Flag ABI-sensitive code and require manual review.
- **Cross-crate**: Check `cargo_metadata` for dependent crates; avoid breaking published APIs.
- **Formatting**: Use `rustfmt` to preserve comments and style.

#### 9. Example Workflow: Rename `foo` → `bar`

1. Query `symbols` and `references` for `foo`.
2. Plan: (a) rename definition, (b) rename references.
3. Generate AST patches using `syn`.
4. Apply in branch, run `cargo check`, `cargo test --test relevant`.
5. Commit with `op_id`, log to `operation_log.parquet`.

#### 10. Implementation Stack

- **Storage**: Parquet + DuckDB for queries.
- **Parsing**: `syn` or `tree-sitter` for ASTs; `rust-analyzer` for types.
- **Execution**: Python/Rust for orchestration; Git CLI for commits; Docker for builds.
- **Patch Engine**: Custom AST applier using `syn` or adapt Coccinelle-style engine.

#### 11. Provenance

- Log every operation in `operation_log.parquet` (`op_id`, `patch_id`, commit hash, diagnostics).
- Embed `op_id` in commit messages for traceability.

---

### Next Steps (Optional)

If you want to dive deeper, I can:
- Provide detailed Parquet schemas and DuckDB example queries.
- Write a Python prototype for the planner generating rename patches.
- Design a JSON patch DSL and AST applier using `syn`.

Let me know your preference!
