use crate::day_number::DayNumber;

use crate::leg::LegIndex;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    day: Option<DayNumber>,
    leg: Option<LegIndex>,
    message: String,
}

impl Finding {
    #[must_use]
    pub fn whole(message: impl fmt::Display) -> Finding {
        Finding {
            day: None,
            leg: None,
            message: message.to_string(),
        }
    }

    #[must_use]
    pub fn on_day(day: DayNumber, message: impl fmt::Display) -> Finding {
        Finding {
            day: Some(day),
            leg: None,
            message: message.to_string(),
        }
    }

    #[must_use]
    pub fn on_leg(day: DayNumber, leg: LegIndex, message: impl fmt::Display) -> Finding {
        Finding {
            day: Some(day),
            leg: Some(leg),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.day {
            Some(day) => self.located(formatter, day),
            None => formatter.write_str(&self.message),
        }
    }
}

impl Finding {
    fn located(&self, formatter: &mut fmt::Formatter<'_>, day: DayNumber) -> fmt::Result {
        match self.leg {
            Some(leg) => write!(formatter, "day {day} leg {leg}: {}", self.message),
            None => write!(formatter, "day {day}: {}", self.message),
        }
    }
}
