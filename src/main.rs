use anyhow::Result;
use clap::{App, Arg};
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use chrono::{NaiveDateTime, LocalResult};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use chrono_tz::{TZ_VARIANTS, Tz};
use chrono::offset::TimeZone;

mod filter;
mod bodyfile_reader;
mod bodyfile_decoder;
mod bodyfile_sorter;
use filter::*;
use bodyfile_reader::*;
use bodyfile_decoder::*;
use bodyfile_sorter::*;

fn main() -> Result<()> {
    let _ = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto);

    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("BODYFILE")
                .short("b")
                .help("path to bodyfile of '-' for stdin")
                .required(false)
                .multiple(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("CSV_FORMAT")
                .short("d")
                .help("output as CSV instead of TXT")
        ).arg(
            Arg::with_name("SRC_ZONE")
                .short("f")
                .takes_value(true)
                .help("name of offset of source timezone (or 'list' to display all possible values")
        ).arg(
            Arg::with_name("DST_ZONE")
                .short("t")
                .takes_value(true)
                .help("name of offset of destination timezone (or 'list' to display all possible values")
        );

    let matches = app.get_matches();

    let src_zone: Tz = match matches.value_of("SRC_ZONE") {
        Some("list") => { display_zones(); return Ok(()); },
        Some(tz) => tz.parse().unwrap(),
        None => Tz::UTC
    };
    let dst_zone: Tz = match matches.value_of("DST_ZONE") {
        Some("list") => { display_zones(); return Ok(()); },
        Some(tz) => tz.parse().unwrap(),
        None => Tz::UTC
    };

    let mut reader = BodyfileReader::from(matches.value_of("BODYFILE"))?;
    let mut decoder = BodyfileDecoder::with_receiver(reader.get_receiver());
    let mut sorter = BodyfileSorter::with_receiver(decoder.get_receiver());

    let _ = reader.join();
    let _ = decoder.join();
    match sorter.join() {
        Ok(entries) => {
            if matches.is_present("CSV_FORMAT") {
                display_csv(entries, &src_zone, &dst_zone)
            } else {
                display_txt(entries, &src_zone, &dst_zone)
            }
        }
        Err(why) => {
            log::error!("{:?}", why);
        }
    }
    Ok(())
}

fn display_zones() {
    for v in TZ_VARIANTS.iter() {
        println!("{}", v);
    }
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
        let timestamp = format_date(*ts, src_zone, dst_zone);
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
        let mut timestamp = format_date(*ts, src_zone, dst_zone);
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
