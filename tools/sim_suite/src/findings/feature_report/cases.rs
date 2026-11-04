use crate::suite::{Standing, SuiteWord};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cases {
    NonePassed,
    AllPassed,
    SomeFailed,
}

impl Cases {
    pub fn heard(passed: &[SuiteWord], failed: &[SuiteWord]) -> Cases {
        let none_passed = passed.is_empty().then_some(Cases::NonePassed);
        let all_passed = failed.is_empty().then_some(Cases::AllPassed);
        none_passed.or(all_passed).unwrap_or(Cases::SomeFailed)
    }

    pub fn standing(self) -> Standing {
        match self {
            Cases::AllPassed => Standing::Passed,
            Cases::NonePassed | Cases::SomeFailed => Standing::Failed,
        }
    }
}
