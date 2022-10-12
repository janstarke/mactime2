use std::{
    io::stdin,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use anyhow::Result;

use crate::{stream::*, Joinable};

pub(crate) trait StreamReader<T>: Sized + StreamWorker<T> + Joinable<()>
where
    T: Send + 'static,
{
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
