use anyhow::Result;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use crate::Joinable;

#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;

pub struct BodyfileReader {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<String>>
}

enum BodyfileSource {
    Stdin,
    File(Box<dyn BufRead + Send>),
}

fn worker(mut input: BodyfileSource, tx: Sender<String>) {
    let mut line_ctr = 1;
    loop {
        let mut line = String::new();
        let size = match &mut input {
            BodyfileSource::Stdin => io::stdin().read_line(&mut line),
            BodyfileSource::File(f) => f.read_line(&mut line)
        };

        match size {
            Err(why) => {
                eprintln!("IO Error in line {}: {:?}", line_ctr, why);
                break;
            }
            Ok(s) => {
                if s == 0 { break; }

                if let Err(_) = tx.send(line) {
                    break;
                }
            }
        }
        line_ctr += 1;
    }
}

impl BodyfileReader {
    pub fn from(filename: &Option<String>) -> Result<Self> {
        let input = match filename {
            None => BodyfileSource::Stdin,
            Some(filename) =>  {
                if filename == "-" { BodyfileSource::Stdin }
                else {
                    let file = File::open(filename)?;

                    #[cfg(not(feature = "gzip"))]
                    let reader: Box<dyn BufRead> = Box::new(BufReader::new(file));

                    #[cfg(feature = "gzip")]
                    let reader = Self::open_gzip(filename, file);

                    BodyfileSource::File(reader)
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

    #[cfg(feature = "gzip")]
    fn open_gzip(filename: &str, file: File) -> Box<dyn BufRead + Send> {
        if filename.ends_with(".gz") {
            Box::new(BufReader::new(GzDecoder::new(file)))
        } else {
            Box::new(BufReader::new(file))
        }
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