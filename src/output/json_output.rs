use std::borrow::Borrow;
use std::convert::TryFrom;
use chrono_tz::Tz;
use es4forensics::objects::{PosixFile, ElasticObject};
use crate::{Mactime2Writer};
use crate::bodyfile::*;

pub struct JsonOutput {
    src_zone: Tz
}

impl JsonOutput {
    pub fn new(src_zone: Tz) -> Self {
        Self {
            src_zone
        }
    }
}

impl Mactime2Writer for JsonOutput {
    fn fmt(&self, _timestamp: &i64, entry: &ListEntry) -> String {
        let pf = PosixFile::try_from((entry.line.borrow(), &self.src_zone)).unwrap();
        let lines: Vec<String> = pf.documents().map(|v|serde_json::to_string(&v)).map(Result::unwrap).collect();
        
        if lines.is_empty() {
            log::warn!("file {} has no timestamp entries", entry.line.get_name());
            log::warn!("raw entry is {}", entry.line.to_string());
            String::new()
        } else {
            lines.join("\n")
        }
    }
}
