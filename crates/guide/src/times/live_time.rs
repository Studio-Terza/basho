use std::fmt;

use crate::clock::ClockTime;
use crate::clock_digits::ClockDigits;
use crate::times::basis::Basis;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiveTime {
    clock: ClockTime,
    basis: Basis,
}

impl LiveTime {
    const APPROXIMATE: &str = "≈";

    #[must_use]
    pub fn new(clock: ClockTime, basis: Basis) -> LiveTime {
        LiveTime { clock, basis }
    }
}

impl From<LiveTime> for ClockTime {
    fn from(time: LiveTime) -> ClockTime {
        time.clock
    }
}

impl fmt::Display for LiveTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} · {}",
            Self::APPROXIMATE,
            ClockDigits::new(self.clock),
            self.basis
        )
    }
}
