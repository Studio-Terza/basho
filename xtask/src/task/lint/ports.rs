use std::io;
use std::process::Command;

use crate::task::failure::Failure;
use crate::task::workspace::Workspace;

pub struct SharedLints;

impl SharedLints {
    const BINARY: &str = "lint";

    pub fn ran_on(workspace: &Workspace) -> Result<(), Failure> {
        Command::new(Self::BINARY)
            .arg(workspace.root())
            .status()
            .map_err(|error: io::Error| Failure::LintRunner(error))?
            .success()
            .then_some(())
            .ok_or(Failure::PortedLints)
    }
}
