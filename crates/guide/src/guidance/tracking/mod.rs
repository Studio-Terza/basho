mod floored;
mod pin;
mod placing;
mod stage;

use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

pub use pin::Pin;
pub use placing::Placing;
pub use stage::Stage;

use crate::guidance::PositionSource;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tracking {
    #[default]
    Automatic,
    Pinned(Pin),
}

impl Tracking {
    const SET: &str = "set";
    const BASE: &str = "base";

    #[must_use]
    pub fn pinned(floor: Placing, baseline: Placing) -> Tracking {
        Tracking::Pinned(Pin::new(floor, baseline))
    }

    #[must_use]
    pub fn observing(self, seen: Placing, source: PositionSource) -> Tracking {
        match self {
            Tracking::Automatic => Tracking::Automatic,
            Tracking::Pinned(pin) => Tracking::Pinned(pin.observing(seen, source)),
        }
    }

    #[must_use]
    pub fn holding(self, seen: Placing, source: PositionSource) -> Option<Placing> {
        match self {
            Tracking::Automatic => None,
            Tracking::Pinned(pin) => pin.holding(seen, source),
        }
    }

    #[must_use]
    pub fn floor(self) -> Option<Placing> {
        match self {
            Tracking::Automatic => None,
            Tracking::Pinned(pin) => Some(pin.floor()),
        }
    }
}

impl FromStr for Tracking {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Tracking, Infallible> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let after = |label: &str| {
            words
                .iter()
                .position(|word| *word == label)
                .and_then(|at| words.get(at + 1..))
                .and_then(|rest| rest.join(" ").parse::<Placing>().ok())
        };
        Ok(after(Tracking::SET)
            .zip(after(Tracking::BASE))
            .map_or(Tracking::Automatic, |(floor, baseline)| {
                Tracking::pinned(floor, baseline)
            }))
    }
}

impl fmt::Display for Tracking {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tracking::Automatic => Ok(()),
            Tracking::Pinned(pin) => write!(formatter, " {pin}"),
        }
    }
}
