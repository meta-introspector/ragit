use tokio::sync::mpsc;
use std::path::PathBuf;
use ragit_types::uid::Uid;
use ragit_error::ApiError;

pub struct Channel {
    pub tx_from_main: mpsc::UnboundedSender<Request>,
    pub rx_to_main: mpsc::UnboundedReceiver<Response>,
}

impl Channel {
    pub fn send(&self, msg: Request) -> Result<(), mpsc::error::SendError<Request>> {
        self.tx_from_main.send(msg)
    }

    pub fn try_recv(&mut self) -> Result<Response, mpsc::error::TryRecvError> {
        self.rx_to_main.try_recv()
    }
}

#[derive(Debug)]
pub enum Request {
    BuildChunks { file: PathBuf },
    Kill,
}

#[derive(Debug)]
pub enum Response {
    FileComplete { file: PathBuf, chunk_count: usize },
    ChunkComplete { file: PathBuf, index: usize, chunk_uid: Uid },
    Error(ApiError),
}