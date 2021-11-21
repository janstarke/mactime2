use anyhow::Result;
use clap::{App, Arg};
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use chrono::{NaiveDateTime};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
        );

    let matches = app.get_matches();
    let mut reader = BodyfileReader::from(matches.value_of("BODYFILE"))?;
    let mut decoder = BodyfileDecoder::with_receiver(reader.get_receiver());
    let mut sorter = BodyfileSorter::with_receiver(decoder.get_receiver());

    let _ = reader.join();
    let _ = decoder.join();
    match sorter.join() {
        Ok(entries) => display_csv(entries),
        Err(why) => {
            log::error!("{:?}", why);
        }
    }
    Ok(())
}

fn display_csv(entries: BTreeMap<i64, BTreeSet<ListEntry>>) {
    println!("Date,Size,Type,Mode,UID,GID,Meta,File Name");
    for (ts, entries_at_ts) in entries.iter() {
        let timestamp = NaiveDateTime::from_timestamp(*ts, 0);
        let timestamp = timestamp.format("%a %b %d %Y %T");
        for line in entries_at_ts {
            println!(
                "{},{},{},{},0,0,{},\"{}\"",
                timestamp,
                line.line.get_size(),
                line.flags,
                line.line.get_mode(),
                line.line.get_inode(),
                line.line.get_name()
            );
        }
    }
}
