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
        Label::new(match self {
            FactSource::TrainTimes => "train",
            FactSource::BusTimes => "bus",
            FactSource::BoatTimes => "boat",
            FactSource::TourTimes => "tour",
            FactSource::Booked => "booked",
            FactSource::Closes => "closes",
            FactSource::Entry => "entry closes",
            FactSource::LastOrder => "last order",
            FactSource::CheckIn => "check in by",
            FactSource::Lands => "lands",
            FactSource::AtSunset => "sunset",
            FactSource::Gps => "GPS",
            FactSource::Unknown => "source unknown",
        })
    }

    pub(super) fn beside(
        self,
        digits: ClockDigits,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let word = self.word();
        match self {
            FactSource::Closes
            | FactSource::Entry
            | FactSource::LastOrder
            | FactSource::CheckIn
            | FactSource::Lands
            | FactSource::AtSunset => write!(formatter, "{word} {digits}"),
            FactSource::TrainTimes
            | FactSource::BusTimes
            | FactSource::BoatTimes
            | FactSource::TourTimes
            | FactSource::Booked
            | FactSource::Gps
            | FactSource::Unknown => write!(formatter, "{digits} {word}"),
        }
    }
}
