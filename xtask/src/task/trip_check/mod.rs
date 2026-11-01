use guide::TripSource;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::task::failure::Failure;
use crate::task::problem_count::ProblemCount;
use crate::task::trip_text::TripText;
use crate::task::workspace::Workspace;
use basho_lints::Site;

pub struct TripCheck;

impl TripCheck {
    const ARGUMENT_INDEX: usize = 2;
    pub const FOLDER: &str = "data/trips";
}

impl TripCheck {
    pub fn print_problems(workspace: &Workspace) -> Result<(), Failure> {
        let path = env::args()
            .nth(Self::ARGUMENT_INDEX)
            .map_or_else(|| TripText::path_in(workspace), PathBuf::from);
        Self::check_each(std::iter::once(Ok(path)))
    }

    pub fn every_file_in(folder: &Path) -> Result<(), Failure> {
        let site = |error| Failure::Io {
            site: Site::from(folder),
            error,
        };
        Self::check_each(
            fs::read_dir(folder)
                .map_err(site)?
                .map(|entry| entry.map(|found| found.path()).map_err(site)),
        )
    }

    #[expect(
        clippy::needless_for_each,
        reason = "expression-flow drives iterators with combinators, never a for loop"
    )]
    fn check_each(files: impl Iterator<Item = Result<PathBuf, Failure>>) -> Result<(), Failure> {
        files.into_iter().try_for_each(|file| {
            let path = file?;
            let text = fs::read_to_string(&path).map_err(|error| Failure::Io {
                site: Site::from(path.as_path()),
                error,
            })?;
            let findings = TripSource::new(&text).findings();
            findings.iter().for_each(|finding| println!("{finding}"));
            println!("trip-check: {} problems", findings.len());
            if findings.is_empty() {
                Ok(())
            } else {
                Err(Failure::Trip(ProblemCount::from(findings.as_slice())))
            }
        })
    }
}
