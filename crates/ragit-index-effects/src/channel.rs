use tokio::sync::mpsc;
use ragit_api::{Request, Response};

pub struct Channel {
    pub tx_from_main: mpsc::UnboundedSender<ragit_api::Request>,
    pub rx_to_main: mpsc::UnboundedReceiver<ragit_api::Response>,
}

impl Channel {
    pub fn send(&self, msg: ragit_api::Request) -> Result<(), mpsc::error::SendError<ragit_api::Request>> {
        self.tx_from_main.send(msg)
    }

    pub fn try_recv(&mut self) -> Result<ragit_api::Response, mpsc::error::TryRecvError> {
        self.rx_to_main.try_recv()
    }
}

