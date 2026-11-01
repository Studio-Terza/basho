mod budget;
mod time_limits;

use std::collections::BTreeMap;
use std::str::FromStr;

use serde::Deserialize;

use crate::task::failure::Failure;
use budget::Budget;
use guide::words::Spelling;
use time_limits::TimeLimits;

#[derive(Debug, Default, Deserialize)]
pub struct PolicyBudgets {
    #[serde(default)]
    budgets: BTreeMap<Spelling, Budget>,
}

impl PolicyBudgets {
    #[must_use]
    pub fn limits(&self, name: &Spelling) -> Option<TimeLimits> {
        let whole = self.budgets.get(name)?.allowance().whole()?;
        let warning = self
            .budgets
            .values()
            .find_map(|budget| budget.allowance().portion_of(name, whole));
        Some(TimeLimits::new(whole, warning.unwrap_or(whole)))
    }
}

impl FromStr for PolicyBudgets {
    type Err = Failure;

    fn from_str(policy: &str) -> Result<PolicyBudgets, Failure> {
        toml::from_str(policy).map_err(Failure::Policy)
    }
}
