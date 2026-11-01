use crate::backup_mark::BackupMark;
use crate::clock::{ClockTime, Minutes, TripTime};
use crate::times::{Basis, ListedTime, LiveTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Backup {
    leaves: Option<ListedTime>,
    arrives: Option<TripTime>,
    check_in: Option<TripTime>,
}

impl Backup {
    #[must_use]
    pub fn ride(
        leaves: ListedTime,
        arrives: Option<TripTime>,
        check_in: Option<TripTime>,
    ) -> Backup {
        Backup {
            leaves: Some(leaves),
            arrives,
            check_in,
        }
    }

    #[must_use]
    pub fn none(check_in: Option<TripTime>) -> Backup {
        Backup {
            leaves: None,
            arrives: None,
            check_in,
        }
    }

    #[must_use]
    pub fn time(self, mark: BackupMark) -> Option<ClockTime> {
        match mark {
            BackupMark::Leaves => self.leaves.map(ClockTime::from),
            BackupMark::Arrives => self.arrives.map(TripTime::time),
            BackupMark::CheckIn => self.check_in.map(TripTime::time),
        }
    }

    #[must_use]
    pub fn leaves(self) -> Option<ListedTime> {
        self.leaves
    }

    #[must_use]
    pub fn arrives(self) -> Option<LiveTime> {
        self.arrives
            .map(|arrives| LiveTime::new(arrives.time(), Basis::PlanAfterRide))
    }

    #[must_use]
    pub fn after_check_in(self) -> Option<Minutes> {
        self.check_in
            .zip(self.arrives)
            .map(|(check_in, arrives)| arrives.minutes_since(check_in))
            .filter(|late| *late > Minutes::ZERO)
    }
}
