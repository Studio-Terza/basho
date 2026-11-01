use guide::words::Spelling;
use std::ffi::OsStr;
use std::fmt;
use std::process::Command;

use crate::task::failure::Failure;

pub struct Osmium<'arguments> {
    arguments: &'arguments [&'arguments OsStr],
}

impl<'arguments> Osmium<'arguments> {
    const TOOL: &str = "osmium";

    pub fn new(arguments: &'arguments [&'arguments OsStr]) -> Osmium<'arguments> {
        Osmium { arguments }
    }

    pub fn run(self) -> Result<(), Failure> {
        let output = Command::new(Self::TOOL)
            .args(self.arguments)
            .output()
            .map_err(|error| Failure::Tool {
                tool: Spelling::new(Self::TOOL.to_owned()),
                detail: Spelling::new(error.to_string()),
            })?;
        output
            .status
            .success()
            .then_some(())
            .ok_or_else(|| Failure::Tool {
                tool: Spelling::new(Self::TOOL.to_owned()),
                detail: Spelling::new(String::from_utf8_lossy(&output.stderr).into_owned()),
            })
    }
}

impl fmt::Display for Osmium<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(Self::TOOL)?;
        self.arguments
            .iter()
            .try_for_each(|argument| write!(formatter, " {}", argument.to_string_lossy()))
    }
}
