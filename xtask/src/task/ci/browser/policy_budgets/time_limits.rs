use std::time::Duration;

use crate::task::failure::Failure;
use guide::words::Label;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeLimits {
    limit: Duration,
    warning: Duration,
}

impl TimeLimits {
    pub const fn new(limit: Duration, warning: Duration) -> TimeLimits {
        TimeLimits { limit, warning }
    }

    pub fn past_warning(self, elapsed: Duration) -> Option<Duration> {
        (elapsed > self.warning).then_some(self.warning)
    }

    pub fn judged(self, step: Label, elapsed: Duration) -> Result<(), Failure> {
        if elapsed > self.limit {
            Err(Failure::OverBudget {
                step,
                elapsed,
                limit: self.limit,
            })
        } else {
            Ok(())
        }
    }
}
