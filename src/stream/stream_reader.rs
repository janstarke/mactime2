use std::{sync::mpsc::{Receiver, Sender, self}, thread::{JoinHandle, self}, io::stdin};

use anyhow::Result;

use crate::stream::*;

pub(crate) trait StreamReader<T>: Sized + StreamWorker<T> where T: Send + 'static {
    fn from(filename: &Option<String>) -> Result<Self> {
        let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();

        let worker = match StreamSource::from(filename)? {
            StreamSource::Stdin => thread::spawn(move || {
                <Self as StreamWorker<T>>::worker(stdin(), tx);
            }),
            StreamSource::File(f) => thread::spawn(move || {
                <Self as StreamWorker<T>>::worker(f, tx);
            }),
        };

        Ok(<Self as StreamReader<T>>::new(worker, rx))
    }

    fn new(worker: JoinHandle<()>, rx: Receiver<T>) -> Self;
    fn get_receiver(&mut self) -> Receiver<T>;
}
