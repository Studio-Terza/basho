use newtype::Newtype;
use std::fmt;
use std::iter::Sum;
use std::ops::Add;

use serde::Deserialize;

use crate::words::Label;

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

    fn sign(self) -> Label {
        Some(self.0)
            .filter(|count| *count < 0)
            .map_or(Label::new(""), |_| Label::new("-"))
    }

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

#[derive(Clone, Copy)]
enum Span {
    Minutes,
    Hours,
    Both,
}

impl From<Minutes> for Span {
    fn from(minutes: Minutes) -> Span {
        Some(minutes.0.abs() / Minutes::PER_HOUR)
            .filter(|hours| *hours == 0)
            .map(|_| Span::Minutes)
            .or_else(|| {
                Some(minutes.0.abs() % Minutes::PER_HOUR)
                    .filter(|rest| *rest == 0)
                    .map(|_| Span::Hours)
            })
            .unwrap_or(Span::Both)
    }
}

impl Span {
    fn written(self, formatter: &mut fmt::Formatter<'_>, minutes: Minutes) -> fmt::Result {
        match self {
            Span::Minutes => write!(
                formatter,
                "{}{} min",
                minutes.sign(),
                minutes.0.abs() % Minutes::PER_HOUR
            ),
            Span::Hours => write!(
                formatter,
                "{}{} h",
                minutes.sign(),
                minutes.0.abs() / Minutes::PER_HOUR
            ),
            Span::Both => write!(
                formatter,
                "{}{} h {} min",
                minutes.sign(),
                minutes.0.abs() / Minutes::PER_HOUR,
                minutes.0.abs() % Minutes::PER_HOUR
            ),
        }
    }
}

impl fmt::Display for Minutes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Span::from(*self).written(formatter, *self)
    }
}
