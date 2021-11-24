use chrono_tz::Tz;
use anyhow::Result;
use chrono::{NaiveDateTime, LocalResult};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use chrono::offset::TimeZone;

mod filter;
mod bodyfile_reader;
mod bodyfile_decoder;
mod bodyfile_sorter;
use filter::*;
use bodyfile_reader::*;
use bodyfile_decoder::*;
use bodyfile_sorter::*;

pub enum OutputFormat {
    CSV,
    TXT
}

pub struct Mactime2Application {
    format: OutputFormat,
    bodyfile: Option<String>,
    src_zone: Tz,
    dst_zone: Tz,
}

impl Mactime2Application {

    pub fn new() -> Self {
        Self {
            format: OutputFormat::CSV,
            bodyfile: None,
            src_zone: Tz::UTC,
            dst_zone: Tz::UTC,
        }
    }

    pub fn with_format(mut self, format: OutputFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_bodyfile(mut self, bodyfile: String) -> Self {
        self.bodyfile = Some(bodyfile);
        self
    }

    pub fn with_src_zone(mut self, src_zone: Tz) -> Self {
        self.src_zone = src_zone;
        self
    }

    pub fn with_dst_zone(mut self, dst_zone: Tz) -> Self {
        self.dst_zone = dst_zone;
        self
    }

    pub fn run(&self) -> Result<()> {

        let mut reader = BodyfileReader::from(&self.bodyfile)?;
        let mut decoder = BodyfileDecoder::with_receiver(reader.get_receiver());
        let mut sorter = BodyfileSorter::with_receiver(decoder.get_receiver());

        let _ = reader.join();
        let _ = decoder.join();
        
        match sorter.join() {
            Ok(entries) => {
                match self.format {
                    OutputFormat::CSV => {
                        Self::display_csv(entries,
                            &self.src_zone, &self.dst_zone);
                        }
                    OutputFormat::TXT => {
                        Self::display_txt(entries,
                            &self.src_zone, &self.dst_zone);
                        }
                }
            }
            Err(why) => {
                log::error!("{:?}", why);
            }
        }
        Ok(())
    }


fn format_date(unix_ts: i64, src_zone: &Tz, dst_zone: &Tz) -> String {
    let src_timestamp = match src_zone.from_local_datetime(&NaiveDateTime::from_timestamp(unix_ts, 0)) {
        LocalResult::None => { return "INVALID DATETIME".to_owned(); }
        LocalResult::Single(t) => t,
        LocalResult::Ambiguous(t1, _t2) => t1
    };
    let dst_timestamp = src_timestamp.with_timezone(dst_zone);
    dst_timestamp.to_rfc3339()
}

fn display_csv(entries: BTreeMap<i64, BTreeSet<ListEntry>>, src_zone: &Tz, dst_zone: &Tz) {
    println!("Date,Size,Type,Mode,UID,GID,Meta,File Name");
    for (ts, entries_at_ts) in entries.iter() {
        let timestamp = Self::format_date(*ts, src_zone, dst_zone);
        for line in entries_at_ts {
            println!(
                "{},{},{},{},{},{},{},\"{}\"",
                timestamp,
                line.line.get_size(),
                line.flags,
                line.line.get_mode(),
                line.line.get_uid(),
                line.line.get_gid(),
                line.line.get_inode(),
                line.line.get_name()
            );
        }
    }
}

fn display_txt(entries: BTreeMap<i64, BTreeSet<ListEntry>>, src_zone: &Tz, dst_zone: &Tz) {
    for (ts, entries_at_ts) in entries.iter() {
        let mut timestamp = Self::format_date(*ts, src_zone, dst_zone);
        for line in entries_at_ts {
            println!(
                "{} {:>8} {} {:<12} {:<7} {:<7} {} {}",
                timestamp,
                line.line.get_size(),
                line.flags,
                line.line.get_mode(),
                line.line.get_uid(),
                line.line.get_gid(),
                line.line.get_inode(),
                line.line.get_name()
            );
            timestamp = "                         ".to_owned();
        }
    }
}
}