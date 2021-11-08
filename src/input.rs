use std::{sync::{Mutex, mpsc::{Receiver, Sender}}, thread::Thread};

struct Input {}

struct Inputs {
    pub tx: Sender<Input>,
    pub rx: Receiver<Input>,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            tx: Arc::new(Mutex::new(t));
        }
    }
}
