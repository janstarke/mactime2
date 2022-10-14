use std::{sync::mpsc::Receiver, thread::{spawn, JoinHandle}};
use anyhow::Result;

use elastic4forensics::{Index, IndexBuilder, WithHost};
use elasticsearch::auth::Credentials;
use serde_json::Value;

use crate::{Mactime2Writer, RunOptions, Joinable};

pub struct ElasticOutput {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    index_name: String,
    expect_existing: bool,
    omit_certificate_validation: bool,
    receiver: Option<Receiver<Value>>,
    options: RunOptions,
    worker: Option<JoinHandle<()>>
}

impl ElasticOutput {
    pub fn new(host: String, port: u16, username: String, password: String, index_name: String, expect_existing: bool, omit_certificate_validation: bool, receiver: Receiver<Value>, options: RunOptions) -> Self {
        Self {
            host,
            port,
            username: Some(username),
            password: Some(password),
            index_name,
            expect_existing,
            omit_certificate_validation,
            receiver: Some(receiver),
            options,
            worker: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let receiver = self.receiver.take().expect("no receiver provided; please call with_receiver()");

        let index = self.create_client()?;
        self.worker = Some(
            std::thread::spawn(move || Self::worker(receiver, index)));
        Ok(())
    }

    fn create_client(&mut self) -> Result<Index> {
        IndexBuilder::with_name(self.index_name.clone())
            .with_host(self.host.clone())
            .with_port(self.port)
            .with_credentials(Credentials::Basic(self.username.take().unwrap(), self.password.take().unwrap()))
            .build()
    }

    fn worker(decoder: Receiver<Value>, index: Index) -> () {
    }
}

impl Mactime2Writer for ElasticOutput {
    fn fmt(&self, timestamp: &i64, entry: &crate::ListEntry) -> String {
        todo!()
    }
}

impl Joinable<()> for ElasticOutput {
    fn join(&mut self) -> std::thread::Result<()> {
        match self.worker.take() {
            Some(w) => w.join(),
            None => Ok(()),
        }
    }
}