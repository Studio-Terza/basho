mod ports;

use basho_lints::class_sources::ClassSources;
use basho_lints::ownership::Ownership;
use basho_lints::ui_numbers::UiNumbers;
use basho_lints::{Report, SourceRule, SourceTree};
use style_lints::SheetRule;
use style_lints::style_kinds::StyleKinds;
use style_lints::style_lengths::StyleLengths;
use style_lints::style_motion::StyleMotion;
use style_lints::style_sheet::StyleSheet;

use crate::task::failure::Failure;
use crate::task::workspace::Workspace;
use ports::SharedLints;

pub struct Lint;

impl Lint {
    pub fn all(workspace: &Workspace) -> Result<(), Failure> {
        SharedLints::ran_on(workspace)?;
        let files = SourceTree::files_under(workspace.root())?;
        let sheet = StyleSheet::read(workspace.root())?;
        let reports: Vec<Report> = [
            UiNumbers::report(&files),
            Ownership::report(&files),
            StyleLengths::report(&sheet),
            StyleMotion::report(&sheet),
            StyleKinds::report(&sheet),
            ClassSources::report(&files),
        ]
        .into_iter()
        .filter(|report| !report.is_clean())
        .collect();
        if reports.is_empty() {
            println!("lint: basho rules clean in {} rust files", files.len());
            Ok(())
        } else {
            Err(basho_lints::Failure::Lint(reports).into())
        }
    }
}
