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
        arrival.zip(cutoff).map(|(arrival, cutoff)| {
            if arrival < cutoff.deadline() {
                Opening::Open
            } else {
                Opening::Closed
            }
        })
    }
}
