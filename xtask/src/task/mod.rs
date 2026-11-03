pub mod catch_import;
pub mod ci;
mod failure;
pub mod flag;
mod fuzz_trip;
pub mod gate;
mod lint;
pub mod mutants;
mod print;
mod problem_count;
mod tile_build;
mod trip_check;
pub mod trip_text;
mod workspace;

use std::collections::BTreeSet;
use std::str::FromStr;

use basho_lints::class_sources::ClassSources;
use basho_lints::ownership::Ownership;
use basho_lints::ui_numbers::UiNumbers;
use basho_lints::{SourceRule, SourceTree};
use guide::words::Spelling;
use meta_xtask::Task as Shared;
use newtype::Newtype;
use strum::{EnumString, VariantNames};
use style_lints::SheetRule;
use style_lints::style_kinds::StyleKinds;
use style_lints::style_lengths::StyleLengths;
use style_lints::style_motion::StyleMotion;
use style_lints::style_sheet::StyleSheet;
use testflight_ports::Testflight;

use crate::task::failure::Failure;
use crate::task::flag::Flag;
use crate::task::lint::Lint;
use crate::task::print::Print;
use crate::task::tile_build::TileBuild;
use crate::task::trip_check::TripCheck;
use crate::task::workspace::Workspace;
use catch_import::CatchImport;
use ci::Ci;
use fuzz_trip::FuzzTrip;
use gate::Gate;
use mutants::Mutants;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum Task {
    Browser,
    CatchImport,
    Ci,
    FuzzTrip,
    Gate,
    Lint,
    Mutants,
    Ownership,
    Print,
    Sim,
    StyleKinds,
    StyleLengths,
    StyleMotion,
    Testflight,
    TileBuild,
    TripCheck,
    UiNumbers,
}

impl Task {
    pub fn run(self, flags: impl Iterator<Item = Flag>) -> Result<(), Failure> {
        let workspace = Workspace::locate();
        match self {
            Task::Browser => Ci::browser(&workspace, None),
            Task::CatchImport => CatchImport::import_catches(&workspace),
            Task::Ci => Ci::run(&workspace, flags),
            Task::FuzzTrip => FuzzTrip::from_flags(flags).campaign(&workspace),
            Task::Gate => Gate::run(&workspace, flags),
            Task::Lint => Lint::all(&workspace),
            Task::Mutants => Mutants::run(&workspace, flags.collect()),
            Task::Ownership => {
                Ok(Ownership::report(&SourceTree::files_under(workspace.root())?).verdict()?)
            }
            Task::Print => Print::write_pages(&workspace),
            Task::Sim => Gate::simulate(&workspace),
            Task::StyleKinds => Ok(StyleKinds::report(&StyleSheet::read(workspace.root())?)
                .verdict()
                .and(
                    ClassSources::report(&SourceTree::files_under(workspace.root())?).verdict(),
                )?),
            Task::StyleLengths => {
                Ok(StyleLengths::report(&StyleSheet::read(workspace.root())?).verdict()?)
            }
            Task::StyleMotion => {
                Ok(StyleMotion::report(&StyleSheet::read(workspace.root())?).verdict()?)
            }
            Task::Testflight => Ok(Testflight::run(workspace.root())?),
            Task::TileBuild => TileBuild::write_archive(&workspace),
            Task::TripCheck => TripCheck::print_problems(&workspace),
            Task::UiNumbers => {
                Ok(UiNumbers::report(&SourceTree::files_under(workspace.root())?).verdict()?)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Basho(Task),
    Shared(Shared),
}

impl Command {
    const SHARED: [Shared; 11] = [
        Shared::Affected,
        Shared::Clippy,
        Shared::MutantCosts,
        Shared::Mutants,
        Shared::MutantsDiffPlan,
        Shared::MutantsFull,
        Shared::MutantsPlan,
        Shared::Stacked,
        Shared::Test,
        Shared::TestRunner,
        Shared::TestTime,
    ];

    #[must_use]
    pub fn usage() -> Failure {
        Failure::Usage(Spelling::new(format!(
            "usage: cargo xtask <{}>",
            Task::VARIANTS
                .iter()
                .copied()
                .chain(Self::SHARED.iter().copied().map(<&str>::from))
                .collect::<BTreeSet<&str>>()
                .into_iter()
                .collect::<Vec<&str>>()
                .join("|")
        )))
    }

    pub fn run(self, flags: impl Iterator<Item = Flag>) -> Result<(), Failure> {
        match self {
            Command::Basho(task) => task.run(flags),
            Command::Shared(task) => task
                .run(flags.map(Flag::into_inner))
                .map_err(Failure::shared),
        }
    }
}

impl FromStr for Command {
    type Err = Failure;

    fn from_str(name: &str) -> Result<Command, Failure> {
        name.parse::<Task>()
            .map(Command::Basho)
            .or_else(|_| {
                name.parse::<Shared>()
                    .ok()
                    .filter(|task| Self::SHARED.contains(task))
                    .map(Command::Shared)
                    .ok_or(())
            })
            .map_err(|()| Command::usage())
    }
}
