use crate::end::End;
use crate::geo::{Coordinate, Distance, Share};
use crate::leg::Leg;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Marker {
    Astray(Coordinate),
    LastFix(Coordinate),
    OnRoute(Coordinate),
}

impl Marker {
    #[must_use]
    pub fn on_route(leg: &Leg, along: Distance) -> Option<Marker> {
        Some(leg.line())
            .filter(|line| line.points().len() > 1)
            .map_or_else(
                || {
                    Some(
                        leg.point(End::Start)
                            .toward(leg.point(End::Finish), Share::ratio(along, leg.length())),
                    )
                },
                |line| line.point_along(along),
            )
            .map(Marker::OnRoute)
    }

    #[must_use]
    pub fn location(self) -> Coordinate {
        match self {
            Marker::Astray(at) | Marker::LastFix(at) | Marker::OnRoute(at) => at,
        }
    }
}
