use std::sync::mpsc::{Sender, Receiver};

pub trait Filter<From, To> {
    fn with_receiver(previous: Receiver<From>) -> Self;
    fn get_receiver(&mut self) -> Receiver<To>;
    fn worker(reader: Receiver<From>, tx: Sender<To>);
}

pub trait Joinable {
    fn join(&mut self) -> std::thread::Result<()>;
}