#!/bin/bash

# Fix syntax errors in impl_backend_storage_for_cpu_storage.rs
sed -i 's/device_fn::DeviceFn::device(self.device()self)/device_fn::DeviceFn::device(self)/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/impl_backend_storage_for_cpu_storage.rs
sed -i 's/copy_strided_src_fn::CopyStridedSrcFn::copy_strided_src(kernel.copy_strided_srckernel, kernel.copy_strided_srcmut kernel_c, 0, kernel_l)(&mut kernel_c, 0, kernel_l)?;/copy_strided_src_fn::CopyStridedSrcFn::copy_strided_src(kernel, &mut kernel_c, 0, kernel_l)?;/' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/impl_backend_storage_for_cpu_storage.rs
sed -i 's/matmul_fn::MatMulFn::matmul(self.matmulself, kernel, bmnk, lhs_l, rhs_l)(/matmul_fn::MatMulFn::matmul(self, kernel, bmnk, lhs_l, rhs_l)/g' /data/data/com.termux/files/home/storage/github/ragit/vendor/meta-introspector/solfunmeme-dioxus/vendor/candle/candle-core/src/cpu_backend/ops/impl_backend_storage_for_cpu_storage.rs
