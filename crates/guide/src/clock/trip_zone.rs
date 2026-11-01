use crate::words::Written;
use std::str::FromStr;

use chrono::{FixedOffset, TimeDelta};
use serde::Deserialize;

use crate::clock::clock_error::ClockError;
use crate::clock::minutes::Minutes;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct TripZone(FixedOffset);

impl TripZone {
    #[must_use]
    pub fn ahead_of_utc(self) -> Minutes {
        Minutes::new(TimeDelta::seconds(i64::from(self.0.local_minus_utc())).num_minutes())
    }
}

impl Default for TripZone {
    fn default() -> TripZone {
        TripZone(FixedOffset::east_opt(0).expect("zero is a valid offset"))
    }
}

impl From<FixedOffset> for TripZone {
    fn from(offset: FixedOffset) -> TripZone {
        TripZone(offset)
    }
}

impl From<TripZone> for FixedOffset {
    fn from(zone: TripZone) -> FixedOffset {
        zone.0
    }
}

impl FromStr for TripZone {
    type Err = ClockError;

    fn from_str(text: &str) -> Result<TripZone, ClockError> {
        text.parse()
            .map(TripZone)
            .map_err(|_| ClockError::Zone(Written::new(text.to_owned())))
    }
}

impl TryFrom<String> for TripZone {
    type Error = ClockError;

    fn try_from(text: String) -> Result<TripZone, ClockError> {
        text.parse()
    }
}
