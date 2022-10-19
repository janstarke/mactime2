use std::{thread::JoinHandle, sync::mpsc::{Receiver, Sender, self}};

use bodyfile::Bodyfile3Line;
use es4forensics::objects::PosixFile;

use crate::{Filter, RunOptions, Provider, Consumer, Joinable};

pub struct BodyfileConverter {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<PosixFile>>,
}

impl Provider<PosixFile, ()> for BodyfileConverter {
    fn get_receiver(&mut self) -> Receiver<PosixFile> {
        self.rx.take().unwrap()
    }
}

impl Consumer<Bodyfile3Line> for BodyfileConverter {
    fn with_receiver(reader: Receiver<Bodyfile3Line>, options: RunOptions) -> Self {
        let (tx, rx): (Sender<PosixFile>, Receiver<PosixFile>) = mpsc::channel();
        Self {
            worker: Some(std::thread::spawn(move || {
                Self::worker(reader, tx, options)
            })),
            rx: Some(rx),
        }
    }
}

impl Filter<Bodyfile3Line, PosixFile, ()> for BodyfileConverter {
    fn worker(reader: Receiver<Bodyfile3Line>, tx: std::sync::mpsc::Sender<PosixFile>, _options: crate::RunOptions) {
        loop {
            let bf_line = match reader.recv() {
                Err(_) => break,
                Ok(l) => l
            };

            let pfile = PosixFile::from(bf_line);
            if tx.send(pfile).is_err() {
                return;
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