use std::fmt;

use crate::clock::trip_time::TripTime;

#[derive(newtype::Newtype, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TripStamp(TripTime);

impl TripStamp {
    const WRITTEN: &str = "%Y-%m-%d %H:%M";
}

impl fmt::Display for TripStamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0.naive().format(Self::WRITTEN))
    }
}
