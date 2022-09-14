use anyhow::Result;
use clap::Parser;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};
use libmactime2::{Mactime2Application, OutputFormat};
use chrono_tz::TZ_VARIANTS;

#[cfg(feature = "gzip")]
const BODYFILE_HELP: &str = "path to bodyfile of '-' for stdin (files ending with .gz will be treated as being gzipped)";
#[cfg(not(feature = "gzip"))]
const BODYFILE_HELP: &str = "path to bodyfile of '-' for stdin";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(default_value="-", help=BODYFILE_HELP, display_order(100))]
    pub(crate) bodyfile: String,

    /// output format, if not specified, default value is 'txt'
    #[clap(short('F'), long("format"), arg_enum, display_order(600))]
    pub(crate) output_format: Option<OutputFormat>,

    /// output as CSV instead of TXT. This is a conveniance option, which is identical to `--format=csv`
    /// and will be removed in a future release. If you specified `--format` and `-d`, the latter will be ignored.
    #[clap(short('d'), display_order(700))]
    pub(crate) csv_format: bool,

    /// output as JSON instead of TXT. This is a conveniance option, which is identical to `--format=json`
    /// and will be removed in a future release. If you specified `--format` and `-j`, the latter will be ignored.
    #[clap(short('j'), display_order(800))]
    pub(crate) json_format: bool,

    /// name of offset of source timezone (or 'list' to display all possible values
    #[clap(short('f'), long("from-timezone"), display_order(300))]
    pub(crate) src_zone: Option<String>,

    /// name of offset of destination timezone (or 'list' to display all possible values
    #[clap(short('t'), long("to-timezone"), display_order(400))]
    pub(crate) dst_zone: Option<String>,

    /// strict mode: do not only warn, but abort if an error occurs
    #[clap(long("strict"), display_order(500))]
    pub(crate) strict_mode: bool,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let _ = TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto);
    
    let mut app = Mactime2Application::default();
    app = app.with_bodyfile(cli.bodyfile);

    if cli.strict_mode {
        app = app.with_strict_mode();
    }

    match cli.src_zone {
        Some(tz) => { 
            if tz == "list" {
                display_zones(); return Ok(()); 
            } else {
                app = app.with_src_zone(tz.parse().unwrap());
            }
        }
        None => (),
    };

    match cli.dst_zone {
        Some(tz) => { 
            if tz == "list" {
                display_zones(); return Ok(()); 
            } else {
                app = app.with_dst_zone(tz.parse().unwrap());
            }
        }
        None => (),
    };

    app = app.with_format(
        match cli.output_format {
            Some(f) => f,
            None => {
                if cli.csv_format { OutputFormat::CSV }
                else if cli.json_format { OutputFormat::JSON }
                else { OutputFormat::TXT }
            }
        }
    );

    app.run()
}

fn display_zones() {
    for v in TZ_VARIANTS.iter() {
        println!("{}", v);
    }
}

