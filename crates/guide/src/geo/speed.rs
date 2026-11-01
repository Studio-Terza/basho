use std::fmt;

use crate::clock::Minutes;
use crate::geo::Distance;
use crate::geo::kilometers::Kilometers;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Speed(f64);

impl Speed {
    const MINUTES_PER_HOUR: f64 = 60.0;

    #[must_use]
    pub const fn kilometers_per_hour(speed: f64) -> Speed {
        Speed(speed)
    }

    #[must_use]
    pub fn distance_in(self, minutes: Minutes) -> Distance {
        Distance::from(Kilometers::new(
            self.0 * minutes.amount() / Self::MINUTES_PER_HOUR,
        ))
    }

    #[must_use]
    pub fn covering(distance: Distance, minutes: Minutes) -> Speed {
        Speed(
            distance.kilometers().amount()
                / (minutes.max(Minutes::new(1)).amount() / Self::MINUTES_PER_HOUR),
        )
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.1} km/h", self.0)
    }
}
