use tokio::sync::mpsc;
use ragit_types::ApiError;
use ragit_types::uid::Uid;
use std::path::PathBuf;

#[derive(Debug)]
pub enum WorkerRequest {
    Kill,
    BuildChunks { file: PathBuf },
}

#[derive(Debug)]
pub enum WorkerResponse {
    ChunkComplete {
        file: String,
        chunk_uid: Uid,
        index: usize,
    },
    FileComplete {
        file: String,
        chunk_count: usize,
    },
    Error(ApiError),
}

pub struct Channel {
    pub tx_from_main: mpsc::UnboundedSender<WorkerRequest>,
    pub rx_to_main: mpsc::UnboundedReceiver<WorkerResponse>,
}

impl Channel {
    pub fn send(&self, msg: WorkerRequest) -> Result<(), mpsc::error::SendError<WorkerRequest>> {
        self.tx_from_main.send(msg)
    }

    pub fn try_recv(&mut self) -> Result<WorkerResponse, mpsc::error::TryRecvError> {
        self.rx_to_main.try_recv()
    }
}