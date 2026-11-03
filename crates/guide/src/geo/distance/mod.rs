mod ground;

use newtype::Newtype;
use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Div, Mul};

use serde::{Deserialize, Deserializer};

use crate::geo::not_finite::NotFinite;

use crate::geo::kilometers::Kilometers;
use crate::geo::share::Share;

pub use ground::Ground;

#[derive(Newtype, Clone, Copy, Debug, PartialEq, Default, Deserialize)]
#[serde(try_from = "f64")]
pub struct Distance(f64);

subtracted!(Distance);
signed!(Distance);

impl TryFrom<f64> for Distance {
    type Error = NotFinite;

    fn try_from(number: f64) -> Result<Distance, NotFinite> {
        NotFinite::ensure_finite(number).map(Distance)
    }
}

impl Distance {
    const METERS_PER_KILOMETER: f64 = 1000.0;
    const ROUNDING_METERS: f64 = 10.0;

    pub fn deserialize_kilometers<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Distance, D::Error> {
        Kilometers::deserialize(deserializer).map(Distance::from)
    }

    #[must_use]
    pub fn meters(self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn kilometers(self) -> Kilometers {
        Kilometers::new(self.0 / Self::METERS_PER_KILOMETER)
    }
}

impl From<Kilometers> for Distance {
    fn from(kilometers: Kilometers) -> Distance {
        Distance(kilometers.amount() * Distance::METERS_PER_KILOMETER)
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Distance) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Distance) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or_else(|| self.0.total_cmp(&other.0))
    }
}

impl Mul<f64> for Distance {
    type Output = Distance;

    fn mul(self, factor: f64) -> Distance {
        Distance(self.0 * factor)
    }
}

impl Mul<Share> for Distance {
    type Output = Distance;

    fn mul(self, share: Share) -> Distance {
        Distance(self.0 * share.fraction())
    }
}

impl Div for Distance {
    type Output = f64;

    fn div(self, other: Distance) -> f64 {
        self.0 / other.0
    }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, other: Distance) -> Distance {
        Distance::new(self.0 + other.0)
    }
}

impl Sum for Distance {
    fn sum<I: Iterator<Item = Distance>>(distances: I) -> Distance {
        distances.fold(Distance::default(), Add::add)
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < Self::METERS_PER_KILOMETER {
            let rounded = (self.0 / Self::ROUNDING_METERS).round() * Self::ROUNDING_METERS;
            write!(formatter, "{rounded:.0} m")
        } else {
            write!(formatter, "{:.1} km", self.kilometers().amount())
        }
    }
}
