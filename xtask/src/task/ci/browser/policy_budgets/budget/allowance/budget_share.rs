use std::time::Duration;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct BudgetShare(u32);

impl BudgetShare {
    const WHOLE: u32 = 100;

    pub fn portion_of(self, whole: Duration) -> Duration {
        whole * self.0 / Self::WHOLE
    }
}
