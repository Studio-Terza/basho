use std::fmt;

use crate::app::step::Spot;

#[derive(Clone, Copy, Debug)]
pub enum StepText {
    Current(Spot),
    Browsing(Spot),
}

impl fmt::Display for StepText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepText::Current(spot) => {
                write!(formatter, "day {} step {}", spot.day(), spot.step())
            }
            StepText::Browsing(spot) => {
                write!(
                    formatter,
                    "day {} step {} browsing",
                    spot.day(),
                    spot.step()
                )
            }
        }
    }
}
