use anyhow::Result;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use crate::Joinable;

pub struct BodyfileReader {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<String>>
}

enum BodyfileSource {
    Stdin,
    File(BufReader<File>),
}

fn worker(mut input: BodyfileSource, tx: Sender<String>) {
    loop {
        let mut line = String::new();
        let size = match &mut input {
            BodyfileSource::Stdin => io::stdin().read_line(&mut line),
            BodyfileSource::File(f) => f.read_line(&mut line)
        };

        match size {
            Err(_) => {break;}
            Ok(s) => {
                if s == 0 { break; }

                if let Err(_) = tx.send(line) {
                    break;
                }}
        }
    }
}

impl BodyfileReader {
    pub fn from(filename: &Option<String>) -> Result<Self> {
        let input = match filename {
            None => BodyfileSource::Stdin,
            Some(filename) =>  {
                if filename == "-" { BodyfileSource::Stdin }
                else {BodyfileSource::File(BufReader::new(File::open(filename)?))
                }
            }
        };
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let worker = thread::spawn(move || {worker(input, tx);});

        Ok(Self {
            worker: Some(worker),
            rx: Some(rx)
        })
    }

    pub fn get_receiver(&mut self) -> Receiver<String> {
        self.rx.take().unwrap()
    }
}

impl Joinable<()> for BodyfileReader {
    fn join(&mut self) -> std::thread::Result<()> {
        self.worker.take().unwrap().join()
    }
}