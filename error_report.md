^C
~/storage/solfunmeme-dioxus $ ../github/ragit/target/debug/rag build
---
elapsed time: 80:08
staged files: 835, processed files: 187
errors: 357
---
elapsed time: 80:09
staged files: 835, processed files: 187
errors: 357
committed chunks: 197
buffered files: 26, buffered chunks: 18
flush count: 2
model: llama3.3-70b-groq
input tokens: 143840 (0.085$), output tokens: 12844 (0.010$)
   0:       0x702d06ce18 - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::hd07606ddb6eec661
   1:       0x702d08d874 - core::fmt::write::hb83521df870a89bf
   2:       0x702d04b0e0 - std::io::default_write_fmt::ha22b84d014fbb72f
   3:       0x702d06d4f4 - std::panicking::default_hook::{{closure}}::h6911909537cbc5b0
   4:       0x702d06d3bc - std::panicking::default_hook::hfb8348094d22159e
   5:       0x702d06dc7c - std::panicking::rust_panic_with_hook::h0c46ac19880364ba
   6:       0x702d06d914 - std::panicking::begin_panic_handler::{{closure}}::h04bf73feeeda5246
   7:       0x702d06d080 - std::sys::backtrace::__rust_end_short_backtrace::h8ea58025c42cd029
   8:       0x702d06d5ec - __rustc[b723cc1e8dfe1f7e]::rust_begin_unwind
   9:       0x702d09a4ac - core::panicking::panic_fmt::h640feb197dad2f42
  10:       0x702c00a83c - ragit_api::message::message_to_json::h558375838c0b0de7
                               at /data/data/com.termux/files/home/storage/github/ragit/crates/api/src/message.rs:82:19
  11:       0x702c0149f8 - ragit_api::request::Request::build_json_body::h14498db60d501848
                               at /data/data/com.termux/files/home/storage/github/ragit/crates/api/src/request.rs:125:35
  12:       0x702bfe7b00 - ragit_api::request::Request::send::{{closure}}::hfc318d3c32a73380
                               at /data/data/com.termux/files/home/storage/github/ragit/crates/api/src/request.rs:226:20
  13:       0x702be7aaac - ragit_api::request::Request::send_and_validate::{{closure}}::h60a580fbe91c0e83
                               at /data/data/com.termux/files/home/storage/github/ragit/crates/api/src/request.rs:193:41
  14:       0x702bd53e24 - ragit::chunk::Chunk::create_chunk_from::{{closure}}::h38c63013656cd4af
                               at /data/data/com.termux/files/home/storage/github/ragit/src/chunk.rs:266:117
  15:       0x702bed05e0 - ragit::index::file::FileReader::generate_chunk::{{closure}}::hee9c422894dcd937
                               at /data/data/com.termux/files/home/storage/github/ragit/src/index/file.rs:217:11
  16:       0x702be9a274 - ragit::index::commands::build::build_chunks::{{closure}}::hedd95d11ef246755
                               at /data/data/com.termux/files/home/storage/github/ragit/src/index/commands/build.rs:469:11
  17:       0x702be9b140 - ragit::index::commands::build::init_worker::{{closure}}::he88aeae75063c5df
                               at /data/data/com.termux/files/home/storage/github/ragit/src/index/commands/build.rs:595:19
  18:       0x702bd8782c - <core::pin::Pin<P> as core::future::future::Future>::poll::haa7e1811ece2fb25
                               at /home/builder/.termux-build/rust/src/library/core/src/future/future.rs:124:9
  19:       0x702becdef4 - tokio::runtime::task::core::Core<T,S>::poll::{{closure}}::h1708b14e3b4c8123
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/core.rs:365:17
  20:       0x702becdaf4 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h2ac0cb0681691469
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/loom/std/unsafe_cell.rs:16:9
  21:       0x702becdaf4 - tokio::runtime::task::core::Core<T,S>::poll::h186f44b15eb5790a
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/core.rs:354:13
  22:       0x702beb1948 - tokio::runtime::task::harness::poll_future::{{closure}}::hf0a5ba3de00672c3
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:535:19
  23:       0x702beb08ec - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h023dbfd6112dca6c
                               at /home/builder/.termux-build/rust/src/library/core/src/panic/unwind_safe.rs:272:9
  24:       0x702be82410 - std::panicking::try::do_call::h915bb5c2223ad6b7
                               at /home/builder/.termux-build/rust/src/library/std/src/panicking.rs:589:40
  25:       0x702be82058 - __rust_try
  26:       0x702be81f60 - std::panicking::try::h37b58fc45063b502
                               at /home/builder/.termux-build/rust/src/library/std/src/panicking.rs:552:19
  27:       0x702be81f60 - std::panic::catch_unwind::hf579794a055e7126
                               at /home/builder/.termux-build/rust/src/library/std/src/panic.rs:359:14
  28:       0x702beb0f6c - tokio::runtime::task::harness::poll_future::h406e3ad7b3dad8f6
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:523:18
  29:       0x702beb1d80 - tokio::runtime::task::harness::Harness<T,S>::poll_inner::h96259683314c4a0c
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:210:27
  30:       0x702beb2a04 - tokio::runtime::task::harness::Harness<T,S>::poll::hdd1b13b5285934be
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:155:15
  31:       0x702becf2fc - tokio::runtime::task::raw::poll::h441cc4e36c061f49
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/raw.rs:325:5
  32:       0x702c3479f8 - tokio::runtime::task::raw::RawTask::poll::h79c317fe80df02fc
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/raw.rs:255:18
  33:       0x702c3778b0 - tokio::runtime::task::LocalNotified<S>::run::h68187c185857d66d
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/mod.rs:509:9
  34:       0x702c36746c - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::{{closure}}::h62581bf7f13b707b
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/scheduler/multi_thread/worker.rs:600:13
  35:       0x702c36735c - tokio::task::coop::with_budget::haca11063d8daf3e3
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/task/coop/mod.rs:167:5
  36:       0x702c36735c - tokio::task::coop::budget::he05f3a1f70540f42
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/task/coop/mod.rs:133:5
  37:       0x702c36735c - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::ha85926a64c8f89c3
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/scheduler/multi_thread/worker.rs:591:9
  38:       0x702c366a70 - tokio::runtime::scheduler::multi_thread::worker::Context::run::hcc50b04d02e4d3ec
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/scheduler/multi_thread/worker.rs:539:24
  39:       0x702c3665e0 - tokio::runtime::scheduler::multi_thread::worker::run::{{closure}}::{{closure}}::h72384c8f8ec1db82
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/scheduler/multi_thread/worker.rs:504:21
  40:       0x702c375a4c - tokio::runtime::context::scoped::Scoped<T>::set::hc0089eeebfc430f4
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/context/scoped.rs:40:9
  41:       0x702c398b50 - tokio::runtime::context::set_scheduler::{{closure}}::h129349ae16873d75
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/context.rs:176:26
  42:       0x702c33d00c - std::thread::local::LocalKey<T>::try_with::h22aa17a37680e319
                               at /home/builder/.termux-build/rust/src/library/std/src/thread/local.rs:315:12
  43:       0x702c33cd40 - std::thread::local::LocalKey<T>::with::h729d370d97a3a9ad
                               at /home/builder/.termux-build/rust/src/library/std/src/thread/local.rs:279:15
  44:       0x702c398adc - tokio::runtime::context::set_scheduler::hcbcbabbce01f4b73
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/context.rs:176:9
  45:       0x702c36650c - tokio::runtime::scheduler::multi_thread::worker::run::{{closure}}::hef4488a752e6c1bd
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/scheduler/multi_thread/worker.rs:499:9
  46:       0x702c398620 - tokio::runtime::context::runtime::enter_runtime::h3c76c474d9e566ba
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/context/runtime.rs:65:16
  47:       0x702c366368 - tokio::runtime::scheduler::multi_thread::worker::run::hfd0daa1d60cd50f6
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/scheduler/multi_thread/worker.rs:491:5
  48:       0x702c366108 - tokio::runtime::scheduler::multi_thread::worker::Launch::launch::{{closure}}::h92b72aefc0f0236b
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/scheduler/multi_thread/worker.rs:457:45
  49:       0x702c351f68 - <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll::h70caddc1622d4131
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/blocking/task.rs:42:21
  50:       0x702c334708 - tokio::runtime::task::core::Core<T,S>::poll::{{closure}}::h057919f12f40da94
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/core.rs:365:17
  51:       0x702c3345b4 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h79fdcf1316f35b9a
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/loom/std/unsafe_cell.rs:16:9
  52:       0x702c3345b4 - tokio::runtime::task::core::Core<T,S>::poll::hf5d6b7a13b237779
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/core.rs:354:13
  53:       0x702c333658 - tokio::runtime::task::harness::poll_future::{{closure}}::h13ecd793cb05bcbe
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:535:19
  54:       0x702c3736fc - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hdc092a02d7185d2b
                               at /home/builder/.termux-build/rust/src/library/core/src/panic/unwind_safe.rs:272:9
  55:       0x702c343f44 - std::panicking::try::do_call::h6e2f9d04aa5a1893
                               at /home/builder/.termux-build/rust/src/library/std/src/panicking.rs:589:40
  56:       0x702c36dbf8 - __rust_try
  57:       0x702c36beb4 - std::panicking::try::h26d262b7f008b2e0
                               at /home/builder/.termux-build/rust/src/library/std/src/panicking.rs:552:19
  58:       0x702c36beb4 - std::panic::catch_unwind::hb0d0872058ea90ca
                               at /home/builder/.termux-build/rust/src/library/std/src/panic.rs:359:14
  59:       0x702c3334b4 - tokio::runtime::task::harness::poll_future::heb22cb88982d668f
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:523:18
  60:       0x702c3321c8 - tokio::runtime::task::harness::Harness<T,S>::poll_inner::h9dba685c573e51e6
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:210:27
  61:       0x702c331ff4 - tokio::runtime::task::harness::Harness<T,S>::poll::hb0234ba5d539ba06
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/harness.rs:155:15
  62:       0x702c347d68 - tokio::runtime::task::raw::poll::h9c092bc6a45b2670
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/raw.rs:325:5
  63:       0x702c3479f8 - tokio::runtime::task::raw::RawTask::poll::h79c317fe80df02fc
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/raw.rs:255:18
  64:       0x702c37796c - tokio::runtime::task::UnownedTask<S>::run::h9186b0730e0bd264
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/task/mod.rs:546:9
  65:       0x702c36fc60 - tokio::runtime::blocking::pool::Task::run::h5af20f769f9ad40a
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/blocking/pool.rs:161:9
  66:       0x702c371894 - tokio::runtime::blocking::pool::Inner::run::h505107c1dba50d74
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/blocking/pool.rs:516:17
  67:       0x702c371610 - tokio::runtime::blocking::pool::Spawner::spawn_thread::{{closure}}::h04ee9f73f21bc535
                               at /data/data/com.termux/files/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.46.1/src/runtime/blocking/pool.rs:474:13
  68:       0x702c36b974 - std::sys::backtrace::__rust_begin_short_backtrace::hfb9075e5c9ea2a20
                               at /home/builder/.termux-build/rust/src/library/std/src/sys/backtrace.rs:152:18
  69:       0x702c380ae0 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::h6a6b97d255f4f27f
                               at /home/builder/.termux-build/rust/src/library/std/src/thread/mod.rs:559:17
  70:       0x702c373764 - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hea77d8d5d09d8838
                               at /home/builder/.termux-build/rust/src/library/core/src/panic/unwind_safe.rs:272:9
  71:       0x702c343e0c - std::panicking::try::do_call::h212b6f9df43f7c23
                               at /home/builder/.termux-build/rust/src/library/std/src/panicking.rs:589:40
  72:       0x702c38a644 - __rust_try
  73:       0x702c380654 - std::panicking::try::he250f0e86b24d684
                               at /home/builder/.termux-build/rust/src/library/std/src/panicking.rs:552:19
---
elapsed time: 80:09
staged files: 835, processed files: 187
errors: 357
committed chunks: 197
buffered files: 26, buffered chunks: 18
flush count: 2
model: llama3.3-70b-groq
input tokens: 143840 (0.085$), output tokens: 12844 (0.010$)
---
Failed to build a knowledge-base
MPSCError("Build worker hung up.")