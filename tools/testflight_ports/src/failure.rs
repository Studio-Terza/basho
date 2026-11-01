use apple_text::TextFailure;
use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{}: {error}", .path.display())]
    Io { path: PathBuf, error: io::Error },
    #[error("missing or invalid environment variable {0}")]
    Environment(&'static str),
    #[error("{tool} failed:\n{detail}")]
    Tool { tool: String, detail: String },
    #[error("App Store Connect: {0}")]
    Connect(String),
    #[error("unexpected App Store Connect response: {0}")]
    Response(serde_json::Error),
    #[error("signing: {0}")]
    Signing(String),
    #[error("{0}")]
    Text(#[from] TextFailure),
}
