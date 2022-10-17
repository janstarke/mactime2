use clap::Parser;
use crate::{InputFormat, OutputFormat};


#[cfg(feature = "gzip")]
const BODYFILE_HELP: &str = "path to bodyfile of '-' for stdin (files ending with .gz will be treated as being gzipped)";
#[cfg(not(feature = "gzip"))]
const BODYFILE_HELP: &str = "path to bodyfile of '-' for stdin";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short('b'), default_value="-", help=BODYFILE_HELP, display_order(100))]
    pub(crate) bodyfile: String,

    /// input format
    #[clap(short('I'), long("input-format"), value_enum, display_order(550), default_value_t=InputFormat::BODYFILE)]
    #[cfg(feature="elastic")]
    pub(crate) input_format: InputFormat,

    /// output format, if not specified, default value is 'txt'
    #[clap(short('F'), long("format"), value_enum, display_order(600))]
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

    /// server name or IP address of elasticsearch server
    #[clap(short('H'), long("host"), display_order=600, default_value="localhost")]
    #[cfg(feature="elastic")]
    pub(crate) host: String,

    /// API port number of elasticsearch server
    #[clap(short('P'), long("port"), display_order=610, default_value_t=9200)]
    #[cfg(feature="elastic")]
    pub(crate) port: u16,

    /// name of the elasticsearch index
    #[clap(long("index"), display_order=615)]
    #[cfg(feature="elastic")]
    pub(crate) index_name: Option<String>,

    /// username for elasticsearch server
    #[clap(short('U'), long("username"), display_order=620, default_value=Some("elastic"))]
    #[cfg(feature="elastic")]
    pub(crate) username: String,

    /// password for authenticating at elasticsearch
    #[clap(short('W'), long("password"), display_order=620)]
    #[cfg(feature="elastic")]
    pub(crate) password: Option<String>,

    /// If this flag is set, a new index is created if it does not exist already, and new values
    /// will be inserted into the index, no matter what. If the flag is not set, mactime2 will
    /// check if the index exists and will abort if there is already such an index. Otherwise, the index
    /// will be newly created.
    #[clap(short('X'), long("expect-existing"), display_order=630, default_value_t=false)]
    #[cfg(feature="elastic")]
    pub(crate) expect_existing: bool,

    /// omit certificate validation
    #[clap(short('k'), long("insecure"), display_order=640, default_value_t=false)]
    #[cfg(feature="elastic")]
    pub(crate) omit_certificate_validation: bool,

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