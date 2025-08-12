# `solfunmeme_clifford` Crate Documentation

This document provides an overview of the `solfunmeme_clifford` Rust crate, detailing its purpose, structure, and interactions within the Solfunmeme-Dioxus project.

## Overview

The `solfunmeme_clifford` crate implements a 3D Clifford algebra for generating and manipulating multivectors, which are used to create semantic representations of data (e.g., crate names, emojis) in the Solfunmeme-Dioxus project. These multivectors are integrated into RDF graphs via the `solfunmeme_ontology_vibe` crate to enrich ontology data with geometric algebra-based semantics.

## Dependencies

`solfunmeme_clifford` depends on the following crates:
- **`tclifford`**: Provides the underlying Clifford algebra implementation.
- **`rand`**: Used for generating random coefficients in some multivector operations.
- **`serde`**: Enables serialization and deserialization of multivectors.
- **`anyhow`**: Simplifies error handling.
- **`sha2`**: Provides SHA-256 hashing for converting strings to multivectors.

## Core Functionality

The crate defines the following key components:

### `SolCliffordAlgebra`
- Represents a 3D Clifford algebra with basis vectors (e.g., e0, e1, e2).
- Provides operations for geometric products, outer products, and other algebraic manipulations.

### `SolMultivector`
- A struct representing a multivector in the 3D Clifford algebra.
- Contains coefficients for scalar, vector, bivector, and trivector components.
- Key methods:
  - `get_multivector_norm`: Computes the norm (magnitude) of the multivector.
  - `get_multivector_coefficients`: Returns the coefficients of the multivector's basis components.

### `generate_multivector_from_string`
- **Signature**: `fn generate_multivector_from_string(input: &str) -> Result<SolMultivector>`
- Converts a string input (e.g., a crate name or emoji) into a `SolMultivector` using SHA-256 hashing.
- The hash is used to derive deterministic coefficients for the multivector’s basis components, ensuring consistent semantic representations.

### `SerializableMultivector`
- A struct for serializing and deserializing `SolMultivector` instances.
- Uses `serde` to convert multivectors to/from formats like JSON or Turtle (when integrated with RDF).

## Usage in Solfunmeme-Dioxus

`solfunmeme_clifford` is primarily used by the `solfunmeme_ontology_vibe` crate, specifically in:
- `processor/crate_data.rs`: Generates multivectors for crate names and adds them as `onto:hasCliffordVector` triples to the RDF graph.
- `processor/emoji_data.rs`: Generates multivectors for emoji names and adds them as `onto:hasCliffordVector` triples.

Example usage (from `solfunmeme_ontology_vibe`):
```rust
use solfunmeme_clifford::generate_multivector_from_string;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;

fn add_crate_data_internal(graph: &mut RdfGraph) -> Result<()> {
    // Example: For each crate subject with rdfs:label
    let crate_name = "example_crate";
    let multivector = generate_multivector_from_string(crate_name)?;
    // Add multivector as a literal triple to the RDF graph
    // ...
    Ok(())
}
```

## Integration with Other Crates

`solfunmeme_clifford` is a dependency for multiple crates in the Solfunmeme-Dioxus project (15 Cargo.toml files reference it). Its multivectors are used to create semantic "vibes" for ontology entities, enabling advanced querying and reasoning in the `ragit` system.

## Notes for Developers
- **Deterministic Multivectors**: The use of SHA-256 ensures that the same string input always produces the same multivector, which is critical for consistent ontology data.
- **Extensibility**: Developers can extend `generate_multivector_from_string` to incorporate other embedding methods (e.g., BERT) by modifying the coefficient derivation logic.
- **Documentation Discoverability**: This file is linked from `docs/index.md` under the "Rust Crates" section. Inline documentation in `lib.rs` can be generated with `cargo doc`.

