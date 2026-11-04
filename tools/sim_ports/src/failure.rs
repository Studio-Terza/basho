use std::io;
use std::path::PathBuf;
use std::time::Duration;

use thiserror::Error;
use tool_text::{ToolName, ToolOutput};

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{}: {error}", .path.display())]
    Io { path: PathBuf, error: io::Error },
    #[error("{tool} failed:\n{detail}")]
    Tool { tool: ToolName, detail: ToolOutput },
    #[error("{tool} was stopped after {limit:.0?}")]
    TimedOut { tool: ToolName, limit: Duration },
    #[error("the simulator: {0}")]
    Simulator(ToolOutput),
}
