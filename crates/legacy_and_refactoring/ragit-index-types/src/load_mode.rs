//use serde::{Deserialize, Serialize};

//pub use ragit_index_types::load_mode::LoadMode;

/// 1. If you want to do something with chunks, use `LoadMode::QuickCheck`.
/// 2. If you have nothing to do with chunks, use `LoadMode::OnlyJson`.
/// 3. If something's broken and you don't want it to crash, use `LoadMode::Minimum`. It can still crash, though.
/// 4. If you want to be very sure that nothing's broken and you don't care about init-time, use `LoadMode::Check`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LoadMode {
    /// It only loads `index.json`. It doesn't care whether config files prompt files, or chunk files are broken.
    Minimum,

    /// It loads json files, but doesn't care whether the chunk files are broken.
    OnlyJson,

    /// It checks and auto-recovers if `self.curr_processing_file` is not None. If the value is not None,
    /// a previous build was interrupted and something could be broken.
    QuickCheck,

    /// It always checks and auto-recovers. You should be very careful, `check` and `auto-recover` are very expensive.
    Check,
}
