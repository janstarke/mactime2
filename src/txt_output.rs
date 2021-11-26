use chrono_tz::Tz;
use std::cell::RefCell;
use crate::{Mactime2Application, Mactime2Writer};
use crate::bodyfile_sorter::*;

pub struct TxtOutput {
    src_zone: Tz, dst_zone: Tz,
    last_ts: (RefCell<i64>, RefCell<String>),
    empty_ts: RefCell<String>
}

impl TxtOutput {
    pub fn new(src_zone: Tz, dst_zone: Tz) -> Self {
        Self {
            src_zone, dst_zone,
            last_ts: (RefCell::new(-1), RefCell::new("".to_owned())),
            empty_ts: RefCell::new("                         ".to_owned())
        }
    }
}

impl Mactime2Writer for TxtOutput {
    fn fmt(&self, timestamp: &i64, entry: &ListEntry) -> String {
        let ts = if *timestamp != *self.last_ts.0.borrow() {
            *self.last_ts.1.borrow_mut() = Mactime2Application::format_date(*timestamp, &self.src_zone, &self.dst_zone);
            *self.last_ts.0.borrow_mut() = *timestamp;
            self.last_ts.1.borrow()
        } else {
            self.empty_ts.borrow()
        };
        format!(
            "{} {:>8} {} {:<12} {:<7} {:<7} {} {}",
            ts,
            entry.line.get_size(),
            entry.flags,
            entry.line.get_mode(),
            entry.line.get_uid(),
            entry.line.get_gid(),
            entry.line.get_inode(),
            entry.line.get_name()
        )
    }
}
