use newtype::Newtype;
use std::fmt;

use guide::day_number::DayNumber;

#[derive(Newtype, newtype::Arbitrary, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DayName {
    number: DayNumber,
}

impl DayName {
    const ARRIVAL: DayNumber = DayNumber::new(0);
}

impl fmt::Display for DayName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.number == Self::ARRIVAL {
            formatter.write_str("Arrival")
        } else {
            write!(formatter, "Day {}", self.number)
        }
    }
}
