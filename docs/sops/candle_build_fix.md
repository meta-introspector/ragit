# SOP: Candle Build Fix Documentation

## 1. Purpose
This Standard Operating Procedure (SOP) documents the steps taken to address compilation issues related to the `candle` crate, specifically focusing on disabling problematic dependencies like `gemm` that cause build failures on certain architectures (e.g., AArch64 Android). This ensures a clear record of modifications and their rationale for future reference and reproducibility.

## 2. Scope
This SOP applies to modifications made within the `vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/` directory and its sub-crates, specifically targeting `Cargo.toml` files to manage dependencies.

## 3. Procedure

### 3.1. Initial Problem Identification
The `candle` crate, a vendored dependency, was causing build failures due to a transitive dependency on `gemm-common` and `fullfp16`, which are not compatible with the target architecture (AArch64 Android). This was identified during `cargo test` runs.

### 3.2. Analysis of `candle` Workspace `Cargo.toml`
The top-level `Cargo.toml` for the `candle` workspace (`vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/Cargo.toml`) was inspected to identify direct and workspace-level dependencies. The `gemm` dependency was found listed under `[workspace.dependencies]`.

### 3.3. Modification to Disable `gemm` Dependency
To prevent `gemm` from being pulled into the build, the `gemm` entry in the `[workspace.dependencies]` section of `vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/Cargo.toml` was commented out.

**Specific Change:**
```toml
# Before:
gemm = { version = "0.17.0", features = ["wasm-simd128-enable"] }

# After:
# gemm = { version = "0.17.0", features = ["wasm-simd128-enable"] }
```
*(Note: The line was already commented out in `candle-core/Cargo.toml`, but was present in the top-level `candle/Cargo.toml` workspace dependencies.)*

### 3.6. `candle-core/src/cpu_backend/mod.rs` Modifications
The `MatMul` implementation in `candle-core/src/cpu_backend/mod.rs` was modified to prioritize `accelerate` or `mkl` features over `gemm`. The `gemm` related code block was commented out, and the `#[cfg(feature = "accelerate")]` and `#[cfg(feature = "mkl")]` blocks were made active. This ensures that `candle` uses available optimized backends instead of the problematic `gemm` dependency.

**Specific Change (Conceptual):**
```rust
// Original gemm implementation (commented out)
/*
fn f<T: 'static + WithDType + num_traits::Num + Copy>(
    &self,
    lhs: &[T],
    lhs_l: &Layout,
    rhs: &[T],
    rhs_l: &Layout,
) -> Result<Vec<T>> {
    use gemm::{gemm, Parallelism};
    // ... gemm implementation ...
}
*/

// Active accelerate implementation
#[cfg(feature = "accelerate")]
fn f<T: 'static + WithDType + num_traits::Num + Copy>(
    // ... accelerate implementation ...
)

// Active mkl implementation
#[cfg(feature = "mkl")]
fn f<T: 'static + WithDType + num_traits::Num + Copy>(
    // ... mkl implementation ...
)
```


### 3.4. Verification Attempt and Subsequent Issues
After commenting out the `gemm` dependency, a `cargo build` was attempted from the project root. This revealed a new set of compilation errors in the main `ragit` crate, primarily "unresolved module or unlinked crate" errors for `ragit_agent`, `ragit_types`, `ragit_utils`, `ragit_index`, `ragit_model`, `ragit_model_query_response`, `lazy_static`, `regex`, `serde`, `serde_json`, `tokio`, and `sha3`. This indicates that while the `candle` specific issue might be resolved, the main project's `Cargo.toml` requires further dependency management.

## 4. Tools
*   `git status`
*   `read_file` tool
*   `replace` tool
*   `run_shell_command` (for `cargo build`)

## 5. Expected Outcome
*   The `gemm` dependency is successfully excluded from the `candle` build process.
*   `candle-core` utilizes `accelerate` or `mkl` for `MatMul` operations, avoiding `gemm`.
*   A clear record of the changes made to `candle`'s `Cargo.toml` and `candle-core/src/cpu_backend/mod.rs` is maintained.
*   Identification of subsequent dependency issues in the main `ragit` crate, requiring further attention.

