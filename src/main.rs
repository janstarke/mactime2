use anyhow::Result;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod bodyfile_reader;
use bodyfile_reader::*;

fn main() -> Result<()> {
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
    let reader = BodyfileReader::from(matches.value_of("BODYFILE"))?;
    Ok(())
}
