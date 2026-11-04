use std::cmp::Ordering;

use guide::clock::ClockTime;
use guide::leg::Cutoff;
use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum Opening {
    #[strum(serialize = "open when you arrive")]
    Open,
    #[strum(serialize = "closed when you arrive")]
    Closed,
}

impl Opening {
    #[must_use]
    pub fn judged(arrival: Option<ClockTime>, cutoff: Option<&Cutoff>) -> Option<Opening> {
        arrival
            .zip(cutoff)
            .map(|(arrival, cutoff)| Opening::from(arrival.cmp(&cutoff.deadline())))
    }
}

impl From<Ordering> for Opening {
    fn from(order: Ordering) -> Opening {
        match order {
            Ordering::Less => Opening::Open,
            Ordering::Equal | Ordering::Greater => Opening::Closed,
        }
    }
}
