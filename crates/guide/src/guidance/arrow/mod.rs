mod aim;
mod arrow_angle;

use crate::clock::Minutes;
use crate::end::End;
use crate::geo::{Bearing, Coordinate, Distance, RouteFix};
use crate::leg::{Leg, Mode};
use crate::readout::Pointer;
use crate::times::FactTime;

pub use aim::Aim;
use arrow_angle::ArrowAngle;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arrow {
    pointer: Pointer,
    aim: Aim,
    from: Option<FactTime>,
}

impl Arrow {
    pub const LOOK_AHEAD: Distance = Distance::new(50.0);
    pub const STALE: Minutes = Minutes::new(2);

    #[must_use]
    pub fn ahead(here: Coordinate, leg: &Leg, along: Distance, next: Option<&Leg>) -> Arrow {
        let reach = along + Self::LOOK_AHEAD;
        let target = next
            .filter(|next| next.mode() == Mode::Walk && reach > leg.length())
            .and_then(|next| next.line().point_along(reach - leg.length()))
            .or_else(|| leg.line().point_along(reach))
            .unwrap_or_else(|| leg.point(End::Finish));
        Self::toward(here, target)
    }

    #[must_use]
    pub fn toward(here: Coordinate, target: Coordinate) -> Arrow {
        Arrow {
            pointer: Pointer::between(here, target),
            aim: Aim::Ahead,
            from: None,
        }
    }

    #[must_use]
    pub fn back_to_route(here: Coordinate, fix: RouteFix) -> Arrow {
        Arrow {
            pointer: Pointer::between(here, fix.nearest()),
            aim: Aim::BackToRoute,
            from: None,
        }
    }

    #[must_use]
    pub fn dated(self, from: Option<FactTime>) -> Arrow {
        Arrow { from, ..self }
    }

    #[must_use]
    pub fn fix_time(self) -> Option<FactTime> {
        self.from
    }

    #[must_use]
    pub fn pointer(self) -> Pointer {
        self.pointer
    }

    #[must_use]
    pub fn aim(self) -> Aim {
        self.aim
    }

    #[must_use]
    pub fn angle(self, heading: Option<Bearing>) -> ArrowAngle {
        ArrowAngle::new(self.pointer.bearing(), heading)
    }
}
