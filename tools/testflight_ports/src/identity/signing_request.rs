use std::fs;
use std::path::PathBuf;

use crate::failure::Failure;
use crate::staging::Staging;
use crate::tool::Tool;

pub struct SigningRequest {
    key: PathBuf,
    content: String,
}

impl SigningRequest {
    const KEY: &str = "distribution.key";
    const REQUEST: &str = "distribution.csr";
    const SUBJECT: &str = "/CN=Basho Walk Distribution";

    pub fn generate(staging: &Staging) -> Result<SigningRequest, Failure> {
        let key = staging.path(Self::KEY);
        let request = staging.path(Self::REQUEST);
        Tool::named("openssl")
            .args([
                "req",
                "-new",
                "-newkey",
                "rsa:2048",
                "-nodes",
                "-subj",
                Self::SUBJECT,
            ])
            .arg("-keyout")
            .arg(&key)
            .arg("-out")
            .arg(&request)
            .output()?;
        let content = fs::read_to_string(&request).map_err(Staging::io_failure(&request))?;
        Ok(SigningRequest { key, content })
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn key(&self) -> &PathBuf {
        &self.key
    }

    pub fn discard(self) -> Result<(), Failure> {
        fs::remove_file(&self.key).map_err(Staging::io_failure(&self.key))
    }
}
