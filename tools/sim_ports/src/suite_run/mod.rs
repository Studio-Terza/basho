mod top_probe;

use crate::phase_length::PhaseLength;
use sim_suite::map_phase::PhaseFigures;
use sim_suite::{CrashReports, SimFindings, Standing, SuiteEar, SuiteLine, SuiteWord, TestName};

use crate::device::Udid;
use crate::failure::Failure;
use top_probe::TopProbe;

#[derive(Default)]
pub struct SuiteRun {
    findings: SimFindings,
    probe: Option<TopProbe>,
    udid: Udid,
}

impl SuiteRun {
    pub fn watching(&mut self, udid: Udid) {
        self.udid = udid;
    }

    pub fn heard(&mut self, line: &str) {
        line.parse::<SuiteLine>()
            .into_iter()
            .for_each(|line| line.told_to(self));
    }

    #[must_use]
    pub fn closed(self, crashes: CrashReports, result: &Result<(), Failure>) -> SimFindings {
        self.findings.closed(crashes, result.as_ref().err())
    }
}

impl SuiteEar for SuiteRun {
    fn test_finished(&mut self, test: TestName, standing: Standing) {
        println!("sim: {:?} {standing:?}", test.case());
        self.findings.heard_test(test, standing);
    }

    fn phase_started(&mut self) {
        self.probe = Some(TopProbe::started(&self.udid, PhaseLength::SIMULATED));
    }

    fn phase_ended(&mut self, phase: SuiteWord) {
        self.probe
            .take()
            .map(|probe| PhaseFigures::from((phase, probe.finished())))
            .into_iter()
            .for_each(|figures| self.findings.heard_figures(figures));
    }
}
