use crate::Joinable;
use bitflags::bitflags;
use bodyfile::Bodyfile3Line;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use chrono::{NaiveDate, NaiveDateTime};
use std::cmp::Ordering;

pub struct BodyfileSorter {
    worker: Option<JoinHandle<()>>,
}

bitflags! {
    struct MACBFlags: u8 {
        const NONE = 0b00000000;
        const M = 0b00000001;
        const A = 0b00000010;
        const C = 0b00000100;
        const B = 0b00001000;
    }
}

impl fmt::Display for MACBFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = if *self & Self::M == Self::M { 'm' } else { '.' };
        let a = if *self & Self::A == Self::A { 'a' } else { '.' };
        let c = if *self & Self::C == Self::C { 'c' } else { '.' };
        let b = if *self & Self::B == Self::B { 'b' } else { '.' };
        write!(f, "{}{}{}{}", m, a, c, b)
    }
}

struct ListEntry {
    flags: MACBFlags,
    line: Rc<Bodyfile3Line>,
}

impl Eq for ListEntry {}
impl PartialEq for ListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.line.get_name().eq(other.line.get_name())
    }
}
impl PartialOrd for ListEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.line.get_name().cmp(&other.line.get_name())
    }
}



fn insert_timestamp(entries: &mut BTreeMap<i64, BTreeSet<ListEntry>>, flag: MACBFlags, line: Rc<Bodyfile3Line>) {
    let timestamp = if flag.contains(MACBFlags::M) {
        line.get_mtime()
    } else if flag.contains(MACBFlags::A) {
        line.get_atime()
    } else if flag.contains(MACBFlags::C) {
        line.get_ctime()
    } else if flag.contains(MACBFlags::B) {
        line.get_crtime()
    } else {
        panic!("no flags set")
    };
    assert_ne!(timestamp, -1);

    match entries.get_mut(&timestamp) {
        None => {
            let mut entries_at_ts = BTreeSet::new();
            let entry = ListEntry {
                flags: flag,
                line: line,
            };
            entries_at_ts.insert(entry);
            entries.insert(timestamp, entries_at_ts);
        }

        Some(entries_at_ts) => {
            let entry = ListEntry {
                flags: flag,
                line: line,
            };
            entries_at_ts.insert(entry);
        }
    }
}

fn worker(decoder: Receiver<Bodyfile3Line>) {
    let mut entries: BTreeMap<i64, BTreeSet<ListEntry>> = BTreeMap::new();

    loop {
        let line = Rc::new(match decoder.recv() {
            Err(_) => {
                break;
            }
            Ok(l) => l,
        });

        // # we need *some* value in mactimes!
        if line.get_mtime() == 0 && line.get_atime() == 0 && line.get_ctime() == 0 && line.get_crtime() == 0 {
            continue;
        }

        let mut flags: [MACBFlags; 4] = [MACBFlags::NONE; 4];

        if line.get_mtime() != -1 {
            flags[0] |= MACBFlags::M;
        }
        if line.get_atime() != -1 {
            if line.get_mtime() == line.get_atime() {
                flags[0] |= MACBFlags::A;
            } else {
                flags[1] |= MACBFlags::A;
            }
        }
        if line.get_ctime() != -1 {
            if        line.get_mtime() == line.get_ctime() {
                flags[0] |= MACBFlags::C;
            } else if line.get_atime() == line.get_ctime() {
                flags[1] |= MACBFlags::C;
            } else  {
                flags[2] |= MACBFlags::C;
            }
        }
        if line.get_crtime() != -1 {
            if        line.get_mtime() == line.get_crtime() {
                flags[0] |= MACBFlags::B;
            } else if line.get_atime() == line.get_crtime() {
                flags[1] |= MACBFlags::B;
            } else if line.get_ctime() == line.get_crtime() {
                flags[2] |= MACBFlags::B;
            } else  {
                flags[3] |= MACBFlags::B;
            }
        }
        for flag in flags {
            if flag != MACBFlags::NONE {
                insert_timestamp(&mut entries, flag, Rc::clone(&line));
            }
        }
    }

    println!("Date,Size,Type,Mode,UID,GID,Meta,File Name");
    for (ts, entries_at_ts) in entries.iter() {
        let timestamp = NaiveDateTime::from_timestamp(*ts, 0);
        let timestamp = timestamp.format("%a %b %d %Y %T");
        for line in entries_at_ts {
            println!("{},{},{},{},0,0,{},\"{}\"",timestamp,
                line.line.get_size(),
                line.flags,
                line.line.get_mode(),
                line.line.get_inode(),
                line.line.get_name());
        }
    }
}

impl BodyfileSorter {
    pub fn with_receiver(decoder: Receiver<Bodyfile3Line>) -> Self {
        Self {
            worker: Some(std::thread::spawn(move || worker(decoder))),
        }
    }
}

impl Joinable for BodyfileSorter {
    fn join(&mut self) -> std::thread::Result<()> {
        self.worker.take().unwrap().join()
    }
}
