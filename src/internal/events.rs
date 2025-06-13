use std::sync::mpsc::{self, Receiver, SendError, Sender};

#[derive(Debug, Clone, PartialEq)]
pub enum InternalEvent {
    Initialized,
}

pub struct EventBus {
    receiver: Receiver<InternalEvent>,
    sender: Sender<InternalEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<InternalEvent>();
        return Self { sender, receiver };
    }

    pub fn notify(&self, event: InternalEvent) -> Result<(), SendError<InternalEvent>> {
        return self.sender.send(event);
    }
}
