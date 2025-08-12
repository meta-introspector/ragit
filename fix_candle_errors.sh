#!/bin/bash

# FixCpuDevice import in device.rs
sed -i 's/use crate::cpu_backend::prelude::CpuDevice;/use crate::cpu_backend::cpu_storage::CpuDevice;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/device.rs

# Fix CpuStorageRef import in dtype.rs
sed -i 's/use crate::cpu_backend::prelude::{CpuStorage, CpuStorageRef};/use crate::cpu_backend::cpu_storage::{CpuStorage, CpuStorageRef};/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/dtype.rs

# Fix CpuStorageRef import in lib.rs
sed -i 's/pub use cpu_backend::prelude::{CpuStorage, CpuStorageRef};/pub use cpu_backend::cpu_storage::{CpuStorage, CpuStorageRef};/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/lib.rs

# Remove feature gates above commented-out modules in lib.rs
sed -i '/^#\[cfg(feature = "accelerate")\]
\/\/ mod accelerate;/
/d' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/lib.rs
sed -i '/^#\[cfg(feature = "mkl")\]
\/\/ mod mkl;/
/d' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/lib.rs

# Comment out accelerate and mkl extern crates in lib.rs
sed -i 's/extern crate intel_mkl_src;/\/\/ extern crate intel_mkl_src;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/lib.rs
sed -i 's/extern crate accelerate_src;/\/\/ extern crate accelerate_src;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/lib.rs

# Fix missing field `reduce_dims_and_stride` in ReduceSum initializer (placeholder for now)
sed -i 's/ReduceSum {/ReduceSum { reduce_dims_and_stride: (),/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/impl_backend_storage_for_cpu_storage.rs

# Revert problematic ambiguous method call fixes (will re-evaluate later)
sed -i 's/device_fn::DeviceFn::device(&self)/self.device()/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/impl_backend_storage_for_cpu_storage.rs
sed -i 's/copy_strided_src_fn::CopyStridedSrcFn::copy_strided_src(&kernel, &mut kernel_c, 0, kernel_l)/kernel.copy_strided_src(&mut kernel_c, 0, kernel_l)/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/impl_backend_storage_for_cpu_storage.rs
sed -i 's/matmul_fn::MatMulFn::matmul(&self, kernel, bmnk, lhs_l, rhs_l)/self.matmul(kernel, bmnk, lhs_l, rhs_l)/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/impl_backend_storage_for_cpu_storage.rs

# Revert problematic MatMul related fixes (will re-evaluate later)
sed -i 's/let (b, m, n, k) = self.params;/let (b, m, n, k) = self.0;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_accelerate.rs
sed -i 's/self.params.ab_skip(lhs_l, rhs_l)?;/self.ab_skip(lhs_l, rhs_l)?;/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_accelerate.rs
sed -i 's/self.params.striding_error(lhs_l, rhs_l, "non-contiguous rhs")/self.striding_error(lhs_l, rhs_l, "non-contiguous rhs")/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_accelerate.rs
sed -i 's/self.params.striding_error(lhs_l, rhs_l, "non-contiguous lhs")/self.striding_error(lhs_l, rhs_l, "non-contiguous lhs")/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_accelerate.rs

sed -i 's/let (b, m, n, k) = self.params;/let (b, m, n, k) = self.0;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_mkl.rs
sed -i 's/self.params.ab_skip(lhs_l, rhs_l)?;/self.ab_skip(lhs_l, rhs_l)?;/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_mkl.rs
sed -i 's/self.params.striding_error(lhs_l, rhs_l, "non-contiguous rhs")/self.striding_error(lhs_l, rhs_l, "non-contiguous rhs")/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_mkl.rs
sed -i 's/self.params.striding_error(lhs_l, rhs_l, "non-contiguous lhs")/self.striding_error(lhs_l, rhs_l, "non-contiguous lhs")/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_mkl.rs

sed -i 's/let (b, m, n, k) = self.params;/let (b, m, n, k) = self.0;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_fallback.rs
sed -i 's/self.params.ab_skip(lhs_l, rhs_l)?;/self.ab_skip(lhs_l, rhs_l)?;/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/mat_mul/mat_mul_fallback.rs

sed -i 's/MatMulFallback(bmnk).map(self, lhs_l, rhs, rhs_l)/MatMul(bmnk).map(self, lhs_l, rhs, rhs_l)/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/matmul_fn.rs

# Fix `len()` and indexing on `CpuStorage` in `copy_strided_src_fn.rs`
sed -i 's/dst.len()/dst.as_slice::<u8>()?.len()/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/copy_strided_src_fn.rs
sed -i 's/self[src_index]/self.as_slice::<u8>()?[src_index]/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/copy_strided_src_fn.rs
sed -i 's/dst[dst_offset..dst_offset + to_copy]/dst.as_slice_mut::<u8>()?[dst_offset..dst_offset + to_copy]/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/copy_strided_src_fn.rs
sed -i 's/self[src_index..src_index + to_copy]/self.as_slice::<u8>()?[src_index..src_index + to_copy]/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/copy_strided_src_fn.rs
sed -i 's/dst[dst_index] = self[src_index]/dst.as_slice_mut::<u8>()?[dst_index] = self.as_slice::<u8>()?[src_index]/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/copy_strided_src_fn.rs

# Fix Mismatched Types in conv1d_fn.rs and conv2d_fn.rs
sed -i 's/BackendDevice::alloc_uninit(&self.device(), res_l.shape(), res.dtype())?/CpuStorage::new(res_l.shape(), res.dtype(), &self.device())?/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/conv1d_fn.rs
sed -i 's/BackendStorage::copy_strided_src(&res, &mut res_t, 0, &res_l)?;/res_t.copy_strided_src(&res, 0, &res_l)?;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/conv1d_fn.rs
sed -i 's/Ok(res_t)/Ok(res_t)/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/conv1d_fn.rs

sed -i 's/BackendDevice::alloc_uninit(&self.device(), res_l.shape(), res.dtype())?/CpuStorage::new(res_l.shape(), res.dtype(), &self.device())?/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/conv2d_fn.rs
sed -i 's/BackendStorage::copy_strided_src(&res, &mut res_t, 0, &res_l)?;/res_t.copy_strided_src(&res, 0, &res_l)?;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/conv2d_fn.rs
sed -i 's/Ok(res_t)/Ok(res_t)/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/conv2d_fn.rs
