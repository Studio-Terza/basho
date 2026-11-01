use newtype::Newtype;
use std::fmt;
use std::iter::Sum;
use std::ops::Add;

use serde::Deserialize;

#[derive(
    Newtype, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize,
)]
pub struct Minutes(i64);

added!(Minutes);
subtracted!(Minutes);
signed!(Minutes);

impl Minutes {
    pub const ZERO: Minutes = Minutes(0);
    const PER_HOUR: i64 = 60;

    #[must_use]
    pub const fn count(self) -> i64 {
        self.0
    }

    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        reason = "only a finite count inside the i64 range is cast"
    )]
    pub const fn rounded(minutes: f64) -> Option<Minutes> {
        let whole = minutes.round_ties_even();
        if whole.is_finite() && whole.abs() < i64::MAX as f64 {
            Some(Minutes(whole as i64))
        } else {
            None
        }
    }

    #[must_use]
    pub fn amount(self) -> f64 {
        f64::from(i32::try_from(self.0).unwrap_or(i32::MAX))
    }
}

impl Sum for Minutes {
    fn sum<I: Iterator<Item = Minutes>>(minutes: I) -> Minutes {
        minutes.fold(Minutes::ZERO, Add::add)
    }
}

impl fmt::Display for Minutes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let hours = self.0.abs() / Self::PER_HOUR;
        let rest = self.0.abs() % Self::PER_HOUR;
        match (hours, rest) {
            (0, _) => write!(formatter, "{sign}{rest} min"),
            (_, 0) => write!(formatter, "{sign}{hours} h"),
            _ => write!(formatter, "{sign}{hours} h {rest} min"),
        }
    }
}
