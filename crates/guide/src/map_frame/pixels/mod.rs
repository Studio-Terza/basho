mod halving;

use newtype::Newtype;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{AddAssign, Div, Mul};

pub use halving::Halving;

#[derive(Newtype, Clone, Copy, Debug, Default, PartialEq)]
pub struct Pixels(f64);

added!(Pixels);
subtracted!(Pixels);
signed!(Pixels);

impl Pixels {
    pub const ZERO: Pixels = Pixels(0.0);

    #[must_use]
    #[inline]
    pub const fn pixels(self) -> f64 {
        self.0
    }

    #[must_use]
    #[inline]
    pub fn clamp(self, low: Pixels, high: Pixels) -> Pixels {
        Pixels(self.0.clamp(low.0, high.0))
    }

    #[must_use]
    #[inline]
    pub fn hypot(self, other: Pixels) -> Pixels {
        Pixels(self.0.hypot(other.0))
    }
}

impl Eq for Pixels {}

impl PartialOrd for Pixels {
    #[inline]
    fn partial_cmp(&self, other: &Pixels) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pixels {
    #[inline]
    fn cmp(&self, other: &Pixels) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or_else(|| self.0.total_cmp(&other.0))
    }
}

impl AddAssign for Pixels {
    #[inline]
    fn add_assign(&mut self, other: Pixels) {
        self.0 += other.0;
    }
}

impl Mul<f64> for Pixels {
    type Output = Pixels;

    #[inline]
    fn mul(self, factor: f64) -> Pixels {
        Pixels(self.0 * factor)
    }
}

impl Div<f64> for Pixels {
    type Output = Pixels;

    #[inline]
    fn div(self, divisor: f64) -> Pixels {
        Pixels(self.0 / divisor)
    }
}

impl Div for Pixels {
    type Output = f64;

    #[inline]
    fn div(self, other: Pixels) -> f64 {
        self.0 / other.0
    }
}

impl fmt::Display for Pixels {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.1}", self.0)
    }
}
