mod bits;
mod key_entry;
mod map_level;
pub mod map_mark;
mod map_sent;
mod map_sight;
mod map_text;
mod mark_at;
mod mark_source;
mod own_layer;
mod picked_mark;

use guide::Overview;
use guide::end::End;
use guide::geo::{Bearing, Coordinate, RouteLine};
use guide::guidance::Guidance;
use guide::itinerary::Day;
use guide::leg::{Leg, LegIndex};
use guide::words::Spelling;

use bits::MapBounds;
pub use key_entry::KeyEntry;
pub use map_level::{MapLevel, MapMode, PinchWay};
pub use map_sent::{MapSent, MapUpdate};
pub use map_sight::MapSight;
pub use mark_at::MarkAt;
use mark_source::{
    Catches, DayEnds, GoalMarks, LandmarkMarks, LegEnds, LoopMarks, MarkSource, NoPathMarks, Stops,
    TripMarks,
};
use own_layer::{LegState, MarkLook, OwnFeature, OwnLayer, RouteLook, Travel, WalkerPart};
pub use picked_mark::PickedMark;

pub struct MapPlan<'view> {
    overview: &'view Overview,
    day: &'view Day,
    guidance: &'view Guidance<'view>,
    sight: MapSight<'view>,
    level: MapLevel,
    shown: LegIndex,
}

impl<'view> MapPlan<'view> {
    #[must_use]
    pub fn new(
        overview: &'view Overview,
        guidance: &'view Guidance<'view>,
        sight: MapSight<'view>,
        level: MapLevel,
    ) -> Option<MapPlan<'view>> {
        let day = guidance.day();
        let last = day.legs().len().checked_sub(1)?;
        Some(MapPlan {
            overview,
            day,
            guidance,
            sight,
            level,
            shown: sight.current(guidance).unwrap_or(LegIndex::new(last)),
        })
    }

    #[must_use]
    pub fn bounds(&self) -> Option<MapBounds> {
        let walker = self.sight.guiding().and_then(|_| self.sight.walker());
        let points: Vec<Coordinate> = self
            .framed_legs()
            .into_iter()
            .flat_map(Self::leg_points)
            .chain(walker)
            .collect();
        MapBounds::around(&points)
    }

    #[must_use]
    pub fn route(&self) -> OwnLayer<RouteLook> {
        let current = self.sight.current(self.guidance);
        self.day
            .legs()
            .iter()
            .enumerate()
            .flat_map(|(position, leg)| {
                Self::leg_lines(leg, LegState::of_progress(LegIndex::new(position), current))
            })
            .collect()
    }

    #[must_use]
    pub fn marks(&self, place: &Spelling) -> OwnLayer<MarkLook> {
        [
            LegEnds::features(self, place),
            Stops::features(self, place),
            DayEnds::features(self, place),
            GoalMarks::features(self, place),
            LoopMarks::features(self, place),
            LandmarkMarks::features(self, place),
            Catches::features(self, place),
            TripMarks::features(self, place),
            NoPathMarks::features(self, place),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[must_use]
    pub fn walker(&self, trail: &[Coordinate], heading: Option<Bearing>) -> OwnLayer<WalkerPart> {
        let walker = self.sight.walker();
        let you = walker.map(|at| OwnFeature::spot(WalkerPart::you(self.sight.you_mark()), at));
        let cone = walker
            .zip(heading)
            .map(|(at, facing)| OwnFeature::spot(WalkerPart::cone(facing), at));
        let dots = trail
            .iter()
            .filter(|_| walker.is_some())
            .map(|at| OwnFeature::spot(WalkerPart::trail(), *at));
        dots.chain(cone).chain(you).collect()
    }

    fn framed_legs(&self) -> Vec<&'view Leg> {
        let day = self.day;
        match self.level {
            MapLevel::Leg => day.leg(self.shown).into_iter().collect(),
            MapLevel::Goal => day
                .goal_holding(self.shown)
                .map_or(&[][..], |goal| day.legs_toward(goal))
                .iter()
                .collect(),
            MapLevel::Day => day.legs().iter().collect(),
        }
    }

    fn leg_lines(leg: &Leg, state: LegState) -> Vec<OwnFeature<RouteLook>> {
        let whole = OwnFeature::line(
            RouteLook::new(Travel::of_mode(leg.mode()), state),
            Self::leg_points(leg),
        );
        let unmapped = leg.no_mapped_paths().map(|stretch| {
            let line: &RouteLine = stretch.as_ref();
            OwnFeature::line(
                RouteLook::new(Travel::NoPath, state),
                line.points().to_vec(),
            )
        });
        std::iter::once(whole).chain(unmapped).collect()
    }

    fn leg_points(leg: &Leg) -> Vec<Coordinate> {
        if leg.line().points().len() > 1 {
            leg.line().points().to_vec()
        } else {
            vec![leg.point(End::Start), leg.point(End::Finish)]
        }
    }
}
