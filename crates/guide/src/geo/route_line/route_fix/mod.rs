pub mod off_route;
use crate::geo::coordinate::Coordinate;
use crate::geo::distance::Distance;
use crate::geo::share::Share;
use off_route::OffRoute;

use super::RouteLine;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouteFix {
    nearest: Coordinate,
    off_route: OffRoute,
    along: Distance,
}

impl RouteFix {
    #[must_use]
    pub fn along(self) -> Distance {
        self.along
    }

    #[must_use]
    pub fn on_segment(
        start: Coordinate,
        end: Coordinate,
        walked_before: Distance,
        position: Coordinate,
    ) -> RouteFix {
        let share = Self::share(start, end, position);
        let nearest = start.toward(end, share);
        RouteFix {
            nearest,
            off_route: OffRoute::new(nearest.distance_to(position)),
            along: walked_before + start.distance_to(nearest),
        }
    }

    #[must_use]
    pub fn on_measured_segment(
        ends: [Coordinate; RouteLine::SEGMENT_ENDS_LENGTH],
        walked: [Distance; RouteLine::SEGMENT_ENDS_LENGTH],
        position: Coordinate,
    ) -> RouteFix {
        let [start, end] = ends;
        let [before, after] = walked;
        let share = Self::share(start, end, position);
        let nearest = start.toward(end, share);
        RouteFix {
            nearest,
            off_route: OffRoute::new(nearest.distance_to(position)),
            along: before + (after - before) * share,
        }
    }

    fn share(start: Coordinate, end: Coordinate, position: Coordinate) -> Share {
        let across = Share::new(start.lat().angle().degrees().to_radians().cos());
        let east = end.lon().east_across(start, across).meters();
        let north = end.lat().north_of(start).meters();
        let span = east.mul_add(east, north * north);
        let reach = position
            .lon()
            .east_across(start, across)
            .meters()
            .mul_add(east, position.lat().north_of(start).meters() * north);
        Share::ratio(reach, span)
    }

    #[must_use]
    pub fn nearest(self) -> Coordinate {
        self.nearest
    }

    #[must_use]
    pub fn off_route(self) -> OffRoute {
        self.off_route
    }
}
