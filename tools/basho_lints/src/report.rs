use std::fmt;

use crate::SourceRule;
use crate::failure::Failure;
use crate::source_file::SourceFile;
use crate::violation::Violation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    headline: String,
    violations: Vec<Violation>,
}

impl Report {
    #[must_use]
    pub fn new(headline: impl fmt::Display, violations: Vec<Violation>) -> Report {
        Report {
            headline: headline.to_string(),
            violations,
        }
    }

    #[must_use]
    pub fn found<Rule: SourceRule>(files: &[SourceFile]) -> Option<Report> {
        Rule::report(files).unclean()
    }

    #[must_use]
    pub fn unclean(self) -> Option<Report> {
        Some(self).filter(|report| matches!(report.violations.as_slice(), [_, ..]))
    }

    pub fn verdict(self) -> Result<(), Failure> {
        self.unclean()
            .map_or(Ok(()), |report| Err(Failure::Lint(vec![report])))
    }
}

impl fmt::Display for Report {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}:", self.headline)?;
        self.violations
            .iter()
            .try_for_each(|violation| writeln!(formatter, "{violation}"))
    }
}
