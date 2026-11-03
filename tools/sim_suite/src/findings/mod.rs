mod feature_report;
pub mod map_phase;
mod test_tally;

use std::fmt;

use crate::crash_reports::CrashReports;
use crate::suite::{Standing, TestName};
use feature_report::FeatureReport;
use map_phase::PhaseFigures;
use test_tally::TestTally;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SimFindings {
    device: &'static str,
    trouble: Option<String>,
    crash_reports: CrashReports,
    features: Vec<FeatureReport>,
    map_figures: Vec<PhaseFigures>,
    #[serde(skip)]
    tally: TestTally,
}

#[derive(Debug, thiserror::Error)]
#[error("the simulator run failed:\n{0}")]
pub struct Unproved(String);

impl SimFindings {
    const DEVICE: &str = "iPhone 16 Plus, 430 x 932";

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
            trouble: trouble.map(ToString::to_string),
            crash_reports,
            features: self.tally.reports(),
            ..self
        }
    }

    pub fn verdict(&self) -> Result<(), Unproved> {
        (self.trouble.is_none()
            && self.crash_reports.found().is_empty()
            && self
                .features
                .iter()
                .all(|report| report.standing() != Standing::Failed))
        .then_some(())
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
