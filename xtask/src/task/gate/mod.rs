mod commit;
mod detail;
mod outcome;
mod ports;
mod posting;
mod record;
mod status_result;
mod tree;

use std::env;
use std::fmt::Display;
use std::fs;
use std::io::Write;

use file_ports::Files;

use crate::task::ports::Host;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};

use crate::task::chromium::Chromium;
use crate::task::ci::Ci;
use crate::task::failure::Failure;
use crate::task::flag::Flag;
use crate::task::trip_text::TripText;
use crate::task::workspace::Workspace;
use basho_lints::Site;
use guide::EvidenceCounts;
use guide::TripSource;
use guide::words::Label;
pub(crate) use ports::Git;
use ports::Status;
use sim_ports::Sim;

pub use commit::Commit;
pub use detail::Detail;
pub use outcome::Outcome;
pub use posting::Posting;
pub use record::Record;
pub use status_result::StatusResult;
pub use tree::Tree;

pub struct Gate;

impl Gate {
    const CHROME: &str = "CHROME";
    const HOME: &str = "HOME";
    const CACHE: &str = "Library/Caches/ms-playwright";
    pub(crate) const ARTIFACTS: &str = "target/procedures";

    pub fn run(
        workspace: &Workspace,
        flags: impl Iterator<Item = Flag>,
        host: &mut Host<'_, impl Files, impl Write>,
    ) -> Result<(), Failure> {
        let chromium = Self::chromium()?;
        let commit = Git::head(workspace)?;
        let tree = Git::tree(workspace)?;
        let record = Record::started(commit.clone(), tree, &chromium, SystemTime::now());
        let started = Instant::now();
        let result = Ci::run_with(workspace, flags, Some(&chromium), host)
            .and_then(|()| Self::evidence_counts(workspace));
        let elapsed = started.elapsed();
        let outcome = Outcome::from(&result);
        let status = Self::posted(workspace, &commit, outcome.posting(tree));
        println!("gate: {status:?} for {commit}");
        let counts = result.as_ref().ok().copied();
        let finished = record.finished(elapsed, outcome, status, counts);
        let written = Self::written(
            &finished.path_under(&workspace.root().join(Self::ARTIFACTS)),
            &finished,
        );
        result.and(written)
    }

    pub fn simulate(workspace: &Workspace) -> Result<(), Failure> {
        let commit = Git::head(workspace)?;
        let tree = Git::tree(workspace)?;
        let root = workspace.root().join(Self::ARTIFACTS);
        let moment = SystemTime::now();
        let findings = Sim::proved(workspace.root(), &root.join(&commit));
        let verdict = findings.verdict();
        let record = Record::simulated(commit, tree, moment, findings);
        let written = Self::written(&record.path_under(&root), &record);
        verdict.map_err(Failure::from).and(written)
    }

    fn posted(workspace: &Workspace, commit: &Commit, posting: Posting) -> StatusResult {
        match posting {
            Posting::Post => Status::post_success(workspace, commit).map_or_else(
                |failure| StatusResult::PostFailed(Detail::from(&failure)),
                |()| StatusResult::Posted,
            ),
            other => StatusResult::from(other),
        }
    }

    fn evidence_counts(workspace: &Workspace) -> Result<EvidenceCounts, Failure> {
        let path = TripText::path_in(workspace);
        let text = fs::read_to_string(&path).map_err(|error| Failure::Io {
            site: Site::from(path.as_path()),
            error,
        })?;
        let counts = TripSource::new(&text).evidence_counts()?;
        println!("gate: evidence {counts}");
        Ok(counts)
    }

    fn chromium() -> Result<Chromium, Failure> {
        let cache = env::var_os(Self::HOME)
            .map(|home| PathBuf::from(home).join(Self::CACHE))
            .ok_or_else(|| Failure::Environment(Label::new(Self::HOME)))?;
        Chromium::installed_in(&cache).ok_or_else(|| Failure::Environment(Label::new(Self::CHROME)))
    }

    pub(crate) fn written(path: &Path, record: &impl Display) -> Result<(), Failure> {
        let io = |error| Failure::Io {
            site: Site::from(path),
            error,
        };
        fs::create_dir_all(path.parent().unwrap_or(path)).map_err(io)?;
        write!(fs::File::create(path).map_err(io)?, "{record}").map_err(io)?;
        println!("artifact {}", path.display());
        Ok(())
    }
}
