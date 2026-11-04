use crate::day_number::DayNumber;

use crate::leg::LegIndex;
use crate::words::Spelling;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    day: Option<DayNumber>,
    leg: Option<LegIndex>,
    message: Spelling,
}

impl Finding {
    #[must_use]
    pub fn whole(message: impl fmt::Display) -> Finding {
        Finding {
            day: None,
            leg: None,
            message: Spelling::new(message.to_string()),
        }
    }

    #[must_use]
    pub fn on_day(day: DayNumber, message: impl fmt::Display) -> Finding {
        Finding {
            day: Some(day),
            leg: None,
            message: Spelling::new(message.to_string()),
        }
    }

    #[must_use]
    pub fn on_leg(day: DayNumber, leg: LegIndex, message: impl fmt::Display) -> Finding {
        Finding {
            day: Some(day),
            leg: Some(leg),
            message: Spelling::new(message.to_string()),
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.day, self.leg) {
            (Some(day), Some(leg)) => write!(formatter, "day {day} leg {leg}: {}", self.message),
            (Some(day), None) => write!(formatter, "day {day}: {}", self.message),
            _ => formatter.write_str(&self.message),
        }
    }
}
