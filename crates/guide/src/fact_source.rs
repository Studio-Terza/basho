use std::fmt;

use serde::Deserialize;

use crate::clock_digits::ClockDigits;
use crate::words::Label;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactSource {
    #[serde(rename = "train")]
    TrainTimes,
    #[serde(rename = "bus")]
    BusTimes,
    #[serde(rename = "boat")]
    BoatTimes,
    #[serde(rename = "tour")]
    TourTimes,
    Booked,
    Closes,
    Entry,
    LastOrder,
    CheckIn,
    Lands,
    #[serde(skip)]
    AtSunset,
    #[serde(skip)]
    Gps,
    Unknown,
}

impl FactSource {
    #[must_use]
    pub fn word(self) -> Label {
        match self {
            FactSource::TrainTimes => Label::new("train"),
            FactSource::BusTimes => Label::new("bus"),
            FactSource::BoatTimes => Label::new("boat"),
            FactSource::TourTimes => Label::new("tour"),
            FactSource::Booked => Label::new("booked"),
            FactSource::Closes => Label::new("closes"),
            FactSource::Entry => Label::new("entry closes"),
            FactSource::LastOrder => Label::new("last order"),
            FactSource::CheckIn => Label::new("check in by"),
            FactSource::Lands => Label::new("lands"),
            FactSource::AtSunset => Label::new("sunset"),
            FactSource::Gps => Label::new("GPS"),
            FactSource::Unknown => Label::new("source unknown"),
        }
    }

    pub(super) fn beside(
        self,
        digits: ClockDigits,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            FactSource::Closes
            | FactSource::Entry
            | FactSource::LastOrder
            | FactSource::CheckIn
            | FactSource::Lands
            | FactSource::AtSunset => write!(formatter, "{} {digits}", self.word()),
            FactSource::TrainTimes
            | FactSource::BusTimes
            | FactSource::BoatTimes
            | FactSource::TourTimes
            | FactSource::Booked
            | FactSource::Gps
            | FactSource::Unknown => write!(formatter, "{digits} {}", self.word()),
        }
    }
}
