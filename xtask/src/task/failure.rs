use std::error::Error;
use std::io;
use std::time::Duration;

use thiserror::Error;

use crate::task::problem_count::ProblemCount;
use basho_lints::Site;
use guide::words::{Label, Spelling};

#[derive(Debug, Error)]
pub enum Failure {
    #[error("{0}")]
    Usage(Spelling),
    #[error("{0}")]
    Shared(Box<dyn Error>),
    #[error("{site}: {error}")]
    Io { site: Site, error: io::Error },
    #[error("an argument is not UTF-8 text: {0:?}")]
    NotText(std::ffi::OsString),
    #[error("writing to the console: {0}")]
    Console(io::Error),
    #[error("missing or invalid environment variable {0}")]
    Environment(Label),
    #[error("{tool} failed:\n{detail}")]
    Tool { tool: Spelling, detail: Spelling },
    #[error("catching features: {0}")]
    Catch(serde_json::Error),
    #[error("catching features: {0}")]
    CatchToml(toml::ser::Error),
    #[error("git did not print a full commit hash")]
    NotACommit,
    #[error("trip data: {0} problems")]
    Trip(ProblemCount),
    #[error("map data: {0}")]
    MapData(serde_json::Error),
    #[error("{0}")]
    Testflight(#[from] testflight_ports::Failure),
    #[error("{0}")]
    TripFile(#[from] guide::TripError),
    #[error("{0}")]
    Rules(#[from] basho_lints::Failure),
    #[error("{0}")]
    Simulator(#[from] sim_ports::Failure),
    #[error("{0}")]
    Unproved(#[from] sim_suite::Unproved),
    #[error("the shared lints found violations")]
    PortedLints,
    #[error(
        "the shared lints did not start ({0}); install them with cargo install --git https://github.com/Studio-Terza/meta-dev --rev 10eaa06225 --locked lint"
    )]
    LintRunner(io::Error),
    #[error("policy: {0}")]
    Policy(toml::de::Error),
    #[error("the cargo step failed ({0})")]
    Cargo(std::process::ExitStatus),
    #[error("policy: no wall-time budget for the {0} step")]
    Unbudgeted(Label),
    #[error("{step} took {elapsed:.1?}, over its budget of {limit:.1?}")]
    OverBudget {
        step: Label,
        elapsed: Duration,
        limit: Duration,
    },
}

impl Failure {
    pub fn unreadable(path: &std::path::Path, error: io::Error) -> Failure {
        Failure::Io {
            site: Site::from(path),
            error,
        }
    }

    pub fn shared(failure: impl Error + 'static) -> Failure {
        Failure::Shared(Box::new(failure))
    }
}
