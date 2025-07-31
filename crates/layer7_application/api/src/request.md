// This file is deprecated. The `Request` struct has been refactored into an `enum Request`
// located at `crates/api/src/request/mod.rs`.
//
// All fields of the old `struct Request` are now part of the `ChatRequest` variant within the new enum.
//
// For detailed documentation on the new API Request structure, please refer to `docs/api_request_structure.md`.

pub use crate::request::Request;