mod budget_millis;
mod budget_share;
use std::time::Duration;

use serde::Deserialize;

use budget_millis::BudgetMillis;
use budget_share::BudgetShare;
use guide::words::Spelling;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Allowance {
    Whole { duration: BudgetMillis },
    PartOf { of: Spelling, percent: BudgetShare },
}

impl Allowance {
    pub fn whole(&self) -> Option<Duration> {
        match self {
            Allowance::Whole { duration } => Some(duration.duration()),
            Allowance::PartOf { .. } => None,
        }
    }

    pub fn portion_of(&self, parent: &Spelling, whole: Duration) -> Option<Duration> {
        match self {
            Allowance::PartOf { of, percent } if of == parent => Some(percent.portion_of(whole)),
            Allowance::PartOf { .. } | Allowance::Whole { .. } => None,
        }
    }
}
