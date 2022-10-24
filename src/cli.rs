
use clap::{Parser};
use crate::{OutputFormat};


#[cfg(feature = "gzip")]
const BODYFILE_HELP: &str = "path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped)";
#[cfg(not(feature = "gzip"))]
const BODYFILE_HELP: &str = "path to input file or '-' for stdin";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short('b'), default_value="-", help=BODYFILE_HELP, display_order(100))]
    pub(crate) input_file: String,

    /// output format, if not specified, default value is 'txt'
    #[clap(short('F'), long("format"), value_enum, display_order(600))]
    pub(crate) output_format: Option<OutputFormat>,

    /// output as CSV instead of TXT. This is a conveniance option, which is identical to `--format=csv`
    /// and will be removed in a future release. If you specified `--format` and `-d`, the latter will be ignored.
    #[clap(short('d'), display_order(610))]
    pub(crate) csv_format: bool,

    /// output as JSON instead of TXT. This is a conveniance option, which is identical to `--format=json`
    /// and will be removed in a future release. If you specified `--format` and `-j`, the latter will be ignored.
    #[clap(short('j'), display_order(620))]
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

impl Cli {
    pub fn verbose(&self) -> &clap_verbosity_flag::Verbosity {
        &self.verbose
    }

    pub fn src_zone(&self) -> &Option<String> {
        &self.src_zone
    }

    pub fn dst_zone(&self) -> &Option<String> {
        &self.dst_zone
    }
}