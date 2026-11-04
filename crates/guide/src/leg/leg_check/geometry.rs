use crate::end::End;
use crate::geo::{Distance, Share, Speed};
use crate::itinerary::Problem;
use crate::leg::leg_check::leg_rule::LegRule;
use crate::leg::{Leg, Mode};

pub struct Geometry;

impl Geometry {
    const LENGTH_SHARE: Share = Share::new(0.1);
    const LENGTH_FLOOR: Distance = Distance::new(150.0);
    const END_SLACK: Distance = Distance::new(80.0);
    const SLOWEST: Speed = Speed::kilometers_per_hour(1.5);
    const SHORTEST_LINE_LENGTH: usize = 2;

    fn walked(leg: &Leg) -> Vec<Problem> {
        let measured = leg.line().length();
        let allowed = (leg.length() * Self::LENGTH_SHARE).max(Self::LENGTH_FLOOR);
        let miss = leg.point(End::Finish).distance_to(leg.at);
        let speed = Speed::covering(leg.length(), leg.minutes());
        [
            ((measured - leg.length()).abs() > allowed).then_some(Problem::LengthMismatch {
                stated: leg.length(),
                measured,
            }),
            (miss > Self::END_SLACK).then_some(Problem::EndMiss { miss }),
            Some(speed)
                .filter(|speed| *speed < Self::SLOWEST)
                .or(Some(speed).filter(|speed| *speed > leg.gait().fastest()))
                .map(|speed| Problem::Pace {
                    length: leg.length(),
                    minutes: leg.minutes(),
                    speed,
                }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl LegRule for Geometry {
    fn problems(&self, leg: &Leg) -> Vec<Problem> {
        match leg.mode() {
            Mode::Walk => [
                leg.road().is_none().then_some(Problem::NoRoad),
                (leg.line().points().len() < Self::SHORTEST_LINE_LENGTH)
                    .then_some(Problem::NoRouteLine),
                leg.segments()
                    .is_empty()
                    .then_some(Problem::WalkWithoutSegments),
            ]
            .into_iter()
            .flatten()
            .next()
            .map_or_else(|| Self::walked(leg), |problem| vec![problem]),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => [
                leg.boarding().is_none().then_some(Problem::NoBoarding),
                leg.alighting().is_none().then_some(Problem::NoAlighting),
                leg.segments().first().map(|_| Problem::SegmentsOnRide),
            ]
            .into_iter()
            .flatten()
            .collect(),
        }
    }
}
