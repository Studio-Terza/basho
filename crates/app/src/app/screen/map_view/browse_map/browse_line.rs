use newtype::Newtype;
use std::fmt;

use crate::app::screen::day_name::DayName;
use crate::app::step::Spot;

#[derive(Newtype, newtype::Arbitrary, Debug)]
pub struct BrowseLine {
    spot: Spot,
}

impl fmt::Display for BrowseLine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "BROWSING · {} · step {}",
            DayName::new(self.spot.day()),
            self.spot.step()
        )
    }
}
