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
        let routed = if leg.line().points().len() > 1 {
            leg.line().point_along(along)
        } else {
            Some(
                leg.point(End::Start)
                    .toward(leg.point(End::Finish), Share::ratio(along, leg.length())),
            )
        };
        routed.map(Marker::OnRoute)
    }

    #[must_use]
    pub fn location(self) -> Coordinate {
        match self {
            Marker::Astray(at) | Marker::LastFix(at) | Marker::OnRoute(at) => at,
        }
    }
}
