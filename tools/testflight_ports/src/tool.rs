use std::ffi::OsStr;
use std::fmt;
use std::path::Path;
use std::process::{Command, Output};
use std::str::FromStr;
use tool_text::{ToolName, ToolOutput};

use crate::bits::RawBytes;
use crate::failure::Failure;
use crate::xcrun_tool::XcrunTool;

pub struct Tool {
    command: Command,
    name: String,
}

impl Tool {
    const DETAIL_LEN: usize = 12;

    pub fn named(program: &str) -> Tool {
        Tool {
            command: Command::new(program),
            name: program.to_owned(),
        }
    }

    pub fn xcrun(program: XcrunTool) -> Tool {
        let name: &str = program.into();
        Tool {
            name: name.to_owned(),
            ..Tool::named("xcrun").arg(name)
        }
    }

    pub fn arg(mut self, argument: impl AsRef<OsStr>) -> Tool {
        self.command.arg(argument);
        self
    }

    pub fn rendered(self, argument: &impl fmt::Display) -> Tool {
        self.arg(argument.to_string())
    }

    pub fn args<I, S>(mut self, arguments: I) -> Tool
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.command.args(arguments);
        self
    }

    pub fn env(mut self, key: &str, setting: &str) -> Tool {
        self.command.env(key, setting);
        self
    }

    pub fn within(mut self, directory: &Path) -> Tool {
        self.command.current_dir(directory);
        self
    }

    pub fn output(self) -> Result<RawBytes, Failure> {
        self.ran().map(|output| RawBytes::from(output.stdout))
    }

    pub fn text(self) -> Result<String, Failure> {
        self.output()
            .map(|bytes| String::from_utf8_lossy(bytes.as_slice()).into_owned())
    }

    pub fn transcript<Parsed: FromStr<Err: Into<Failure>>>(self) -> Result<Parsed, Failure> {
        self.ran()
            .and_then(|output| Self::combined(&output).parse().map_err(Into::into))
    }

    pub fn status(mut self) -> Result<(), Failure> {
        self.command
            .status()
            .ok()
            .filter(std::process::ExitStatus::success)
            .map(|_| ())
            .ok_or(Failure::Tool {
                tool: ToolName::new(self.to_string()),
                detail: ToolOutput::new("exited with an error; see the output above".to_owned()),
            })
    }

    fn ran(mut self) -> Result<Output, Failure> {
        let output = self.command.output().map_err(|error| Failure::Tool {
            tool: ToolName::new(self.to_string()),
            detail: ToolOutput::new(error.to_string()),
        })?;
        if output.status.success() {
            Ok(output)
        } else {
            Err(Failure::Tool {
                tool: ToolName::new(self.to_string()),
                detail: ToolOutput::new(Self::detail(&Self::combined(&output))),
            })
        }
    }

    fn combined(output: &Output) -> String {
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }

    fn detail(text: &str) -> String {
        let lines: Vec<&str> = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        lines[lines.len().saturating_sub(Self::DETAIL_LEN)..].join("\n")
    }
}

impl fmt::Display for Tool {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}
