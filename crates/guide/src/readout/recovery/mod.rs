pub mod catch_words;
pub mod day_catch;
mod kana_file;

use crate::geo::Coordinate;
use crate::itinerary::Day;
use crate::leg::LegIndex;
use crate::readout::pointer::Pointer;
use crate::times::FactTime;

pub use day_catch::{CatchSight, DayCatch};
pub use kana_file::KanaFile;

#[derive(Clone, Debug)]
pub struct Recovery<'catch> {
    route: Option<Pointer>,
    toward: Vec<CatchSight<'catch>>,
    from: Option<FactTime>,
}

impl<'catch> Recovery<'catch> {
    #[must_use]
    pub fn near(
        day: &'catch Day,
        leg: LegIndex,
        here: Coordinate,
        from: Option<FactTime>,
    ) -> Recovery<'catch> {
        let route = day
            .leg(leg)
            .and_then(|walk| walk.line().fix(here))
            .map(|fix| Pointer::between(here, fix.nearest()));
        Recovery {
            route,
            toward: day
                .catch()
                .map_or_else(Vec::new, |catch| catch.toward(here, leg, route)),
            from,
        }
    }

    #[must_use]
    pub fn route(&self) -> Option<Pointer> {
        self.route
    }

    #[must_use]
    pub fn toward(&self) -> &[CatchSight<'catch>] {
        &self.toward
    }

    #[must_use]
    pub fn fix_time(&self) -> Option<FactTime> {
        self.from
    }
}
