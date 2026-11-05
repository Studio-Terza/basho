use crate::geo::{Coordinate, RouteFix};
use crate::guidance::Guidance;
use crate::guidance::arrow::Arrow;
use crate::guidance::nearness::Nearness;
use crate::leg::{Leg, Mode};
use newtype::Counter;

impl Guidance<'_> {
    pub(super) fn aimed(&self, leg: &Leg, here: Coordinate) -> Option<Arrow> {
        match leg.mode() {
            Mode::Walk => Some(self.walking_arrow(
                leg,
                self.off_route.and_then(|_| leg.line().fix(here)),
                here,
            )),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => {
                self.platform_arrow(leg.boarding(), here)
            }
        }
    }

    fn walking_arrow(&self, leg: &Leg, fix: Option<RouteFix>, here: Coordinate) -> Arrow {
        fix.map_or_else(
            || Arrow::ahead(here, leg, self.along, self.day.leg(self.leg.incremented())),
            |fix| Arrow::back_to_route(here, fix),
        )
    }

    fn platform_arrow(&self, boarding: Option<Coordinate>, here: Coordinate) -> Option<Arrow> {
        self.phase
            .at_platform()
            .and(boarding)
            .filter(|boarding| here.distance_to(*boarding) > Nearness::GpsError.distance())
            .map(|boarding| Arrow::toward(here, boarding))
    }
}
