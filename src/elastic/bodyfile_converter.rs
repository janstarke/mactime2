use std::{thread::JoinHandle, sync::mpsc::{Receiver, Sender, self}};

use bodyfile::Bodyfile3Line;
use elastic4forensics::objects::PosixFile;
use serde_json::Value;

use crate::{Filter, RunOptions, Provider, Consumer, Joinable};

pub struct BodyfileConverter {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<Value>>,
}

impl Provider<Value, ()> for BodyfileConverter {
    fn get_receiver(&mut self) -> Receiver<Value> {
        self.rx.take().unwrap()
    }
}

impl Consumer<Bodyfile3Line> for BodyfileConverter {
    fn with_receiver(reader: Receiver<Bodyfile3Line>, options: RunOptions) -> Self {
        let (tx, rx): (Sender<Value>, Receiver<Value>) = mpsc::channel();
        Self {
            worker: Some(std::thread::spawn(move || {
                Self::worker(reader, tx, options)
            })),
            rx: Some(rx),
        }
    }
}

impl Filter<Bodyfile3Line, Value, ()> for BodyfileConverter {
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

impl Joinable<()> for BodyfileConverter {
    fn join(&mut self) -> std::thread::Result<()> {
        match self.worker.take() {
            Some(w) => w.join(),
            None => Ok(()),
        }
    }
}