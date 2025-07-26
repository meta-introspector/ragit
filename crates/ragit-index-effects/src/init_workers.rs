use std::path::PathBuf;
use crate::channel::Channel;
use crate::init_worker::init_worker;

pub fn init_workers(n: usize, root_dir: PathBuf) -> Vec<Channel> {
    (0..n).map(|_| init_worker(root_dir.clone())).collect()
}