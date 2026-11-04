use std::io::Write;

use file_ports::Files;
use meta_xtask::Task as Shared;

pub mod browser;
mod ports;

use crate::task::chromium::Chromium;
use crate::task::failure::Failure;
use crate::task::flag::Flag;
use crate::task::lint::Lint;
use crate::task::ports::{CargoStep, Host};
use crate::task::trip_check::TripCheck;
use crate::task::workspace::Workspace;
use browser::Browser;
use newtype::Newtype;
use ports::ChromeSession;

pub struct Ci;

impl Ci {
    const FORMATTED: [&str; 3] = ["fmt", "--all", "--check"];

    pub fn run(
        workspace: &Workspace,
        flags: impl Iterator<Item = Flag>,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        Self::run_with(workspace, flags, None, host)
    }

    pub fn run_with(
        workspace: &Workspace,
        flags: impl Iterator<Item = Flag>,
        chromium: Option<&Chromium>,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        let flags: Vec<String> = flags.map(Flag::into_inner).collect();
        CargoStep::new(&Self::FORMATTED).ran_in(workspace)?;
        Shared::Clippy
            .run(flags.iter().cloned())
            .map_err(Failure::shared)?;
        Shared::TestTime
            .run(flags.into_iter())
            .map_err(Failure::shared)?;
        Lint::all(workspace, host)?;
        TripCheck::every_file_in(&workspace.root().join(TripCheck::FOLDER), host)?;
        Self::browser(workspace, chromium)
    }

    pub fn browser(workspace: &Workspace, chromium: Option<&Chromium>) -> Result<(), Failure> {
        let _chrome = ChromeSession::start(workspace, chromium)?;
        Browser::check_pages(workspace)
    }
}
