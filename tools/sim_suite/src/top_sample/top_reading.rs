use std::num::ParseFloatError;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct TopReading(f64);

impl Add for TopReading {
    type Output = TopReading;

    fn add(self, other: TopReading) -> TopReading {
        TopReading(self.0 + other.0)
    }
}

impl FromStr for TopReading {
    type Err = ParseFloatError;

    fn from_str(cell: &str) -> Result<TopReading, ParseFloatError> {
        cell.parse().map(TopReading)
    }
}
