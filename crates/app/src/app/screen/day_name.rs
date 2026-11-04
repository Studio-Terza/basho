use newtype::Newtype;
use std::fmt;

use guide::day_number::DayNumber;

#[derive(Newtype, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DayName {
    number: DayNumber,
}

impl DayName {
    const ARRIVAL: DayNumber = DayNumber::new(0);
}

impl fmt::Display for DayName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let words = Some(self.number)
            .filter(|number| *number == Self::ARRIVAL)
            .map_or_else(|| format!("Day {}", self.number), |_| "Arrival".to_owned());
        formatter.write_str(&words)
    }
}
