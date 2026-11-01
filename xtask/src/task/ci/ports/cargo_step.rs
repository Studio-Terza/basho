use std::collections::BTreeMap;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

use basho_lints::Site;

use crate::task::failure::Failure;
use crate::task::workspace::Workspace;

pub struct CargoStep<'step> {
    arguments: &'step [&'step str],
    settings: BTreeMap<&'step str, String>,
}

impl<'step> CargoStep<'step> {
    const CARGO: &str = "CARGO";
    const FALLBACK: &str = "cargo";
    const ONLY: &str = "UI_CHECK_ONLY";

    #[must_use]
    pub fn new(arguments: &'step [&'step str]) -> CargoStep<'step> {
        CargoStep {
            arguments,
            settings: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn setting(mut self, name: &'step str, value: String) -> CargoStep<'step> {
        self.settings.insert(name, value);
        self
    }

    pub fn ran_in(self, workspace: &Workspace) -> Result<(), Failure> {
        let io = |error| Failure::Io {
            site: Site::Workspace,
            error,
        };
        let output =
            Command::new(env::var_os(Self::CARGO).unwrap_or_else(|| Self::FALLBACK.into()))
                .args(self.arguments)
                .envs(&self.settings)
                .current_dir(workspace.root())
                .env_remove(Self::ONLY)
                .stderr(Stdio::inherit())
                .output()
                .map_err(io)?;
        std::io::stdout().write_all(&output.stdout).map_err(io)?;
        output
            .status
            .success()
            .then_some(())
            .ok_or(Failure::Cargo(output.status))
    }
}
