use apple_text::TextFailure;
use std::env;
use std::io;
use std::path::PathBuf;

use thiserror::Error;
use tool_text::{ToolName, ToolOutput};

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{}: {error}", .path.display())]
    Io { path: PathBuf, error: io::Error },
    #[error("missing or invalid environment variable {0}")]
    Environment(VariableName),
    #[error("{tool} failed:\n{detail}")]
    Tool { tool: ToolName, detail: ToolOutput },
    #[error("unexpected App Store Connect response: {0}")]
    Response(serde_json::Error),
    #[error("signing: {0}")]
    Signing(ToolOutput),
    #[error("{0}")]
    Text(#[from] TextFailure),
}

#[derive(newtype::Newtype, newtype::Display, Clone, Copy, Debug, PartialEq, Eq)]
pub struct VariableName(&'static str);

impl VariableName {
    pub fn folder(self) -> Result<PathBuf, Failure> {
        env::var_os(self.0)
            .map(PathBuf::from)
            .ok_or(Failure::Environment(self))
    }
}
