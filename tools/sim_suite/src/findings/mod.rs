mod feature_report;
pub mod map_phase;
mod test_tally;
mod wording;

use std::fmt;

use crate::crash_reports::CrashReports;
use crate::suite::{Standing, TestName};
use feature_report::FeatureReport;
use map_phase::PhaseFigures;
use test_tally::TestTally;
use wording::Wording;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SimFindings {
    device: Wording,
    trouble: Option<Trouble>,
    crash_reports: CrashReports,
    features: Vec<FeatureReport>,
    map_figures: Vec<PhaseFigures>,
    #[serde(skip)]
    tally: TestTally,
}

#[derive(
    newtype::Newtype,
    newtype::Arbitrary,
    newtype::Display,
    Clone,
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
)]
#[serde(transparent)]
pub struct Trouble(String);

#[derive(Debug, thiserror::Error)]
#[error("the simulator run failed:\n{0}")]
pub struct Unproved(String);

impl SimFindings {
    const DEVICE: Wording = Wording::new("iPhone 16 Plus, 430 x 932");

    pub fn heard_test(&mut self, test: TestName, standing: Standing) {
        self.tally = std::mem::take(&mut self.tally).heard(test, standing);
    }

    pub fn heard_figures(&mut self, figures: PhaseFigures) {
        self.map_figures.push(figures);
    }

    #[must_use]
    pub fn closed(
        self,
        crash_reports: CrashReports,
        trouble: Option<&impl std::error::Error>,
    ) -> SimFindings {
        SimFindings {
            device: Self::DEVICE,
            trouble: trouble.map(ToString::to_string).map(Trouble::new),
            crash_reports,
            features: self.tally.reports(),
            ..self
        }
    }

    pub fn verdict(&self) -> Result<(), Unproved> {
        Some(self)
            .filter(|findings| findings.trouble.is_none())
            .filter(|findings| findings.crash_reports.found().is_empty())
            .filter(|findings| {
                findings
                    .features
                    .iter()
                    .all(|report| report.standing() != Standing::Failed)
            })
            .map(drop)
            .ok_or_else(|| Unproved(self.to_string()))
    }
}

impl fmt::Display for SimFindings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.trouble
            .iter()
            .try_for_each(|trouble| writeln!(formatter, "{trouble}"))?;
        self.features.iter().try_for_each(|report| {
            writeln!(formatter, "{:?}: {:?}", report.feature(), report.standing())
        })?;
        self.crash_reports
            .found()
            .iter()
            .try_for_each(|report| writeln!(formatter, "crash {}", report.display()))
    }
}
