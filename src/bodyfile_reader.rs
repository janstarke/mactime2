use anyhow::Result;
use std::io::{BufRead, BufReader, Read, stdin};
use std::fs::File;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use crate::Joinable;
use encoding_rs_io::DecodeReaderBytesBuilder;

#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;

pub struct BodyfileReader {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<String>>
}

enum BodyfileSource {
    Stdin,
    File(Box<dyn Read + Send>),
}

fn worker<R: Read + Send>(input: R, tx: Sender<String>) {
    let mut line_ctr = 1;

    let drb = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_8))
        .utf8_passthru(true)
        .build(input);
    let mut reader = BufReader::new(drb);

    loop {
        let mut line = String::new();
        let size = reader.read_line(&mut line);

        match size {
            Err(why) => {
                eprintln!("IO Error in line {}: {:?}", line_ctr, why);
                break;
            }
            Ok(s) => {
                if s == 0 { break; }

                if tx.send(line).is_err() {
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
                    let reader: Box<dyn BufRead> = Box::new(file);

                    #[cfg(feature = "gzip")]
                    let reader = Self::open_gzip(filename, file);

                    BodyfileSource::File(reader)
                }
            }
        };
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let worker = match input {
            BodyfileSource::Stdin => thread::spawn(move || {worker(stdin(), tx);}),
            BodyfileSource::File(f) => thread::spawn(move || {worker(f, tx);}),
        };

        Ok(Self {
            worker: Some(worker),
            rx: Some(rx)
        })
    }

    #[cfg(feature = "gzip")]
    fn open_gzip(filename: &str, file: File) -> Box<dyn Read + Send> {
        if filename.ends_with(".gz") {
            Box::new(GzDecoder::new(file))
        } else {
            Box::new(file)
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