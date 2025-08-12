use crate::index::commands::archive::create::channel::Channel;
use crate::index::commands::archive::create::event_loop;
use crate::index::commands::archive::create::response::Response;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub(crate) fn init_worker(worker_id: usize, root_dir: PathBuf, compression_level: u32) -> Channel {
    let (tx_to_main, rx_to_main) = mpsc::channel();
    let (tx_from_main, rx_from_main) = mpsc::channel();

    thread::spawn(move || match event_loop(
        tx_to_main.clone(),
        rx_from_main,
        root_dir.to_str().unwrap().to_string(),
        worker_id,
        compression_level,
    ) {
        Ok(()) => {},
        Err(e) => {
            tx_to_main.send(Response::Error(e)).unwrap();
        },
    });

    Channel {
        rx_to_main, tx_from_main
    }
}
