mod ports;

use std::io::Write;

use basho_lints::class_sources::ClassSources;
use basho_lints::ownership::Ownership;
use basho_lints::ui_numbers::UiNumbers;
use basho_lints::{Report, SourceTree};
use file_ports::Files;
use style_lints::style_kinds::StyleKinds;
use style_lints::style_lengths::StyleLengths;
use style_lints::style_motion::StyleMotion;
use style_lints::style_sheet::StyleSheet;

use crate::task::failure::Failure;
use crate::task::ports::Host;
use crate::task::workspace::Workspace;
use ports::SharedLints;

pub struct Lint;

impl Lint {
    pub fn all(
        workspace: &Workspace,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        SharedLints::ran_on(workspace)?;
        let files = SourceTree::files_under(workspace.root())?;
        let sheet = StyleSheet::read(workspace.root())?;
        let reports: Vec<Report> = [
            Report::found::<UiNumbers>(&files),
            Report::found::<Ownership>(&files),
            sheet.found::<StyleLengths>(),
            sheet.found::<StyleMotion>(),
            sheet.found::<StyleKinds>(),
            Report::found::<ClassSources>(&files),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(reports)
            .filter(|reports| matches!(reports.as_slice(), [_, ..]))
            .map_or_else(
                || {
                    host.said_all([format_args!(
                        "lint: basho rules clean in {} rust files",
                        files.len()
                    )])
                },
                |reports| Err(basho_lints::Failure::Lint(reports).into()),
            )
    }
}
