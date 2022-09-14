use chrono_tz::Tz;
use crate::{Mactime2Application, Mactime2Writer};
use crate::bodyfile_sorter::*;

pub struct JsonOutput {
    src_zone: Tz, dst_zone: Tz
}

impl JsonOutput {
    pub fn new(src_zone: Tz, dst_zone: Tz) -> Self {
        Self {
            src_zone, dst_zone
        }
    }
}

impl Mactime2Writer for JsonOutput {
    fn fmt(&self, timestamp: &i64, entry: &ListEntry) -> String {
        let timestamp = Mactime2Application::format_date(*timestamp, &self.src_zone, &self.dst_zone);
        format!(
            concat!("{{", 
                "\"ts\": \"{}\", ",
                "\"size\": {}, ",
                "\"flags\": \"{}\", ",
                "\"mode\": \"{}\", ",
                "\"uid\": {}, ",
                "\"gid\": {}, ",
                "\"inode\": \"{}\", ",
                "\"name\": \"{}\"",
            "}}"),
            timestamp,
            entry.line.get_size(),
            entry.flags,
            entry.line.get_mode(),
            entry.line.get_uid(),
            entry.line.get_gid(),
            entry.line.get_inode(),
            entry.line.get_name().replace('\\', "\\\\").replace('\"', "\\\"")
        )
    }
}
