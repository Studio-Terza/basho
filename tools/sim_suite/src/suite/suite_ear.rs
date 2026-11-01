use crate::suite::{Standing, SuiteWord, TestName};

pub trait SuiteEar {
    fn test_finished(&mut self, test: TestName, standing: Standing);
    fn phase_started(&mut self);
    fn phase_ended(&mut self, phase: SuiteWord);
}
