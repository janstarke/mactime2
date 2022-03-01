use rand;
use libmactime2::*;
use std::sync::mpsc::{self, Sender, Receiver};
use std::cell::RefCell;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_sorted() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let options = RunOptions {
        strict_mode: false,
    };

    let mut decoder = BodyfileDecoder::with_receiver(rx, options);
    let mut sorter = BodyfileSorter::new()
        .with_receiver(decoder.get_receiver(), options)
        .with_output(Box::new(EventCatcher::new()));

    sorter.run();
    for _day in 0..364 {
        for _hour in 0..23 {
            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_mtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts())
                .with_mtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts())
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_mtime(random_ts())
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_mtime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts())
                .with_mtime(random_ts())
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts())
                .with_mtime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts())
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_mtime(random_ts())
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = bodyfile::Bodyfile3Line::new()
                .with_atime(random_ts())
                .with_mtime(random_ts())
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();
        }
    }
    drop(tx);

    decoder.join().unwrap();
    sorter.join().unwrap();
}

fn random_ts() -> i64 {
    rand::random::<u32>() as i64
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
    fn fmt(&self, timestamp: &i64, _entry: &ListEntry) -> String {
        assert_le!(*self.last_timestamp.borrow(), *timestamp);

        *self.last_timestamp.borrow_mut() = *timestamp;
        "".to_owned()
    }
}