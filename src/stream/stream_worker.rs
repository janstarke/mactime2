use std::{io::Read, sync::mpsc::Sender};

pub(crate) trait StreamWorker<T> {
    fn worker<R: Read + Send>(input: R, tx: Sender<T>) where T: Send;
}