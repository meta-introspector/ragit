#[derive(Clone)]
pub struct MemorySnapshot {
    pub step: String,
    pub total_memory: u64,
    pub total_delta: i64,
    pub used_memory: u64,
    pub used_delta: i64,
    pub process_rss: u64,
    pub rss_delta: i64,
}
