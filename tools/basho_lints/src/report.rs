use std::fmt;

use crate::failure::Failure;
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
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn verdict(self) -> Result<(), Failure> {
        if self.is_clean() {
            Ok(())
        } else {
            Err(Failure::Lint(vec![self]))
        }
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
