use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::{LocalResult, NaiveDateTime};
use chrono_tz::Tz;

pub mod bodyfile;
pub mod error;
pub mod filter;
mod output;
//use derive_builder::Builder;
use elastic::{BodyfileConverter, ElasticReader};
pub use error::*;
use serde_json::Value;
use es4forensics::objects::PosixFile;
mod elastic;
mod stream;

pub use crate::bodyfile::*;
use crate::stream::*;
use clap::clap_derive::ValueEnum;
pub use filter::*;
use output::*;
mod cli;
pub use cli::Cli;

#[derive(ValueEnum, Clone)]
pub enum InputFormat {
    BODYFILE,

    #[cfg(feature = "elastic")]
    JSON,
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    CSV,
    TXT,
    JSON,

    #[cfg(feature = "elastic")]
    ELASTIC,
}

//#[derive(Builder)]
pub struct Mactime2Application {
    format: OutputFormat,
    bodyfile: Option<String>,
    src_zone: Tz,
    dst_zone: Tz,
    strict_mode: bool,

    #[cfg(feature = "elastic")]
    input_format: InputFormat,

    #[cfg(feature = "elastic")]
    host: String,

    #[cfg(feature = "elastic")]
    port: u16,

    #[cfg(feature = "elastic")]
    username: String,

    #[cfg(feature = "elastic")]
    password: String,

    #[cfg(feature = "elastic")]
    index_name: String,

    #[cfg(feature = "elastic")]
    expect_existing: bool,

    #[cfg(feature = "elastic")]
    omit_certificate_validation: bool,
}

impl Mactime2Application {

    #[cfg(feature = "elastic")]
    fn create_value_provider(&self) -> Result<Box<dyn Provider<PosixFile, ()>>> {

        let options = RunOptions {
            strict_mode: self.strict_mode,
        };

        match self.input_format {
            InputFormat::BODYFILE => {
                let mut reader = <BodyfileReader as StreamReader<String, ()>>::from(&self.bodyfile)?;
                let mut decoder = BodyfileDecoder::with_receiver(reader.get_receiver(), options);
                let filter = BodyfileConverter::with_receiver(decoder.get_receiver(), options);
                Ok(Box::new(filter))
            }
            InputFormat::JSON => todo!()
        }
    }

    pub fn run(&self) -> Result<()> {
        let options = RunOptions {
            strict_mode: self.strict_mode,
        };

        #[cfg(feature = "elastic")]
        if matches!(self.format, OutputFormat::ELASTIC) {
            let mut reader = self.create_value_provider()?;
            let mut writer = ElasticOutput::new(
                self.host.clone(),
                self.port,
                self.username.clone(),
                self.password.clone(),
                self.index_name.clone(),
                self.expect_existing,
                self.omit_certificate_validation,
                reader.get_receiver(),
                options,
            );
            writer.run()?;
            let _ = reader.join();
            let _ = writer.join();
            return Ok(())
        }

        let mut reader = <BodyfileReader as StreamReader<String, ()>>::from(&self.bodyfile)?;
        let mut decoder = BodyfileDecoder::with_receiver(reader.get_receiver(), options);
        let mut sorter = BodyfileSorter::default().with_receiver(decoder.get_receiver(), options);

        sorter = sorter.with_output(match self.format {
            OutputFormat::CSV => Box::new(CsvOutput::new(self.src_zone, self.dst_zone)),
            OutputFormat::TXT => Box::new(TxtOutput::new(self.src_zone, self.dst_zone)),
            OutputFormat::JSON => Box::new(JsonOutput::new(self.src_zone, self.dst_zone)),
            _ => panic!("invalid execution path"),
        });
        sorter.run();

        let _ = reader.join();
        let _ = decoder.join();
        sorter.join().unwrap()?;
        Ok(())
    }

    pub fn format_date(unix_ts: i64, src_zone: &Tz, dst_zone: &Tz) -> String {
        if unix_ts >= 0 {
            let src_timestamp =
                match src_zone.from_local_datetime(&NaiveDateTime::from_timestamp(unix_ts, 0)) {
                    LocalResult::None => {
                        return "INVALID DATETIME".to_owned();
                    }
                    LocalResult::Single(t) => t,
                    LocalResult::Ambiguous(t1, _t2) => t1,
                };
            let dst_timestamp = src_timestamp.with_timezone(dst_zone);
            dst_timestamp.to_rfc3339()
        } else {
            "0000-00-00T00:00:00+00:00".to_owned()
        }
    }
}

impl From<Cli> for Mactime2Application {
    fn from(cli: Cli) -> Self {

        let format = match cli.output_format {
            Some(f) => f,
            None => {
                if cli.csv_format { OutputFormat::CSV }
                else if cli.json_format { OutputFormat::JSON }
                else { OutputFormat::TXT }
            }
        };

        Self {
            format,
            bodyfile: Some(cli.bodyfile),
            src_zone: cli.src_zone.map(|tz| tz.parse().unwrap()).unwrap_or(Tz::UTC),
            dst_zone: cli.dst_zone.map(|tz| tz.parse().unwrap()).unwrap_or(Tz::UTC),
            strict_mode: cli.strict_mode,

            #[cfg(feature = "elastic")]
            input_format: cli.input_format,

            #[cfg(feature = "elastic")]
            host: cli.host,

            #[cfg(feature = "elastic")]
            port: cli.port,

            #[cfg(feature = "elastic")]
            username: cli.username,

            #[cfg(feature = "elastic")]
            password: cli.password.unwrap_or_else(|| "elastic".to_string()),

            #[cfg(feature = "elastic")]
            index_name: cli.index_name.unwrap_or_else(|| "elastic".to_string()),

            #[cfg(feature = "elastic")]
            expect_existing: cli.expect_existing,

            #[cfg(feature = "elastic")]
            omit_certificate_validation: cli.omit_certificate_validation
        }
    }
}

impl Default for Mactime2Application {
    fn default() -> Self {
        Self {
            format: OutputFormat::CSV,
            bodyfile: None,
            src_zone: Tz::UTC,
            dst_zone: Tz::UTC,
            strict_mode: false,

            #[cfg(feature = "elastic")]
            input_format: InputFormat::BODYFILE,

            #[cfg(feature = "elastic")]
            host: "localhost".to_string(),

            #[cfg(feature = "elastic")]
            port: 9200,

            #[cfg(feature = "elastic")]
            username: "elastic".to_string(),

            #[cfg(feature = "elastic")]
            password: "elastic".to_string(),

            #[cfg(feature = "elastic")]
            index_name: "elastic".to_string(),

            #[cfg(feature = "elastic")]
            expect_existing: false,

            #[cfg(feature = "elastic")]
            omit_certificate_validation: false,
        }
    }
}
