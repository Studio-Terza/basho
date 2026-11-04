use std::str::FromStr;

use crate::suite::{Standing, SuiteEar, SuiteWord, TestName};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuiteLine {
    Finished { test: TestName, standing: Standing },
    PhaseStart(SuiteWord),
    PhaseEnd(SuiteWord),
    Other,
}

impl SuiteLine {
    const CASE: &str = "Test Case '-[";
    const START: &str = "PHASE START ";
    const END: &str = "PHASE END ";
    const PASSED: &str = " passed (";
    const FAILED: &str = " failed (";
    const STANDINGS: [(&str, Standing); 2] = [
        (Self::PASSED, Standing::Passed),
        (Self::FAILED, Standing::Failed),
    ];

    pub fn told_to(self, ear: &mut impl SuiteEar) {
        match self {
            SuiteLine::Finished { test, standing } => ear.test_finished(test, standing),
            SuiteLine::PhaseStart(_) => ear.phase_started(),
            SuiteLine::PhaseEnd(phase) => ear.phase_ended(phase),
            SuiteLine::Other => (),
        }
    }
}

impl FromStr for SuiteLine {
    type Err = std::convert::Infallible;

    fn from_str(line: &str) -> Result<SuiteLine, Self::Err> {
        let finished = line
            .strip_prefix(Self::CASE)
            .and_then(|case| case.split_once("]'"))
            .and_then(|(name, rest)| {
                let standing = Self::STANDINGS
                    .into_iter()
                    .find(|(word, _)| rest.starts_with(word))
                    .map(|(_, standing)| standing)?;
                Some(SuiteLine::Finished {
                    test: name.parse().ok()?,
                    standing,
                })
            });
        Ok(finished
            .or_else(|| {
                line.strip_prefix(Self::START)
                    .and_then(|name| name.parse().ok())
                    .map(SuiteLine::PhaseStart)
            })
            .or_else(|| {
                line.strip_prefix(Self::END)
                    .and_then(|name| name.parse().ok())
                    .map(SuiteLine::PhaseEnd)
            })
            .unwrap_or(SuiteLine::Other))
    }
}
