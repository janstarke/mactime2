use std::{thread::JoinHandle, sync::mpsc::{Receiver, Sender, self}};

use bodyfile::Bodyfile3Line;
use elastic4forensics::objects::PosixFile;
use serde_json::Value;

use crate::{Filter, RunOptions};

pub struct BodyfileConverter {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<Value>>,
}


impl Filter<Bodyfile3Line, Value> for BodyfileConverter {
    fn with_receiver(reader: Receiver<Bodyfile3Line>, options: RunOptions) -> Self {
        let (tx, rx): (Sender<Value>, Receiver<Value>) = mpsc::channel();
        Self {
            worker: Some(std::thread::spawn(move || {
                Self::worker(reader, tx, options)
            })),
            rx: Some(rx),
        }
    }

    fn get_receiver(&mut self) -> Receiver<Value> {
        self.rx.take().unwrap()
    }

    fn worker(reader: Receiver<Bodyfile3Line>, tx: std::sync::mpsc::Sender<Value>, _options: crate::RunOptions) {
        loop {
            let bf_line = match reader.recv() {
                Err(_) => break,
                Ok(l) => l
            };

            let pfile = PosixFile::from(bf_line);
            for doc in pfile.documents() {
                if tx.send(doc).is_err() {
                    return;
                }
            }
        }
    }
}