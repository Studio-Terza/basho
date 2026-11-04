mod top_probe;

use sim_suite::SuiteLine;

use crate::suite_run::SuiteRun;
pub use top_probe::TopProbe;

impl SuiteRun {
    pub fn heard(&mut self, line: &str) {
        line.parse::<SuiteLine>()
            .into_iter()
            .for_each(|line| line.told_to(self));
    }
}
