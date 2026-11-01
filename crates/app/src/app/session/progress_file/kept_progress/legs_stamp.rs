use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

use guide::itinerary::Day;

use guide::end::End;
use newtype::Newtype;

#[derive(Newtype, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegsStamp(u64);

impl LegsStamp {
    const HEX: u32 = 16;

    #[must_use]
    pub fn taken_from(day: &Day) -> LegsStamp {
        let mut hasher = DefaultHasher::new();
        day.legs().iter().for_each(|leg| {
            leg.mode().label().hash(&mut hasher);
            leg.time(End::Start).since_midnight().hash(&mut hasher);
            leg.time(End::Finish).since_midnight().hash(&mut hasher);
            [leg.point(End::Start), leg.point(End::Finish)]
                .iter()
                .flat_map(|point| {
                    [
                        point.lat().angle().degrees().to_radians(),
                        point.lon().angle().degrees().to_radians(),
                    ]
                })
                .for_each(|angle| angle.to_bits().hash(&mut hasher));
        });
        LegsStamp(hasher.finish())
    }
}

impl fmt::Display for LegsStamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:016x}", self.0)
    }
}

impl FromStr for LegsStamp {
    type Err = ParseIntError;

    fn from_str(text: &str) -> Result<LegsStamp, ParseIntError> {
        u64::from_str_radix(text, LegsStamp::HEX).map(LegsStamp)
    }
}
