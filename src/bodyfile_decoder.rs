use crate::{Filter, Joinable};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{JoinHandle};
use bodyfile::Bodyfile3Line;
use std::convert::TryFrom;

pub struct BodyfileDecoder {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<Bodyfile3Line>>
}

impl Filter<String, Bodyfile3Line> for BodyfileDecoder {
    fn with_receiver(reader: Receiver<String>) -> Self {
        let (tx, rx): (Sender<Bodyfile3Line>, Receiver<Bodyfile3Line>) = mpsc::channel();
        Self {
            worker: Some(std::thread::spawn(move || {
                Self::worker(reader, tx)
            })),
            rx: Some(rx)
        }
    }

    fn worker(reader: Receiver<String>, tx: Sender<Bodyfile3Line>) {
        loop {
            let mut line = match reader.recv() {
                Err(_) => {break;}
                Ok(l) => l
            };

            if line.starts_with('#') { continue; }
            Self::trim_newline(&mut line);

            let bf_line = match Bodyfile3Line::try_from(line.as_ref()) {
                Err(e) => {
                    log::warn!("bodyfile parser error: {}", e);
                    continue;
                }
                Ok(l) => l
            };

            if let Err(_) = tx.send(bf_line) {
                break;
            }
        }
    }

    fn get_receiver(&mut self) -> Receiver<Bodyfile3Line> {
        self.rx.take().unwrap()
    }
}

impl BodyfileDecoder {
    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
}

impl Joinable<()> for BodyfileDecoder {
    fn join(&mut self) -> std::thread::Result<()> {
        self.worker.take().unwrap().join()
    }
}