impl Channel {
    pub fn send(&self, msg: Request) -> Result<(), mpsc::error::SendError<Request>> {
        self.tx_from_main.send(msg)
    }

    pub fn try_recv(&mut self) -> Result<Response, mpsc::error::TryRecvError> {
        self.rx_to_main.try_recv()
    }
}
