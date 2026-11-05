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
        match self {
            RideSlack::Departure => Self::delay(mode),
            RideSlack::Arrival => Self::landed(Self::delay(mode)),
        }
    }

    fn delay(mode: Mode) -> Minutes {
        match mode {
            Mode::Train | Mode::Boat | Mode::Walk => Minutes::ZERO,
            Mode::Bus | Mode::Car => Self::ROAD_DELAY,
        }
    }

    fn landed(delay: Minutes) -> Minutes {
        Self::LANDING + delay
    }
}
