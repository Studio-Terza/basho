use guide::day_number::DayNumber;

use crate::app::step::Spot;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Now,
    Browsing(Spot),
}

impl Focus {
    #[must_use]
    pub fn toward(spot: Spot, now: Option<Spot>) -> Focus {
        if now == Some(spot) {
            Focus::Now
        } else {
            Focus::Browsing(spot)
        }
    }

    #[must_use]
    pub fn spot(self) -> Option<Spot> {
        match self {
            Focus::Now => None,
            Focus::Browsing(spot) => Some(spot),
        }
    }

    pub fn day(self) -> Option<DayNumber> {
        self.spot().map(Spot::day)
    }
}
