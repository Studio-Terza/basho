mod missed_report;

use std::io::Write;
use std::path::PathBuf;

use file_ports::Files;

use meta_xtask::Task as Shared;
use newtype::Newtype;

use crate::task::failure::Failure;
use crate::task::flag::Flag;
use crate::task::gate::{Gate, Git};
use crate::task::ports::Host;
use crate::task::workspace::Workspace;
pub use missed_report::MissedReport;

pub struct Mutants;

impl Mutants {
    const FULL: &str = "--full";
    const OUTPUT: &str = "target/mutants";
    const ARTIFACT: &str = "mutants.txt";

    pub fn run(
        workspace: &Workspace,
        flags: Vec<Flag>,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        let (full, rest): (Vec<Flag>, Vec<Flag>) = flags
            .into_iter()
            .partition(|flag| flag.clone().into_inner() == Self::FULL);
        let task = if full.is_empty() {
            Shared::Mutants
        } else {
            Shared::MutantsFull
        };
        let result = task
            .run(rest.into_iter().map(Flag::into_inner))
            .map_err(Failure::shared);
        let report =
            MissedReport::in_directory(&workspace.root().join(Self::OUTPUT), host.files())?;
        host.wrote(&report)?;
        let path: PathBuf = workspace
            .root()
            .join(Gate::ARTIFACTS)
            .join(Git::head(workspace)?)
            .join(Self::ARTIFACT);
        let written = Gate::written(&path, &report);
        result.and(written)
    }
}
