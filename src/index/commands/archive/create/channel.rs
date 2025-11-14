use std::sync::mpsc;
use crate::index::commands::archive::create::request::Request;
use crate::index::commands::archive::create::response::Response;

pub(crate) struct Channel {
    pub(crate) tx_from_main: mpsc::Sender<Request>,
    pub(crate) rx_to_main: mpsc::Receiver<Response>,
}

impl Channel {
    pub fn send(&self, msg: Request) -> Result<(), mpsc::SendError<Request>> {
        self.tx_from_main.send(msg)
    }

    pub fn try_recv(&self) -> Result<Response, mpsc::TryRecvError> {
        self.rx_to_main.try_recv()
    }
}
