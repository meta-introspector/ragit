use crate::index::commands::archive::create::channel::Channel;
use crate::index::commands::archive::create::init_worker;
use std::path::PathBuf;

pub(crate) fn init_workers(n: usize, root_dir: &PathBuf, compression_level: u32) -> Vec<Channel> {
    (0..n).map(|i| init_worker(i, root_dir.clone(), compression_level)).collect()
}
