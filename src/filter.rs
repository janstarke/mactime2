use std::sync::mpsc::{Sender, Receiver};

#[derive(Copy, Clone)]
pub struct RunOptions {
    pub strict_mode: bool
}
pub trait Filter<From, To> {
    fn with_receiver(previous: Receiver<From>, options: RunOptions) -> Self;
    fn get_receiver(&mut self) -> Receiver<To>;
    fn worker(reader: Receiver<From>, tx: Sender<To>, options: RunOptions);
}

pub trait Joinable<R> {
    fn join(&mut self) -> std::thread::Result<R>;
}