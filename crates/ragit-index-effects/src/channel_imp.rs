use tokio::sync::mpsc;
use ragit_api::{Request, Response};
use crate::channel::Channel;

impl Channel {
    pub fn send(&self, msg: ragit_api::Request) -> Result<(), mpsc::error::SendError<ragit_api::Request>> {
        self.tx_from_main.send(msg)
    }

    pub fn try_recv(&mut self) -> Result<ragit_api::Response, mpsc::error::TryRecvError> {
        self.rx_to_main.try_recv()
    }
}
