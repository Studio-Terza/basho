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
            RideSlack::Departure => Self::after(Minutes::ZERO, mode),
            RideSlack::Arrival => Self::after(Self::LANDING, mode),
        }
    }

    fn after(base: Minutes, mode: Mode) -> Minutes {
        [Mode::Bus, Mode::Car]
            .contains(&mode)
            .then_some(Self::ROAD_DELAY)
            .map_or(base, |delay| base + delay)
    }
}
