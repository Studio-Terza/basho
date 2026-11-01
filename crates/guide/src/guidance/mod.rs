mod arrival;
mod leg_progress;
pub mod marker;
mod nearness;
mod phase;
mod plan;
mod platform;
mod pointer;
mod position_source;
mod pressing;
mod remaining;
mod ride_search;
mod ride_slack;
pub mod schedule;
mod sighting;
mod snap;
mod track;
pub mod tracking;
mod walk_search;
use crate::clock::ClockTime;
use crate::clock::{DayTense, Minutes, Sunset, TripTime};
use crate::end::End;
use crate::geo::{Coordinate, Distance, RouteFix, Share};
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex, Mode};
use crate::readout::{Arrow, Recovery, Status, TimeCheck};
use crate::timeline::{Buffer, Halt, Stop, Timeline};
use crate::times::{ListedTime, ShownTime};
use marker::Marker;
use nearness::Nearness;
use newtype::Counter;
use platform::Platform;
use remaining::Remaining;
use ride_search::RideSearch;
use schedule::Schedule;
use snap::Snap;
use track::Track;
use tracking::Tracking;
use walk_search::WalkSearch;

pub use arrival::Arrival;
pub use leg_progress::LegProgress;
pub use phase::Phase;
pub use plan::Plan;
pub use pointer::Pointer;
pub use position_source::PositionSource;
pub use pressing::Pressing;
pub use sighting::Sighting;

#[derive(Clone, Copy, Debug)]
pub struct Guidance<'day> {
    day: &'day Day,
    leg: LegIndex,
    along: Distance,
    source: PositionSource,
    phase: Phase,
    offset: Minutes,
    halt: Option<Halt>,
    off_route: Option<Distance>,
    position: Option<Coordinate>,
    now: TripTime,
    kept: LegProgress,
}

impl<'day> Guidance<'day> {
    #[must_use]
    pub fn new(
        day: &'day Day,
        progress: LegProgress,
        now: TripTime,
        position: Option<Coordinate>,
    ) -> Guidance<'day> {
        let schedule = Schedule::estimate(day, progress, now);
        let kept = progress.for_day(day.number());
        let estimate = Guidance {
            day,
            leg: *AsRef::<LegIndex>::as_ref(&schedule),
            along: *AsRef::<Distance>::as_ref(&schedule),
            source: PositionSource::Estimate,
            phase: schedule.phase(),
            offset: schedule.delay().unwrap_or(Minutes::ZERO),
            halt: None,
            off_route: None,
            position,
            now,
            kept: progress,
        };
        match position {
            _ if progress.arrived(day) => estimate.arrived(),
            Some(here) if schedule.phase() == Phase::Preview => WalkSearch::new(day, here, now)
                .best(None)
                .map_or(estimate, |track| estimate.moving(track)),
            Some(here) => estimate.located(here, kept),
            None => estimate,
        }
    }

    #[must_use]
    pub fn paused(self, halt: Option<Halt>) -> Guidance<'day> {
        Guidance { halt, ..self }
    }

    #[must_use]
    pub fn confirmed(self, previous: LegProgress) -> LegProgress {
        let held = self.held(previous);
        let tracking = previous
            .for_day(self.day.number())
            .map_or(Tracking::Automatic, LegProgress::tracking)
            .observing(held.placing(), held.source);
        held.progress().unwrap_or(previous).tracked(tracking)
    }

    #[must_use]
    pub fn after_gps_loss(self, estimate: Guidance<'day>) -> Guidance<'day> {
        let source = PositionSource::GpsLost { since: self.now };
        self.off_route.map_or(
            Guidance {
                source,
                position: self.position,
                ..estimate
            },
            |_| Guidance {
                source,
                now: estimate.now,
                ..self
            },
        )
    }

    #[must_use]
    pub fn day(&self) -> &'day Day {
        self.day
    }

    #[must_use]
    pub fn leg_index(&self) -> LegIndex {
        self.leg
    }

    #[must_use]
    pub fn leg(&self) -> Option<&'day Leg> {
        self.day.legs().get(usize::from(self.leg))
    }

    #[must_use]
    pub fn along(&self) -> Distance {
        self.along
    }

    #[must_use]
    pub fn along_share(&self) -> Share {
        Share::ratio(
            self.along,
            self.leg().map_or(Distance::default(), Leg::length),
        )
    }

    #[must_use]
    pub fn left(&self) -> Remaining {
        self.leg().map_or(Remaining::default(), |leg| {
            Remaining::between(self.along, leg.length())
        })
    }

    #[must_use]
    pub fn source(&self) -> PositionSource {
        self.source
    }

    #[must_use]
    pub fn phase(&self) -> Phase {
        self.phase
    }

    #[must_use]
    pub fn off_route(&self) -> Option<Distance> {
        self.off_route
    }

    #[must_use]
    pub fn missed_ride(&self) -> Option<LegIndex> {
        (self.phase == Phase::Missed).then_some(self.leg)
    }

    #[must_use]
    pub fn marker(&self) -> Option<Coordinate> {
        self.marking().map(Marker::location)
    }

    #[must_use]
    pub fn marking(&self) -> Option<Marker> {
        self.off_route
            .and(self.position)
            .map(Marker::Astray)
            .or_else(|| {
                self.source
                    .lost_for(self.now)
                    .and(self.position)
                    .map(Marker::LastFix)
            })
            .or_else(|| self.leg().and_then(|leg| Marker::on_route(leg, self.along)))
    }

    #[must_use]
    pub fn progress(&self) -> Option<LegProgress> {
        (self.source == PositionSource::Gps)
            .then_some(self.kept)
            .filter(|kept| kept.for_day(self.day.number()).is_some())
    }

    #[must_use]
    pub fn arrival(&self) -> Option<Arrival> {
        let night = self.day.night()?;
        let on_day = |time: ClockTime| TripTime::on_date_at(self.day.zone(), self.day.date(), time);
        let end = on_day(self.day.legs().last()?.time(End::Finish));
        let at = self.timeline().arrival();
        let sunset = self.day.sunset().and_then(Sunset::time).map(on_day);
        (self.tense() == DayTense::Today
            && self.phase != Phase::Preview
            && self.phase != Phase::Done)
            .then(|| Arrival::new(end, at, sunset, Some(on_day(night.check_in_by()))))
    }

    #[must_use]
    pub fn timeline(&self) -> Timeline<'day> {
        Timeline::new(self.day, self.now)
            .delayed(self.position.is_none().then_some(self.offset))
            .paused(self.halt, self.leg)
            .placed(self.leg, self.along, self.phase.stance())
    }

    #[must_use]
    pub fn tense(&self) -> DayTense {
        let early = self
            .day
            .leave_from()
            .filter(|from| self.now.minutes_into(self.day.date()) < from.since_midnight());
        match (self.now.tense_of(self.day.date()), early) {
            (DayTense::Today, Some(_)) => DayTense::Ahead,
            (tense, _) => tense,
        }
    }

    #[must_use]
    pub fn buffer(&self) -> Option<Buffer<'day>> {
        self.timeline()
            .buffer()
            .filter(|_| self.tense() == DayTense::Today)
    }

    #[must_use]
    pub fn buffer_through(&self, first: LegIndex, last: LegIndex) -> Option<Buffer<'day>> {
        self.timeline().buffer_through(first, last)
    }

    #[must_use]
    pub fn stop(&self) -> Option<Stop<'day>> {
        self.timeline().stop()
    }

    #[must_use]
    pub fn shifted(&self, leg: LegIndex, planned: ListedTime) -> ShownTime {
        if leg >= self.leg
            && self
                .day
                .leg(leg)
                .is_some_and(|found| found.mode() == Mode::Walk)
            && self.timeline().holds_now(leg)
        {
            ShownTime::off_by(planned, self.offset)
        } else {
            ShownTime::from(planned)
        }
    }

    #[must_use]
    pub fn now(&self) -> TripTime {
        self.now
    }

    #[must_use]
    pub fn plans(&self, leg: LegIndex) -> Vec<Plan<'day>> {
        let guidance = *self;
        self.day
            .branch_legs(leg)
            .into_iter()
            .filter_map(|index| self.day.leg(index).map(|found| (index, found)))
            .flat_map(|(index, found)| {
                found
                    .branches()
                    .iter()
                    .map(move |branch| Plan::new(&guidance, index, branch))
            })
            .collect()
    }

    #[must_use]
    pub fn time_check(&self) -> Option<TimeCheck> {
        let leg = self.leg()?;
        self.phase
            .due(leg, self.offset)
            .filter(|_| self.tense() == DayTense::Today)
            .map(|time| {
                let due =
                    TripTime::on_date_at(self.day.zone(), self.day.date(), ClockTime::from(time));
                let check = if leg.mode() != Mode::Walk && self.phase.at_platform().is_some() {
                    TimeCheck::boarding(due, time, self.now)
                } else {
                    TimeCheck::comparing(due, time, self.now, leg)
                };
                if self.source == PositionSource::Gps || self.off_route.is_some() {
                    check
                } else {
                    check.estimated(self.source.last_fix_time())
                }
            })
    }

    #[must_use]
    pub fn status(&self) -> Option<Status> {
        let leg = self
            .leg()
            .filter(|_| self.phase != Phase::Preview && self.phase != Phase::Done)?;
        Some(
            self.phase
                .ride_status(leg)
                .filter(|_| self.source == PositionSource::Gps)
                .unwrap_or_else(|| self.source.status(self.now, self.off_route)),
        )
    }

    #[must_use]
    pub fn arrow(&self) -> Option<Arrow> {
        let here = self.position.filter(|_| {
            self.phase != Phase::Done
                && self
                    .source
                    .lost_for(self.now)
                    .is_none_or(|lost| lost <= Arrow::STALE)
        })?;
        let leg = self.leg()?;
        let arrow = if leg.mode() == Mode::Walk {
            Some(
                self.off_route
                    .and_then(|_| leg.line().fix(here))
                    .map_or_else(
                        || {
                            Arrow::ahead(
                                here,
                                leg,
                                self.along,
                                self.day.leg(self.leg.incremented()),
                            )
                        },
                        |fix| Arrow::back_to_route(here, fix),
                    ),
            )
        } else {
            self.phase
                .at_platform()
                .and(leg.boarding())
                .filter(|boarding| here.distance_to(*boarding) > Nearness::GpsError.distance())
                .map(|boarding| Arrow::toward(here, boarding))
        };
        arrow.map(|arrow| arrow.dated(self.source.last_fix_time()))
    }

    #[must_use]
    pub fn recovery(&self) -> Option<Recovery<'day>> {
        let here = self
            .position
            .filter(|_| self.tense() == DayTense::Today && self.phase.at_platform().is_none())?;
        self.off_route
            .map(|_| Minutes::ZERO)
            .or_else(|| self.time_check().and_then(TimeCheck::late_by))
            .and(self.leg())
            .filter(|leg| leg.mode() == Mode::Walk)
            .map(|_| Recovery::near(self.day, self.leg, here, self.source.last_fix_time()))
    }

    #[must_use]
    pub fn moving(self, track: Track) -> Guidance<'day> {
        let snap = track.snap();
        let kept = track.remembered(self.day, self.now).unwrap_or(self.kept);
        let off = Distance::from(
            self.position
                .and_then(|here| self.day.leg(snap.index())?.line().fix(here))
                .map_or(snap.fix().off_route(), RouteFix::off_route),
        )
        .min(Distance::from(snap.fix().off_route()));
        let arrived = LegProgress::confirmed(self.day.number(), snap.index(), Minutes::ZERO)
            .placed(snap.fix().along(), self.now);
        Guidance {
            leg: snap.index(),
            along: snap.fix().along(),
            source: PositionSource::Gps,
            phase: if arrived.arrived(self.day) {
                Phase::Done
            } else {
                Phase::Moving
            },
            offset: track.offset(self.day, self.now),
            off_route: self
                .day
                .leg(snap.index())
                .filter(|leg| off > Nearness::OffRoute.beside(leg.line()))
                .map(|_| off),
            kept,
            ..self
        }
    }

    #[must_use]
    pub fn riding(self, ride: Snap) -> Guidance<'day> {
        Guidance {
            leg: ride.index(),
            along: ride.fix().along(),
            source: PositionSource::Gps,
            phase: Phase::Moving,
            offset: Minutes::ZERO,
            off_route: None,
            kept: LegProgress::confirmed(self.day.number(), ride.index(), Minutes::ZERO)
                .placed(ride.fix().along(), self.now),
            ..self
        }
    }

    #[must_use]
    pub fn boarding(self, platform: Platform<'day>, here: Coordinate) -> Guidance<'day> {
        platform.departing(here, self.now).map_or_else(
            || Guidance {
                leg: platform.ride(),
                along: Distance::default(),
                source: PositionSource::Gps,
                phase: platform.phase(self.now),
                offset: platform.late(self.now),
                off_route: None,
                kept: LegProgress::confirmed(
                    self.day.number(),
                    platform.ride(),
                    platform.late(self.now),
                )
                .placed(Distance::default(), self.now),
                ..self
            },
            |ride| self.riding(ride),
        )
    }

    fn located(self, here: Coordinate, kept: Option<LegProgress>) -> Guidance<'day> {
        let walking = WalkSearch::new(self.day, here, self.now).best(kept);
        let rides = RideSearch::new(self.day, here, self.now);
        let kept_leg = kept.map(leg_progress::LegProgress::leg);
        kept.filter(|_| {
            walking
                .is_none_or(|track| track.snap().fix().off_route() > Nearness::OffRoute.distance())
        })
        .and_then(|kept| rides.riding_on(kept))
        .map(|ride| self.riding(ride))
        .or_else(|| {
            Platform::arriving(self.day, walking.map(Track::snap))
                .map(|platform| self.boarding(platform, here))
        })
        .or_else(|| walking.map(|track| self.moving(track)))
        .or_else(|| {
            Platform::nearby(self.day, here, kept_leg.unwrap_or_default())
                .map(|platform| self.boarding(platform, here))
        })
        .or_else(|| {
            rides
                .upcoming(kept_leg.unwrap_or(self.leg))
                .map(|ride| self.riding(ride))
        })
        .unwrap_or_else(|| self.lost(here, kept.unwrap_or_default()))
    }

    fn arrived(self) -> Guidance<'day> {
        let last = LegIndex::new(self.day.legs().len().saturating_sub(1));
        Guidance {
            leg: last,
            along: self.day.leg(last).map_or(Distance::default(), Leg::length),
            phase: Phase::Done,
            ..self
        }
    }

    fn lost(self, here: Coordinate, kept: LegProgress) -> Guidance<'day> {
        let known = kept.for_day(self.day.number());
        let leg = self.rejoining(known.map_or(self.leg, |kept| kept.resumed(self.day)));
        Guidance {
            leg,
            along: known
                .filter(|kept| kept.leg() == leg)
                .map_or(Distance::default(), leg_progress::LegProgress::along),
            source: PositionSource::Gps,
            phase: Phase::Moving,
            offset: known.map_or(self.offset, LegProgress::offset),
            off_route: self
                .day
                .leg(leg)
                .and_then(|leg| leg.line().fix(here))
                .map(|fix| Distance::from(fix.off_route())),
            ..self
        }
    }

    fn rejoining(&self, base: LegIndex) -> LegIndex {
        let walk = |position: &usize| {
            self.day
                .leg(LegIndex::new(*position))
                .is_some_and(|leg| leg.mode() == Mode::Walk)
        };
        let clock = self.now.minutes_into(self.day.date());
        self.day
            .leg(base)
            .filter(|leg| leg.mode() != Mode::Walk)
            .map_or(base, |ride| {
                let after = (usize::from(base)..self.day.legs().len()).find(walk);
                let before = (0..usize::from(base)).rev().find(walk);
                if clock <= ride.time(End::Finish).since_midnight() {
                    before.or(after)
                } else {
                    after.or(before)
                }
                .map_or(base, LegIndex::new)
            })
    }
}
