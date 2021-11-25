use rand;
use libmactime2::*;
use std::sync::mpsc::{self, Sender, Receiver};
use std::cell::RefCell;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_sorted() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let mut decoder = BodyfileDecoder::with_receiver(rx);
    let mut sorter = BodyfileSorter::new()
        .with_receiver(decoder.get_receiver())
        .with_output(Box::new(EventCatcher::new()));

    sorter.run();
    for _day in 0..364 {
        for _hour in 0..23 {
            let ts = rand::random::<u32>() as i64;
            let bf = bodyfile::Bodyfile3Line::new().with_crtime(ts);
            tx.send(bf.to_string()).unwrap();
        }
    }
    drop(tx);

    decoder.join().unwrap();
    sorter.join().unwrap();
}

struct EventCatcher {
    last_timestamp: RefCell<i64>,
}

impl EventCatcher {
    pub fn new () -> Self {
        Self {
            last_timestamp: RefCell::new(-1)
        }
    }
}

impl Mactime2Writer for EventCatcher {
    fn write(&self, timestamp: &i64, _entry: &ListEntry) {
        assert_le!(*self.last_timestamp.borrow(), *timestamp);

        *self.last_timestamp.borrow_mut() = *timestamp;
    }
}