use std::env;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::boundary::Tool;
use crate::connect::identifier::Identifier;
use crate::failure::Failure;
use crate::staging::Staging;

#[derive(Deserialize)]
pub struct Credentials {
    key_id: Identifier,
    issuer_id: Identifier,
}

impl Credentials {
    const FILE: &str = ".appstoreconnect/basho.json";

    pub fn load() -> Result<Credentials, Failure> {
        let path = env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or(Failure::Environment("HOME"))?
            .join(Self::FILE);
        let text = fs::read(&path).map_err(Staging::io_failure(&path))?;
        serde_json::from_slice(&text).map_err(Failure::Response)
    }

    pub fn appended_to(&self, altool: Tool) -> Tool {
        altool
            .args(["--apiKey", self.key_id.as_ref()])
            .args(["--apiIssuer", self.issuer_id.as_ref()])
    }
}
