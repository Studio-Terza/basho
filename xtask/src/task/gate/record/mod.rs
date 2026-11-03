mod milliseconds;
mod unix_seconds;

use std::fmt;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use guide::evidence::EvidenceCounts;
use sim_suite::SimFindings;

use crate::task::chromium::Chromium;
use crate::task::gate::{Commit, Outcome, StatusResult, Tree};
use milliseconds::Milliseconds;
use unix_seconds::UnixSeconds;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Record {
    procedure: &'static str,
    commit: Commit,
    tree: Tree,
    #[serde(skip)]
    file: &'static str,
    chromium: Option<PathBuf>,
    started_unix_seconds: UnixSeconds,
    elapsed_milliseconds: Milliseconds,
    outcome: Outcome,
    status: Option<StatusResult>,
    evidence: Option<EvidenceCounts>,
    #[serde(flatten)]
    findings: Option<SimFindings>,
}

impl Record {
    const GATE: &str = "gate";
    const GATE_FILE: &str = "gate.toml";
    const SIM: &str = "sim";
    const SIM_FILE: &str = "sim.toml";

    #[must_use]
    pub fn started(commit: Commit, tree: Tree, chromium: &Chromium, moment: SystemTime) -> Record {
        Record {
            procedure: Self::GATE,
            file: Self::GATE_FILE,
            commit,
            tree,
            chromium: Some(chromium.binary().to_path_buf()),
            started_unix_seconds: UnixSeconds::from(moment),
            elapsed_milliseconds: Milliseconds::from(Duration::ZERO),
            outcome: Outcome::Passed,
            status: Some(StatusResult::WithheldFailure),
            evidence: None,
            findings: None,
        }
    }

    #[must_use]
    pub fn finished(
        self,
        elapsed: Duration,
        outcome: Outcome,
        status: StatusResult,
        evidence: Option<EvidenceCounts>,
    ) -> Record {
        Record {
            elapsed_milliseconds: Milliseconds::from(elapsed),
            outcome,
            status: Some(status),
            evidence,
            ..self
        }
    }

    #[must_use]
    pub fn simulated(
        commit: Commit,
        tree: Tree,
        moment: SystemTime,
        findings: SimFindings,
    ) -> Record {
        Record {
            procedure: Self::SIM,
            file: Self::SIM_FILE,
            commit,
            tree,
            chromium: None,
            started_unix_seconds: UnixSeconds::from(moment),
            elapsed_milliseconds: Milliseconds::from(moment.elapsed().unwrap_or_default()),
            outcome: Outcome::from(&findings.verdict()),
            status: None,
            evidence: None,
            findings: Some(findings),
        }
    }

    #[must_use]
    pub fn path_under(&self, directory: &Path) -> PathBuf {
        directory.join(&self.commit).join(self.file)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        toml::to_string_pretty(self)
            .map_err(|_| fmt::Error)
            .and_then(|text| formatter.write_str(&text))
    }
}
