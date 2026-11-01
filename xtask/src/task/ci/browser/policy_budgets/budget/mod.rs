mod allowance;

use serde::Deserialize;

use allowance::Allowance;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Budget {
    allowance: Allowance,
}

impl Budget {
    pub fn allowance(&self) -> &Allowance {
        &self.allowance
    }
}
