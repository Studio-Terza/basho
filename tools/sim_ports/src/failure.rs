use std::io;
use std::path::PathBuf;
use std::time::Duration;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{}: {error}", .path.display())]
    Io { path: PathBuf, error: io::Error },
    #[error("{tool} failed:\n{detail}")]
    Tool { tool: String, detail: String },
    #[error("{tool} was stopped after {limit:.0?}")]
    TimedOut { tool: String, limit: Duration },
    #[error("the simulator: {0}")]
    Simulator(String),
}
