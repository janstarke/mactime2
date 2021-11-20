use anyhow::Result;
use clap::{App, Arg};
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

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
        LevelFilter::Warn,
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
    let _ = sorter.join();
    Ok(())
}
