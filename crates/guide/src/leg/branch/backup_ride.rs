use serde::Deserialize;

use crate::clock::ClockTime;
use crate::end::End;
use crate::times::ListedTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackupRide {
    leaves: ListedTime,
    arrives: ListedTime,
}

impl BackupRide {
    #[must_use]
    pub fn time(self, end: End) -> ClockTime {
        ClockTime::from(self.listed(end))
    }

    #[must_use]
    pub fn listed(self, end: End) -> ListedTime {
        match end {
            End::Start => self.leaves,
            End::Finish => self.arrives,
        }
    }

    #[must_use]
    pub fn runs_out_of_order(self, departure: ClockTime) -> Option<BackupRide> {
        (self.time(End::Start) <= departure)
            .then_some(self)
            .or_else(|| (self.time(End::Finish) <= self.time(End::Start)).then_some(self))
    }
}
