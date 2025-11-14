use std::collections::HashMap;
use std::time::Instant;

use super::BlockType;

pub struct Status {
    pub(super) started_at: Instant,
    pub(super) block_count: HashMap<BlockType, usize>,
    pub(super) block_complete: HashMap<BlockType, usize>,
}