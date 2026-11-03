use std::fmt;

use crate::clock::Minutes;
use crate::geo::Distance;
use crate::leg::Mode;
use crate::readout::Emphasis;
use crate::times::ListedTime;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    OnRoute,
    OffRoute(Distance),
    NoGps(Option<Minutes>),
    Waiting { until: ListedTime },
    Riding(Mode),
}

impl Status {
    #[must_use]
    pub fn leaves(self) -> Option<ListedTime> {
        match self {
            Status::Waiting { until } => Some(until),
            Status::OnRoute | Status::OffRoute(_) | Status::NoGps(_) | Status::Riding(_) => None,
        }
    }

    #[must_use]
    pub fn emphasis(self) -> Emphasis {
        match self {
            Status::OffRoute(_) => Emphasis::Inverted,
            Status::OnRoute | Status::NoGps(_) | Status::Waiting { .. } | Status::Riding(_) => {
                Emphasis::Plain
            }
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::OnRoute => write!(formatter, "ON ROUTE"),
            Status::OffRoute(distance) => write!(formatter, "OFF ROUTE {distance}"),
            Status::NoGps(Some(minutes)) => write!(formatter, "NO GPS for {minutes}"),
            Status::NoGps(None) => write!(formatter, "NO GPS"),
            Status::Waiting { until } => write!(formatter, "WAITING · leaves {until}"),
            Status::Riding(Mode::Train) => write!(formatter, "ON THE TRAIN"),
            Status::Riding(Mode::Bus) => write!(formatter, "ON THE BUS"),
            Status::Riding(Mode::Boat) => write!(formatter, "ON THE BOAT"),
            Status::Riding(Mode::Car) => write!(formatter, "IN THE CAR"),
            Status::Riding(Mode::Walk) => write!(formatter, "WALKING"),
        }
    }
}
