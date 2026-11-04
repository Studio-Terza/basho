mod boundary;
pub mod catch_import;
pub mod chromium;
pub mod ci;
mod command;
mod failure;
pub mod flag;
mod fuzz_trip;
pub mod gate;
mod lint;
pub mod mutants;
mod ports;
mod print;
mod problem_count;
mod tile_build;
mod trip_check;
pub mod trip_text;
mod workspace;

pub use command::Command;

use std::io::Write;

use basho_lints::class_sources::ClassSources;
use basho_lints::ownership::Ownership;
use basho_lints::ui_numbers::UiNumbers;
use basho_lints::{SourceRule, SourceTree};
use file_ports::Files;

use crate::task::ports::Host;
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
    pub fn run(
        self,
        flags: impl Iterator<Item = Flag>,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        let workspace = Workspace::locate();
        match self {
            Task::Browser => Ci::browser(&workspace, None),
            Task::CatchImport => CatchImport::import_catches(&workspace),
            Task::Ci => Ci::run(&workspace, flags, host),
            Task::FuzzTrip => FuzzTrip::from_flags(flags).campaign(&workspace),
            Task::Gate => Gate::run(&workspace, flags, host),
            Task::Lint => Lint::all(&workspace, host),
            Task::Mutants => Mutants::run(&workspace, flags.collect(), host),
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
            Task::TripCheck => TripCheck::print_problems(&workspace, flags, host),
            Task::UiNumbers => {
                Ok(UiNumbers::report(&SourceTree::files_under(workspace.root())?).verdict()?)
            }
        }
    }
}
