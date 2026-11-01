use crate::clock::Minutes;
use crate::leg::Mode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RideSlack {
    Departure,
    Arrival,
}

impl RideSlack {
    const ROAD_DELAY: Minutes = Minutes::new(10);
    const LANDING: Minutes = Minutes::new(5);

    #[must_use]
    pub fn for_mode(self, mode: Mode) -> Minutes {
        match (self, mode) {
            (RideSlack::Departure, Mode::Train | Mode::Boat | Mode::Walk) => Minutes::ZERO,
            (RideSlack::Departure, Mode::Bus | Mode::Car) => Self::ROAD_DELAY,
            (RideSlack::Arrival, Mode::Train | Mode::Boat | Mode::Walk) => Self::LANDING,
            (RideSlack::Arrival, Mode::Bus | Mode::Car) => Self::LANDING + Self::ROAD_DELAY,
        }
    }
}
