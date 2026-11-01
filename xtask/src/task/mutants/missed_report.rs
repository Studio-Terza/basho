use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use crate::task::failure::Failure;
use crate::task::problem_count::ProblemCount;
use guide::words::Spelling;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MissedReport(BTreeMap<Spelling, Vec<Spelling>>);

impl MissedReport {
    const LIST: &str = "mutants.out/missed.txt";
    const GROUPS: [&str; 2] = ["crates", "tools"];

    pub fn in_directory(directory: &Path) -> Result<MissedReport, Failure> {
        Self::runs(directory)?
            .iter()
            .try_fold(MissedReport::default(), |all, run| {
                Ok(all.merged(MissedReport::from(Self::listed(run)?.as_str())))
            })
    }

    fn runs(directory: &Path) -> Result<Vec<PathBuf>, Failure> {
        fs::read_dir(directory)
            .and_then(Iterator::collect::<io::Result<Vec<fs::DirEntry>>>)
            .map(|runs| runs.iter().map(fs::DirEntry::path).collect())
            .or_else(|error| {
                (error.kind() == ErrorKind::NotFound)
                    .then(Vec::new)
                    .ok_or(error)
            })
            .map_err(|error| Failure::unreadable(directory, error))
    }

    fn listed(run: &Path) -> Result<String, Failure> {
        let list = run.join(Self::LIST);
        match fs::read_to_string(&list) {
            Err(error) if error.kind() != ErrorKind::NotFound => {
                Err(Failure::unreadable(&list, error))
            }
            found => Ok(found.unwrap_or_default()),
        }
    }

    fn merged(mut self, other: MissedReport) -> MissedReport {
        other
            .0
            .into_iter()
            .for_each(|(name, lines)| self.0.entry(name).or_default().extend(lines));
        self
    }

    fn crate_of(line: &str) -> Spelling {
        let path = line.split(':').next().unwrap_or(line);
        Spelling::new(match path.split('/').collect::<Vec<&str>>().as_slice() {
            [group, name, ..] if Self::GROUPS.contains(group) => (*name).to_owned(),
            [first, ..] => (*first).to_owned(),
            [] => path.to_owned(),
        })
    }
}

impl From<&str> for MissedReport {
    fn from(text: &str) -> MissedReport {
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .fold(MissedReport::default(), |mut report, line| {
                report
                    .0
                    .entry(Self::crate_of(line))
                    .or_default()
                    .push(Spelling::new(line.to_owned()));
                report
            })
    }
}

impl fmt::Display for MissedReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let all: Vec<&Spelling> = self.0.values().flatten().collect();
        writeln!(
            formatter,
            "missed mutants: {}",
            ProblemCount::from(all.as_slice())
        )?;
        self.0.iter().try_for_each(|(name, lines)| {
            writeln!(
                formatter,
                "{name}: {}",
                ProblemCount::from(lines.as_slice())
            )?;
            lines
                .iter()
                .try_for_each(|line| writeln!(formatter, "  {line}"))
        })
    }
}
