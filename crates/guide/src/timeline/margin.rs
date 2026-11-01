use crate::clock::{ClockTime, Minutes};
use crate::timeline::haste::Haste;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Margin {
    before: Minutes,
    haste: Haste,
}

impl Margin {
    pub const TICKET_READY: Margin = Margin {
        before: Minutes::new(10),
        haste: Haste::DoNotStop,
    };
    pub const AT_THE_STOP: Margin = Margin {
        before: Minutes::new(5),
        haste: Haste::DoNotStop,
    };
    pub const TICKET_TO_BUY: Margin = Margin {
        before: Minutes::new(15),
        haste: Haste::DoNotStop,
    };
    pub const CROSS_GATE_TRANSFER: Margin = Margin {
        before: Minutes::new(5),
        haste: Haste::DoNotStop,
    };
    pub const SAME_OPERATOR_TRANSFER: Margin = Margin {
        before: Minutes::new(3),
        haste: Haste::DoNotStop,
    };
    pub const BOAT_CHECK_IN: Margin = Margin {
        before: Minutes::new(10),
        haste: Haste::JogFromTrain,
    };

    #[must_use]
    pub fn target(self, departure: ClockTime) -> ClockTime {
        departure.later_by(-self.before)
    }

    #[must_use]
    pub fn stricter(self, other: Margin) -> Margin {
        if other.before > self.before {
            other
        } else {
            self
        }
    }

    #[must_use]
    pub fn haste(self) -> Haste {
        self.haste
    }
}
