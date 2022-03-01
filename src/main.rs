use anyhow::Result;
use clap::{App, Arg};
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use chrono_tz::TZ_VARIANTS;
use libmactime2::{Mactime2Application, OutputFormat};

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
        ).arg(
            Arg::with_name("STRICT_MODE")
                .long("strict")
                .takes_value(false)
                .help("strict mode: abort if an error occurs")
        );

    let matches = app.get_matches();
    let mut app = Mactime2Application::new();
    if let Some(bodyfile) = matches.value_of("BODYFILE") {
        app = app.with_bodyfile(bodyfile.to_owned());
    }

    if matches.is_present("STRICT_MODE") {
        app = app.with_strict_mode();
    }

    match matches.value_of("SRC_ZONE") {
        Some("list") => { display_zones(); return Ok(()); },
        Some(tz) => { app = app.with_src_zone(tz.parse().unwrap()); }
        None => (),
    };

    match matches.value_of("DST_ZONE") {
        Some("list") => { display_zones(); return Ok(()); },
        Some(tz) => { app = app.with_dst_zone(tz.parse().unwrap()); }
        None => (),
    };

    app = app.with_format(if matches.is_present("CSV_FORMAT") {
        OutputFormat::CSV
    } else {
        OutputFormat::TXT
    });

    app.run()
}

fn display_zones() {
    for v in TZ_VARIANTS.iter() {
        println!("{}", v);
    }
}

