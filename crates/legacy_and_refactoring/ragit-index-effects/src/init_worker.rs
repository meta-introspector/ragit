use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::channel::Channel;
use crate::run_worker_task::run_worker_task;

pub fn init_worker(root_dir: PathBuf) -> Channel {
    let (tx_to_main, rx_to_main) = mpsc::unbounded_channel();
    let (tx_from_main, rx_from_main) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        if let Err(e) = run_worker_task(root_dir, tx_to_main, rx_from_main).await {
            eprintln!("Worker task failed: {}", e);
        }
    });

    Channel {
        rx_to_main,
        tx_from_main,
    }
}