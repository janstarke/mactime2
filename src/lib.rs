use std::f64::consts::E;

use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::{LocalResult, NaiveDateTime};
use chrono_tz::Tz;

pub mod bodyfile_decoder;
pub mod bodyfile_reader;
pub mod bodyfile_sorter;
mod csv_output;
mod txt_output;
mod json_output;
pub mod filter;
pub use bodyfile_decoder::*;
pub use bodyfile_reader::*;
pub use bodyfile_sorter::*;
use clap::clap_derive::ValueEnum;
pub use filter::*;
use csv_output::*;
use json_output::JsonOutput;
use txt_output::*;
pub mod error;
pub use error::*;
mod elastic_output;
use elastic_output::*;

#[derive(ValueEnum, Clone)]
pub enum InputFormat {
    BODYFILE,

    #[cfg(feature="elastic")]
    JSON
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    CSV,
    TXT,
    JSON,

    #[cfg(feature="elastic")]
    ELASTIC
}

pub struct Mactime2Application {
    format: OutputFormat,
    bodyfile: Option<String>,
    src_zone: Tz,
    dst_zone: Tz,
    strict_mode: bool,


    #[cfg(feature="elastic")]
    host: String,

    #[cfg(feature="elastic")]
    port: u16,

    #[cfg(feature="elastic")]
    username: String,

    #[cfg(feature="elastic")]
    password: String,

    #[cfg(feature="elastic")]
    index_name: String,

    #[cfg(feature="elastic")]
    expect_existing: bool,

    #[cfg(feature="elastic")]
    omit_certificate_validation: bool
}

impl Mactime2Application {
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

    pub fn with_strict_mode(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    #[cfg(feature="elastic")]
    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    #[cfg(feature="elastic")]
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    #[cfg(feature="elastic")]
    pub fn with_username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    #[cfg(feature="elastic")]
    pub fn with_password(mut self, password: String) -> Self {
        self.password = password;
        self
    }

    #[cfg(feature="elastic")]
    pub fn with_index_name(mut self, index_name: String) -> Self {
        self.index_name = index_name;
        self
    }

    #[cfg(feature="elastic")]
    pub fn with_expect_existing(mut self, expect_existing: bool) -> Self {
        self.expect_existing = expect_existing;
        self
    }

    #[cfg(feature="elastic")]
    pub fn with_omit_certificate_validation(mut self, omit_certificate_validation: bool) -> Self {
        self.omit_certificate_validation = omit_certificate_validation;
        self
    }


    pub fn run(&self) -> Result<()> {
        let options = RunOptions {
            strict_mode: self.strict_mode,
        };

        let mut reader = BodyfileReader::from(&self.bodyfile)?;
        let mut decoder = BodyfileDecoder::with_receiver(reader.get_receiver(), options);
        let mut sorter = BodyfileSorter::default().with_receiver(decoder.get_receiver(), options);

        sorter = sorter.with_output(match self.format {
            OutputFormat::CSV => Box::new(CsvOutput::new(self.src_zone, self.dst_zone)),
            OutputFormat::TXT => Box::new(TxtOutput::new(self.src_zone, self.dst_zone)),
            OutputFormat::JSON => Box::new(JsonOutput::new(self.src_zone, self.dst_zone)),

            #[cfg(feature="elastic")]
            OutputFormat::ELASTIC => Box::new(ElasticOutput::new(
                self.host.clone(),
                self.port,
                self.username.clone(),
                self.password.clone(),
                self.index_name.clone(),
                self.expect_existing,
                self.omit_certificate_validation
            )),
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

impl Default for Mactime2Application {
    fn default() -> Self {
        Self {
            format: OutputFormat::CSV,
            bodyfile: None,
            src_zone: Tz::UTC,
            dst_zone: Tz::UTC,
            strict_mode: false,

            #[cfg(feature="elastic")]
            host: "localhost".to_string(),

            #[cfg(feature="elastic")]
            port: 9200,
            
            #[cfg(feature="elastic")]
            username: "elastic".to_string(),
            
            #[cfg(feature="elastic")]
            password: "elastic".to_string(),
            
            #[cfg(feature="elastic")]
            index_name: "elastic".to_string(),
            
            #[cfg(feature="elastic")]
            expect_existing: false,
            
            #[cfg(feature="elastic")]
            omit_certificate_validation: false,
        }
    }
}