use serde::{Deserialize, Serialize};
use crate::Uid;
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum IIStatus {
    /// Initial state. There's no ii at all.
    
    None,

    /// ii used to be `Complete` or `Ongoing`, but there're added or removed chunks.
    Outdated,

    /// ii is built and is usable.
    Complete,

    /// ii-building is still going on. `ii-build` commands will
    /// start from this uid. `ii-build` ALWAYS processes chunks in
    /// uid order.
    Ongoing(Uid),
}

