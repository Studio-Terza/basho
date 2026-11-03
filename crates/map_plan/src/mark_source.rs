use guide::LoopStop;
use guide::content::Content;
use guide::day_number::DayNumber;
use guide::end::End;
use guide::evidence::Evidence;
use guide::geo::{Coordinate, RouteLine, Share};
use guide::guidance::Guidance;
use guide::itinerary::Caption;
use guide::leg::{Leg, LegIndex, StopKind};
use guide::reading::Reading;
use guide::ruby_text::RubyText;
use guide::times::PlanTime;
use guide::words::Spelling;
use newtype::Counter;

use crate::map_mark::MapMark;
use crate::map_text::MapText;
use crate::own_layer::{MarkLook, OwnFeature};
use crate::{MapLevel, MapPlan};

type MarkFeature = OwnFeature<MarkLook>;

pub trait MarkSource {
    fn features(plan: &MapPlan<'_>, place: &Spelling) -> Vec<MarkFeature>;
}

pub struct LegEnds;

impl MarkSource for LegEnds {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        let legs = plan.framed_legs();
        legs.iter()
            .take(legs.len().saturating_sub(1))
            .filter(|_| plan.level != MapLevel::Day)
            .map(|leg| OwnFeature::spot(MarkLook::plain(MapMark::LegEnd), leg.point(End::Finish)))
            .collect()
    }
}

pub struct Stops;

impl MarkSource for Stops {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        plan.framed_legs()
            .into_iter()
            .filter(|_| plan.level != MapLevel::Day)
            .flat_map(|leg| leg.along().iter())
            .filter(|stop| stop.kind() != StopKind::Landmark)
            .map(|stop| {
                let name = MapText::plain(AsRef::<RubyText>::as_ref(stop).text(Reading::Spoken));
                let look = MarkLook::named(MapMark::stop(stop.kind()), name);
                OwnFeature::spot(look, Coordinate::from(stop))
            })
            .collect()
    }
}

pub struct DayEnds;

impl MarkSource for DayEnds {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        let legs = plan.day.legs();
        let current = plan.sight.current(plan.guidance);
        let start = legs
            .first()
            .map(|leg| OwnFeature::spot(MarkLook::plain(MapMark::DayStart), leg.point(End::Start)));
        let night = legs
            .last()
            .filter(|_| current.map(Counter::incremented).map(usize::from) != Some(legs.len()))
            .map(|leg| OwnFeature::spot(MarkLook::plain(MapMark::Night), leg.point(End::Finish)));
        start.into_iter().chain(night).collect()
    }
}

pub struct GoalMarks;

impl GoalMarks {
    fn named_at(plan: &MapPlan<'_>, index: LegIndex, name: MapText) -> Option<MarkFeature> {
        plan.day.leg(index).map(|leg| {
            OwnFeature::spot(MarkLook::named(MapMark::Goal, name), leg.point(End::Finish))
        })
    }
}

impl MarkSource for GoalMarks {
    fn features(plan: &MapPlan<'_>, place: &Spelling) -> Vec<MarkFeature> {
        if plan.level == MapLevel::Leg {
            Self::named_at(plan, plan.shown, MapText::plain(place))
                .into_iter()
                .collect()
        } else {
            plan.day
                .goals()
                .iter()
                .filter(|goal| plan.level != MapLevel::Goal || goal.purpose().stops().is_empty())
                .filter_map(|goal| {
                    let name = MapText::marked(
                        goal[Caption::Name].text(Reading::Spoken),
                        plan.day.goal_evidence(goal).and_then(Evidence::mark),
                    );
                    Self::named_at(plan, goal.leg(End::Finish), name)
                })
                .collect()
        }
    }
}

pub struct LoopMarks;

impl LoopMarks {
    fn numbered(plan: &MapPlan<'_>, stop: &LoopStop<'_>) -> Option<MarkFeature> {
        plan.day.leg(*AsRef::<LegIndex>::as_ref(stop)).map(|leg| {
            let name = AsRef::<Content>::as_ref(stop).text(Reading::Spoken);
            let label = MapText::loop_stop(stop.number(), *AsRef::<PlanTime>::as_ref(stop), name);
            OwnFeature::spot(
                MarkLook::named(MapMark::LoopStop, label),
                leg.point(End::Finish),
            )
        })
    }
}

impl MarkSource for LoopMarks {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        plan.day
            .goal_holding(plan.shown)
            .filter(|_| plan.level == MapLevel::Goal)
            .into_iter()
            .flat_map(|goal| goal.purpose().stops())
            .filter_map(|stop| Self::numbered(plan, &stop))
            .collect()
    }
}

pub struct LandmarkMarks;

impl MarkSource for LandmarkMarks {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        let passed = plan.sight.passed(Some(plan.shown));
        plan.day
            .leg(plan.shown)
            .filter(|_| plan.level == MapLevel::Leg)
            .into_iter()
            .flat_map(|leg| leg.along().iter())
            .filter(|stop| stop.kind() == StopKind::Landmark)
            .map(|stop| {
                let reached = passed.filter(|along| stop.distance_along() <= *along);
                let label = MapText::landmark(
                    reached.map_or("", |_| "✓ "),
                    stop.expected().filter(|_| reached.is_none()),
                    AsRef::<RubyText>::as_ref(stop).text(Reading::Spoken),
                );
                OwnFeature::spot(
                    MarkLook::named(MapMark::Landmark, label),
                    Coordinate::from(stop),
                )
            })
            .collect()
    }
}

pub struct Catches;

impl Catches {
    fn around(guidance: &Guidance<'_>, here: Coordinate) -> Option<Vec<MarkFeature>> {
        Some(
            guidance
                .recovery()?
                .toward()
                .iter()
                .map(|sight| {
                    let pointer = sight.pointer();
                    let turn = pointer.bearing().degrees().to_radians();
                    let at = here.offset_by(
                        pointer.distance() * turn.sin(),
                        pointer.distance() * turn.cos(),
                    );
                    let name = MapText::plain(sight.name().text(Reading::Spoken));
                    OwnFeature::spot(MarkLook::named(MapMark::Catch, name), at)
                })
                .collect(),
        )
    }
}

impl MarkSource for Catches {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        plan.sight
            .guiding()
            .and_then(|guidance| Self::around(guidance, guidance.marker()?))
            .unwrap_or_default()
    }
}

pub struct TripMarks;

impl MarkSource for TripMarks {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        let landmarks = plan.overview.landmarks();
        let number = plan.day.number();
        let stars = landmarks
            .star_points()
            .iter()
            .filter(|point| DayNumber::from(*point) == number)
            .map(|point| {
                let look = MarkLook::named(MapMark::Star, MapText::plain(point.short()));
                OwnFeature::spot(look, Coordinate::from(point))
            });
        let peaks = landmarks.peaks().iter().map(|peak| {
            let label = MapText::peak(AsRef::as_ref(peak), peak.elevation());
            OwnFeature::spot(
                MarkLook::named(MapMark::Peak, label),
                Coordinate::from(peak),
            )
        });
        stars.chain(peaks).collect()
    }
}

pub struct NoPathMarks;

impl MarkSource for NoPathMarks {
    fn features(plan: &MapPlan<'_>, _place: &Spelling) -> Vec<MarkFeature> {
        plan.day
            .leg(plan.shown)
            .into_iter()
            .flat_map(Leg::no_mapped_paths)
            .filter_map(|stretch| {
                let line: &RouteLine = stretch.as_ref();
                line.point_along(line.length() * Share::HALF)
            })
            .map(|middle| OwnFeature::spot(MarkLook::plain(MapMark::NoPath), middle))
            .collect()
    }
}
