use std::collections::HashMap;
use std::time::Instant;
use crate::index::commands::archive::BlockType;

pub(crate) struct Status {
    pub(crate) started_at: Instant,
    pub(crate) block_count: HashMap<BlockType, usize>,
}
