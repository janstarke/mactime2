use crate::Mactime2Writer;

pub(crate) struct ElasticOutput {
    
}

impl ElasticOutput {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Mactime2Writer for ElasticOutput {
    fn fmt(&self, timestamp: &i64, entry: &crate::ListEntry) -> String {
        todo!()
    }
}