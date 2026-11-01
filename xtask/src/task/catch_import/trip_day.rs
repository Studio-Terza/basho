use std::fmt;
use std::path::{Path, PathBuf};

use guide::itinerary::DayNumber;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TripDay(DayNumber);

impl TripDay {
    pub fn of_day(day: DayNumber) -> TripDay {
        TripDay(day)
    }

    pub fn file(self, directory: &Path, extension: impl fmt::Display) -> PathBuf {
        directory.join(format!("{self}.{extension}"))
    }
}

impl fmt::Display for TripDay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "day{:02}", self.0.number())
    }
}
