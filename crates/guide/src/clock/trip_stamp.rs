use std::fmt;
use std::time::{Duration, SystemTime};

use newtype::proptest::strategy::Strategy;

use crate::clock::trip_time::TripTime;
use crate::clock::trip_zone::TripZone;

#[derive(newtype::Newtype, newtype::Arbitrary, Clone, Copy, Debug, PartialEq, Eq)]
#[newtype(strategy = TripStamp::drawn())]
pub struct TripStamp(TripTime);

impl TripStamp {
    const WRITTEN: &str = "%Y-%m-%d %H:%M";
    const LATEST_SECOND: u64 = 4_102_444_800;

    fn drawn() -> impl Strategy<Value = TripTime> {
        (0..Self::LATEST_SECOND).prop_map(|second| {
            TripTime::at_instant(
                SystemTime::UNIX_EPOCH + Duration::from_secs(second),
                TripZone::default(),
            )
        })
    }
}

impl fmt::Display for TripStamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0.naive().format(Self::WRITTEN))
    }
}
