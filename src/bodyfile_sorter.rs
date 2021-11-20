use crate::Joinable;
use bodyfile::Bodyfile3Line;
use std::sync::mpsc::{Receiver};
use std::thread::{JoinHandle};

pub struct BodyfileSorter {
    worker: Option<JoinHandle<()>>
}

fn worker(decoder: Receiver<Bodyfile3Line>) {
    loop {
        let line = match decoder.recv() {
            Err(_) => {break;}
            Ok(l) => l
        };

        println!("{}", line);
    }
}

impl BodyfileSorter {
    pub fn with_receiver(decoder: Receiver<Bodyfile3Line>) -> Self {
        Self {
            worker: Some(std::thread::spawn(move || {
                worker(decoder)
            }))
        }
    }
}

impl Joinable for BodyfileSorter {
    fn join(&mut self) -> std::thread::Result<()> {
        self.worker.take().unwrap().join()
    }
}