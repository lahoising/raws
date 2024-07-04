use tokio::sync::mpsc::{Receiver, Sender};

pub struct DataLoader<T> {
    sender: Sender<T>,
    load_requested: bool,
    receiver: Receiver<T>,
    data: Option<T>,
}

impl<T> DataLoader<T> {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(1);
        Self {
            sender,
            load_requested: false,
            receiver,
            data: None,
        }
    }

    pub fn sender_clone(&self) -> Sender<T> {
        self.sender.clone()
    }

    pub fn mark_load_requested(&mut self) {
        self.load_requested = true;
    }

    pub fn load_requested(&self) -> bool {
        self.load_requested
    }

    pub fn poll(&mut self) {
        if self.data.is_some() {
            return;
        }
        if let Ok(data) = self.receiver.try_recv() {
            self.data = Some(data);
        }
    }

    pub fn data(&self) -> &Option<T> {
        &self.data
    }
}
