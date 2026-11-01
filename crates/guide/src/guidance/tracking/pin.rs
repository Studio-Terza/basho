use std::fmt;

use super::placing::Placing;
use crate::guidance::PositionSource;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pin {
    floor: Placing,
    baseline: Placing,
}

impl Pin {
    #[must_use]
    pub fn new(floor: Placing, baseline: Placing) -> Pin {
        Pin { floor, baseline }
    }

    #[must_use]
    pub fn floor(self) -> Placing {
        self.floor
    }

    #[must_use]
    pub fn observing(self, seen: Placing, source: PositionSource) -> Pin {
        Pin {
            floor: self.floor.max(seen),
            baseline: source
                .gps()
                .map_or(self.baseline, |_| self.baseline.max(seen)),
        }
    }

    #[must_use]
    pub fn holding(self, seen: Placing, source: PositionSource) -> Option<Placing> {
        source
            .gps()
            .filter(|_| seen <= self.baseline)
            .map(|_| self.floor)
            .or_else(|| (seen < self.floor).then_some(self.floor))
    }
}

impl fmt::Display for Pin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "set {} base {}", self.floor, self.baseline)
    }
}
