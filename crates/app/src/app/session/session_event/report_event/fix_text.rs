use std::fmt;

use guide::clock::trip_stamp::TripStamp;
use guide::geo::Distance;
use guide::guidance::Sighting;

#[derive(Clone, Copy, Debug)]
pub struct FixText {
    sighting: Sighting,
    spread: Option<Distance>,
}

impl FixText {
    #[must_use]
    pub fn new(sighting: Sighting, spread: Option<Distance>) -> FixText {
        FixText { sighting, spread }
    }
}

impl fmt::Display for FixText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let at = self.sighting.place();
        write!(
            formatter,
            "{:.5}, {:.5}",
            at.lat().angle().degrees(),
            at.lon().angle().degrees(),
        )?;
        self.spread
            .map_or(Ok(()), |spread| write!(formatter, " ± {spread}"))?;
        self.sighting.last_fix().map_or(Ok(()), |since| {
            write!(formatter, ", last fix {}", TripStamp::new(since))
        })
    }
}
