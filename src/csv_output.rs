use chrono_tz::Tz;
use crate::{Mactime2Application, Mactime2Writer};
use crate::bodyfile_sorter::*;

pub struct CsvOutput {
    src_zone: Tz, dst_zone: Tz
}

impl CsvOutput {
    pub fn new(src_zone: Tz, dst_zone: Tz) -> Self {
        Self {
            src_zone, dst_zone
        }
    }
}

impl Mactime2Writer for CsvOutput {
    fn write(&self, timestamp: &i64, entry: &ListEntry) {
        let timestamp = Mactime2Application::format_date(*timestamp, &self.src_zone, &self.dst_zone);
        println!(
            "{},{},{},{},{},{},{},\"{}\"",
            timestamp,
            entry.line.get_size(),
            entry.flags,
            entry.line.get_mode(),
            entry.line.get_uid(),
            entry.line.get_gid(),
            entry.line.get_inode(),
            entry.line.get_name()
        );
    }
}
