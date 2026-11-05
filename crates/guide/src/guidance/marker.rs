use crate::end::End;
use crate::geo::{Coordinate, Distance, Share};
use crate::leg::Leg;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Marker {
    Astray(Coordinate),
    LastFix(Coordinate),
    OnRoute(Coordinate),
}

impl Marker {
    #[must_use]
    pub fn on_route(leg: &Leg, along: Distance) -> Option<Marker> {
        Self::routed(leg, along).map(Marker::OnRoute)
    }

    fn routed(leg: &Leg, along: Distance) -> Option<Coordinate> {
        match leg.line().points().len().cmp(&1) {
            Ordering::Greater => leg.line().point_along(along),
            Ordering::Less | Ordering::Equal => Some(
                leg.point(End::Start)
                    .toward(leg.point(End::Finish), Share::ratio(along, leg.length())),
            ),
        }
    }

    #[must_use]
    pub fn location(self) -> Coordinate {
        match self {
            Marker::Astray(at) | Marker::LastFix(at) | Marker::OnRoute(at) => at,
        }
    }
}
