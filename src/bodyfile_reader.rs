use anyhow::Result;
use std::io::BufReader;
use std::fs::File;

pub struct BodyfileReader {
    input: BodyfileSource,
}

enum BodyfileSource {
    Stdin,
    File(BufReader<File>),
}

impl BodyfileReader {
    pub fn from(filename: Option<&str>) -> Result<Self> {

        let input = match filename {
            None => BodyfileSource::Stdin,
            Some(filename) =>  {
                if filename == "-" { BodyfileSource::Stdin }
                else {BodyfileSource::File(BufReader::new(File::open(filename)?))
                }
            }
        };

        Ok(Self {
            input
        })
    }
}