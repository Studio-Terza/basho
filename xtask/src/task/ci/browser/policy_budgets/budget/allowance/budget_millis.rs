use std::time::Duration;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct BudgetMillis(u64);

impl BudgetMillis {
    #[must_use]
    pub fn duration(self) -> Duration {
        Duration::from_millis(self.0)
    }
}
