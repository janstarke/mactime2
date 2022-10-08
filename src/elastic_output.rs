use crate::Mactime2Writer;

pub(crate) struct ElasticOutput {
    host: String,
    port: u16,
    username: String,
    password: String,
    index_name: String,
    expect_existing: bool,
    omit_certificate_validation: bool,
}

impl ElasticOutput {
    pub fn new(host: String, port: u16, username: String, password: String, index_name: String, expect_existing: bool, omit_certificate_validation: bool) -> Self {
        Self {
            host,
            port,
            username,
            password,
            index_name,
            expect_existing,
            omit_certificate_validation
        }
    }
}

impl Mactime2Writer for ElasticOutput {
    fn fmt(&self, timestamp: &i64, entry: &crate::ListEntry) -> String {
        todo!()
    }
}