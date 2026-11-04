use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use file_ports::{FileFailure, Files};

use crate::task::failure::Failure;
use crate::task::problem_count::ProblemCount;
use guide::words::Spelling;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MissedReport(BTreeMap<Spelling, Vec<Spelling>>);

impl MissedReport {
    const LIST: &str = "mutants.out/missed.txt";
    const GROUPS: [&str; 2] = ["crates", "tools"];

    pub fn in_directory(directory: &Path, files: &impl Files) -> Result<MissedReport, Failure> {
        let lists: Vec<PathBuf> = files
            .entries(directory)
            .or_else(|failure| Self::absent_as_empty(directory, failure))?
            .iter()
            .map(|run| run.join(Self::LIST))
            .collect();
        lists.iter().zip(files.read_text_many(&lists)).try_fold(
            MissedReport::default(),
            |all, (list, text)| {
                let text = text.or_else(|failure| Self::absent_as_empty(list, failure))?;
                Ok(all.merged(MissedReport::from(text.as_ref())))
            },
        )
    }

    fn absent_as_empty<Found: Default>(
        path: &Path,
        failure: FileFailure,
    ) -> Result<Found, Failure> {
        match failure {
            FileFailure::Missing(_) => Ok(Found::default()),
            FileFailure::Failed(error) => Err(Failure::unreadable(path, error)),
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
