use std::{sync::mpsc, thread};

use super::{event_loop::event_loop, request, response, Channel};

pub fn init_workers(workers: usize, root_dir: &str) -> Vec<Channel> {
    (0..workers)
        .map(|_worker_id| {
            let (tx_to_main, rx_from_worker) = mpsc::channel::<response::Response>();
            let (tx_to_worker, rx_from_main) = mpsc::channel::<request::Request>();
            let root_dir = root_dir.to_string();

            thread::spawn(move || {
                event_loop(tx_to_main, rx_from_main, root_dir).unwrap();
            });

            Channel { tx: tx_to_worker, rx: rx_from_worker }
        })
        .collect()
}
