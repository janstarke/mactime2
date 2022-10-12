use std::{thread::JoinHandle, sync::mpsc::Receiver};

use serde_json::Value;

use crate::{stream::{StreamWorker, StreamReader}, Joinable};


pub struct ElasticReader {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<Value>>,
}

impl StreamWorker<Value> for ElasticReader {
    fn worker<R: std::io::Read + Send>(input: R, tx: std::sync::mpsc::Sender<Value>) where String: Send {
        let value: Value = match serde_json::from_reader(input) {
            Ok(v) => v,
            Err(why) => {
                eprintln!("serde Error {:?}", why);
                return;
            }
        };

        match value {
            Value::Array(a) => {
                for item in a {
                    if tx.send(item).is_err() {
                        return;
                    }
                }
            }
            Value::Object(_) => {
                let _ = tx.send(value);
            },
            _ => eprintln!("unexpected value read: {:?}", value)
        }
    }
}

impl StreamReader<Value> for ElasticReader {
    fn new(worker: JoinHandle<()>, rx: Receiver<Value>) -> Self {
        Self {
            worker: Some(worker),
            rx: Some(rx)
        }
    }

    fn get_receiver(&mut self) -> Receiver<Value> {
        self.rx.take().unwrap()
    }
}

impl Joinable<()> for ElasticReader {
    fn join(&mut self) -> std::thread::Result<()> {
        self.worker.take().unwrap().join()
    }
}