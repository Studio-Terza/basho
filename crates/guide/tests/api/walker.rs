use std::cell::Cell;
use std::ops::Range;

use guide::clock::{ClockTime, Minutes, TripTime};
use guide::end::End;
use guide::geo::{Coordinate, Degrees, Distance, Share};
use guide::guidance::{Guidance, LegProgress, Pointer};
use guide::itinerary::Day;
use guide::leg::{Leg, LegIndex, Mode};
use guide::leg_line::LegLine;
use guide::reading::Reading;
use guide::readout::{Arrow, Recovery, Status};

pub struct Walker<'day> {
    day: &'day Day,
    progress: LegProgress,
}

impl<'day> Walker<'day> {
    const STRIDE: Distance = Distance::new(10.0);
    const NEAR_END: Distance = Distance::new(60.0);
    const WRONG_WAY: Degrees = Degrees::new(100.0);
    const TOWARD: Degrees = Degrees::new(90.0);

    const STEPS: [Share; 4] = [
        Share::new(0.1),
        Share::HALF,
        Share::new(0.9),
        Share::new(1.0),
    ];

    pub fn walked(day: &'day Day, walk: LegIndex, arrival: ClockTime) -> Walker<'day> {
        let leg = day.leg(walk).expect("a walk");
        let span = leg.time(End::Start).minutes_until(arrival).amount();
        Self::STEPS
            .iter()
            .fold(Walker::fresh(day), |walker, share| {
                let time = leg
                    .time(End::Start)
                    .later_by(Minutes::rounded(span * share.fraction()).unwrap_or_default());
                let here = leg.line().point_along(leg.length() * share.fraction());
                walker.stepped(time, here.expect("a point on the walk"))
            })
    }

    pub fn reaching(day: &Day, place: &str) -> LegIndex {
        day.legs()
            .iter()
            .position(|leg| {
                leg.line_text(LegLine::Reach)
                    .text(Reading::Written)
                    .contains(place)
            })
            .map(LegIndex::new)
            .expect("a step reaching the place")
    }

    pub fn strolled(self, walk: LegIndex, until: Distance, late: Minutes) -> Walker<'day> {
        let leg = self.day.leg(walk).expect("a walk");
        Self::strides(until).fold(self, |walker, along| {
            let time = Self::planned(leg, along, late);
            walker.stepped(time, leg.line().point_along(along).expect("a point"))
        })
    }

    pub fn wrong_turns(
        self,
        walk: LegIndex,
        late: Minutes,
        aside: Distance,
        settle: Distance,
    ) -> Vec<String> {
        let leg = self.day.leg(walk).expect("a walk");
        let kept = Cell::new(self.progress);
        Self::strides(leg.length())
            .filter_map(|along| {
                let here = Self::beside(leg, along, aside)?;
                let walker = Walker::kept(self.day, kept.get());
                let guidance = walker.fix(Self::planned(leg, along, late), here);
                kept.set(guidance.progress().unwrap_or(kept.get()));
                let ahead = leg.line().point_along(along + Arrow::LOOK_AHEAD)?;
                let truth = leg.line().point_along(along)?.bearing_to(ahead);
                let left = leg.length() - along;
                let gap = guidance
                    .arrow()
                    .map_or(0.0, |arrow| arrow.pointer().bearing().gap_to(truth));
                let misplaced = (guidance.leg_index() != walk
                    && (left > Self::NEAR_END || guidance.leg_index() < walk))
                    .then_some("step");
                let reversed = (left > Self::NEAR_END
                    && along >= Self::STRIDE
                    && gap > Self::WRONG_WAY.degrees())
                .then_some("arrow");
                let alarmed = (left > Self::NEAR_END && guidance.status() != Some(Status::OnRoute))
                    .then_some("status");
                misplaced
                    .or(reversed)
                    .or(alarmed)
                    .filter(|_| along >= settle)
                    .map(|wrong| {
                        format!(
                            "day {} leg {walk} at {along}: {wrong}, leg {} arrow {gap:.0}° {:?}",
                            self.day.number(),
                            guidance.leg_index(),
                            guidance.status()
                        )
                    })
            })
            .collect()
    }

    pub fn fix_along(&self, walk: LegIndex, along: Distance, late: Minutes) -> Guidance<'day> {
        let leg = self.day.leg(walk).expect("a walk");
        let here = leg.line().point_along(along).expect("a point");
        self.fix(Self::planned(leg, along, late), here)
    }

    pub fn fixes_on(&self, walk: LegIndex) -> usize {
        self.day
            .leg(walk)
            .map_or(0, |leg| Self::strides(leg.length()).count())
    }

    pub fn strayed(
        self,
        walk: LegIndex,
        onto: LegIndex,
        off: &Range<f64>,
    ) -> Option<Guidance<'day>> {
        let leg = self.day.leg(walk)?;
        let road = self.day.leg(onto)?.line();
        let half = leg.length() * Share::HALF;
        let astray = Self::strides(road.length())
            .filter_map(|along| road.point_along(along))
            .find(|point| {
                leg.line()
                    .fix(*point)
                    .is_some_and(|fix| off.contains(&Distance::from(fix.off_route()).meters()))
            })?;
        Some(
            self.strolled(walk, half, Minutes::ZERO)
                .fix(Self::planned(leg, half, Minutes::ZERO), astray),
        )
    }

    fn strides(until: Distance) -> impl Iterator<Item = Distance> {
        (0..)
            .map(|stride| Self::STRIDE * f64::from(stride))
            .take_while(move |along| *along <= until)
    }

    pub fn stepped(self, time: ClockTime, here: Coordinate) -> Walker<'day> {
        let progress = self.fix(time, here).progress().unwrap_or(self.progress);
        Walker { progress, ..self }
    }

    pub fn fresh(day: &'day Day) -> Walker<'day> {
        Walker::kept(day, LegProgress::default())
    }

    pub fn kept(day: &'day Day, progress: LegProgress) -> Walker<'day> {
        Walker { day, progress }
    }

    pub fn on_step(day: &'day Day, walk: LegIndex, late: Minutes) -> Walker<'day> {
        Walker::kept(day, LegProgress::confirmed(day.number(), walk, late))
    }

    pub fn misleading_sights(day: &'day Day, aside: Distance) -> Vec<String> {
        day.legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() == Mode::Walk)
            .flat_map(|(position, _)| Self::misleading_on(day, LegIndex::new(position), aside))
            .collect()
    }

    pub fn misleading_on(day: &'day Day, walk: LegIndex, aside: Distance) -> Vec<String> {
        let leg = day.leg(walk).expect("a walk");
        let half = leg.length() * Share::HALF;
        let guidance = Self::beside(leg, half, aside).map(|here| {
            Walker::on_step(day, walk, Minutes::ZERO)
                .fix(Self::planned(leg, half, Minutes::ZERO), here)
        });
        let recovery = guidance.and_then(|guidance| guidance.recovery());
        let route = recovery
            .as_ref()
            .and_then(Recovery::route)
            .map(Pointer::bearing);
        recovery
            .iter()
            .flat_map(|recovery| recovery.toward().to_vec())
            .filter(|sight| {
                route.is_some_and(|route| {
                    sight.pointer().bearing().gap_to(route) > Self::TOWARD.degrees()
                })
            })
            .map(|sight| {
                format!(
                    "day {} leg {walk}: {}",
                    day.number(),
                    sight.name().text(Reading::Written)
                )
            })
            .collect()
    }

    fn planned(leg: &Leg, along: Distance, late: Minutes) -> ClockTime {
        leg.time(End::Start)
            .later_by(leg.planned_after(along) + late)
    }

    pub fn beside(leg: &Leg, along: Distance, aside: Distance) -> Option<Coordinate> {
        let back = leg
            .line()
            .point_along((along - Self::STRIDE).max(Distance::default()))?;
        let next = leg.line().point_along(along + Self::STRIDE)?;
        let east = next.lon().east_of(back).meters();
        let north = next.lat().north_of(back).meters();
        let span = east.hypot(north).max(f64::EPSILON);
        Some(
            leg.line()
                .point_along(along)?
                .offset_by(aside * (-north / span), aside * (east / span)),
        )
    }

    pub fn fix(&self, time: ClockTime, here: Coordinate) -> Guidance<'day> {
        let now = TripTime::on_date_at(self.day.zone(), self.day.date(), time);
        Guidance::new(self.day, self.progress, now, Some(here))
    }
}
