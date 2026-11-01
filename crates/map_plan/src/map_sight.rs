use guide::geo::{Coordinate, Distance};
use guide::guidance::{Guidance, Phase};
use guide::leg::LegIndex;

use crate::map_mark::MapMark;

#[derive(Clone, Copy)]
pub struct MapSight<'view> {
    walker: &'view Guidance<'view>,
    browsed: Option<LegIndex>,
}

impl<'view> MapSight<'view> {
    #[must_use]
    pub fn live(walker: &'view Guidance<'view>) -> MapSight<'view> {
        MapSight {
            walker,
            browsed: None,
        }
    }

    #[must_use]
    pub fn browsing(walker: &'view Guidance<'view>, leg: LegIndex) -> MapSight<'view> {
        MapSight {
            walker,
            browsed: Some(leg),
        }
    }

    #[must_use]
    pub fn current(&self, shown: &Guidance<'_>) -> Option<LegIndex> {
        self.browsed
            .or_else(|| (shown.phase() != Phase::Done).then(|| shown.leg_index()))
    }

    #[must_use]
    pub fn walker(&self) -> Option<Coordinate> {
        self.walker
            .marker()
            .filter(|_| !matches!(self.walker.phase(), Phase::Preview | Phase::Done))
    }

    pub fn passed(&self, current: Option<LegIndex>) -> Option<Distance> {
        self.guiding()
            .filter(|walker| {
                current == Some(walker.leg_index()) && walker.phase() != Phase::Preview
            })
            .map(Guidance::along)
    }

    #[must_use]
    pub fn you_mark(&self) -> MapMark {
        MapMark::you(self.walker.source())
    }

    #[must_use]
    pub fn guiding(&self) -> Option<&'view Guidance<'view>> {
        self.browsed.is_none().then_some(self.walker)
    }
}
