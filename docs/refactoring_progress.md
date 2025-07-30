# Refactoring Progress

This document tracks the ongoing refactoring efforts within the `ragit` project.

## 1. Bootstrap Logic Modularity

**Initial Goal:** Extract shared bootstrap logic from `ragit-command-bootstrap` and `ragit-build-index-worker-single-file` into a common library.

**Decision:** Based on architectural feedback, the common code between two Layer 7 modules (application layer) should reside in Layer 6 (presentation layer).
**Action:**
- Created a new crate: `crates/layer6_presentation/ragit-bootstrap-logic`.
- Moved `build_index_logic` module from `ragit-command-bootstrap/src/bootstrap_commands/` to `ragit-bootstrap-logic/src/`.
- Updated `Cargo.toml` dependencies in `ragit-command-bootstrap` and `ragit-build-index-worker-single-file` to depend on `ragit-bootstrap-logic`.
- Removed duplicated `build_index_logic` from `ragit-build-index-worker-single-file`.
- Corrected internal `use` paths within `ragit-bootstrap-logic`.
- Removed `pub mod build_index_logic;` from `mod.rs` files in `ragit-command-bootstrap` and `ragit-build-index-worker-single-file`.
- **Status:** `ragit-bootstrap-logic` compiles successfully.

## 2. Core Type Relocation (`ragit-index-types`)

**Issue:** `ragit-index-types` was located in `crates/legacy_and_refactoring/`. This is an inappropriate location for core data structures.
**Decision:** `ragit-index-types` is a foundational type definition for indexing and belongs in `layer3_network`.
**Action:**
- Moved `crates/legacy_and_refactoring/ragit-index-types` to `crates/layer3_network/`.
- Updated all `Cargo.toml` files referencing `ragit-index-types` to reflect the new path.

## 3. Error Type Relocation (`ragit-error`, `ragit-error-conversions`)

**Issue:** `ragit-error` and `ragit-error-conversions` were located in `crates/legacy_and_refactoring/`. These are fundamental error types.
**Decision:** Error types are foundational and belong in `layer1_physical`.
**Action:**
- Moved `crates/legacy_and_refactoring/ragit-error` to `crates/layer1_physical/`.
- Moved `crates/legacy_and_refactoring/ragit-error-conversions` to `crates/layer1_physical/`.
- Updated all `Cargo.toml` files referencing these crates to reflect the new paths.

## 4. TF-IDF Relocation (`ragit-tfidf`)

**Issue:** `ragit-tfidf` was located in `crates/legacy_and_refactoring/` and was causing build failures due to its location.
**Decision:** `ragit-tfidf` is closely related to indexing logic and should reside in `layer3_network`.
**Action:**
- Moved `crates/legacy_and_refactoring/ragit-tfidf` to `crates/layer3_network/`.
- Updated all `Cargo.toml` files referencing `ragit-tfidf` to reflect its new path in `crates/layer3_network/`.

## Overall Status

**All changes implemented so far compile successfully.**
