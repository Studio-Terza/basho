use crate::geo::Coordinate;
use newtype::Counter;
use std::iter;

use crate::clock::Pause;
use crate::clock::{ClockTime, Minutes};
use crate::end::End;
use crate::geo::Distance;
use crate::itinerary::{Caption, Day};
use crate::leg::{Branch, Leg, LegIndex, Mode};
use crate::leg_line::LegLine;
use crate::place::RubyText;
use crate::timeline::deadline::Deadline;
use crate::timeline::deadline_kind::DeadlineKind;
use crate::timeline::hold::Hold;
use crate::timeline::juncture::Juncture;
use crate::timeline::moment::Moment;
use crate::timeline::planned_stop::PlannedStop;
use crate::timeline::stretch::Stretch;
use crate::timeline::wait::Wait;
use crate::times::{ListedTime, PlanTime};

#[derive(Clone, Copy, Debug)]
pub struct Course<'day> {
    day: &'day Day,
}

impl<'day> Course<'day> {
    #[must_use]
    pub fn new(day: &'day Day) -> Course<'day> {
        Course { day }
    }

    #[must_use]
    pub fn day(self) -> &'day Day {
        self.day
    }

    #[must_use]
    pub fn planned(self, moment: Moment, index: LegIndex) -> Minutes {
        match moment {
            Moment::Start => self.day.span_before(index),
            Moment::Arrival => self.reach_at(index, Distance::new(f64::INFINITY)),
            Moment::Boarding => {
                index
                    .previous()
                    .filter(|before| self.day.leg(*before).is_some_and(Self::is_walk))
                    .map_or_else(
                        || self.planned(Moment::Start, index),
                        |before| self.planned(Moment::Arrival, before),
                    )
                    + self.wait(Wait::Visit, index)
            }
        }
    }

    #[must_use]
    pub fn wait(self, wait: Wait, index: LegIndex) -> Minutes {
        let leg = self.day.leg(index);
        match wait {
            Wait::Station => leg
                .zip(self.clock(Juncture::Resume, index))
                .map_or(Minutes::ZERO, |(leg, resume)| {
                    resume.since_midnight() - leg.time(End::Finish).since_midnight()
                }),
            Wait::Visit => index
                .previous()
                .and_then(|before| self.day.leg(before))
                .filter(|before| Self::is_walk(before))
                .zip(leg)
                .filter(|(before, fixed)| {
                    before
                        .time(End::Finish)
                        .later_by(before.paused(Pause::AtEnd))
                        < fixed.time(End::Start)
                })
                .map_or(Minutes::ZERO, |(before, _)| before.paused(Pause::AtEnd)),
        }
    }

    #[must_use]
    pub fn clock(self, juncture: Juncture, index: LegIndex) -> Option<ClockTime> {
        match juncture {
            Juncture::Boarding => index
                .previous()
                .and_then(|before| self.day.leg(before))
                .map_or_else(
                    || self.day.leave_from(),
                    |before| {
                        Some(
                            before
                                .time(End::Finish)
                                .later_by(self.wait(Wait::Visit, index)),
                        )
                    },
                ),
            Juncture::Resume => {
                let leg = self.day.leg(index)?;
                let dwell = self
                    .day
                    .leg(index.incremented())
                    .filter(|next| Self::is_walk(next))
                    .map_or(Minutes::ZERO, |_| leg.paused(Pause::AtEnd));
                Some(if Self::is_walk(leg) {
                    leg.time(End::Start)
                } else {
                    leg.time(End::Finish).later_by(dwell)
                })
            }
        }
    }

    #[must_use]
    pub fn hold(self, index: LegIndex) -> Hold {
        Hold::from(self.day.leg(index))
    }

    #[must_use]
    pub fn reach_at(self, index: LegIndex, along: Distance) -> Minutes {
        self.planned(Moment::Start, index)
            + self
                .day
                .leg(index)
                .filter(|leg| leg.mode() == Mode::Walk)
                .map_or(Minutes::ZERO, |leg| leg.planned_after(along))
    }

    #[must_use]
    pub fn backup_branch(self, ride: LegIndex) -> Option<&'day Branch> {
        let walked = ride
            .previous()
            .filter(|before| self.day.leg(*before).is_some_and(Self::is_walk));
        [Some(ride), walked]
            .into_iter()
            .flatten()
            .filter_map(|index| self.day.leg(index))
            .flat_map(Leg::branches)
            .find(|branch| branch.backup_ride().is_some())
    }

    #[must_use]
    pub fn run_end(self, ride: LegIndex) -> LegIndex {
        (usize::from(ride)..self.day.legs().len())
            .map(LegIndex::new)
            .take_while(|index| self.hold(*index) == Hold::Ride)
            .last()
            .unwrap_or(ride)
    }

    #[must_use]
    pub fn finish(self) -> Minutes {
        let last = LegIndex::new(self.day.legs().len().saturating_sub(1));
        self.day
            .leg(last)
            .filter(|leg| Self::is_walk(leg))
            .map_or_else(
                || self.planned(Moment::Start, LegIndex::new(self.day.legs().len())),
                |_| self.planned(Moment::Arrival, last),
            )
    }

    #[must_use]
    pub fn stretches(self) -> Vec<Stretch<'day>> {
        let fixed: Vec<LegIndex> = (0..self.day.legs().len())
            .map(LegIndex::new)
            .filter(|index| self.hold(*index) != Hold::Free)
            .collect();
        let openings = iter::once(Stretch::opening(
            LegIndex::default(),
            self.day.leave_from(),
            Minutes::ZERO,
        ))
        .chain(fixed.iter().map(|closer| {
            let resume = self.hold(*closer).resume_after(*closer);
            Stretch::opening(
                resume,
                self.clock(Juncture::Resume, *closer),
                self.planned(Moment::Start, resume),
            )
        }));
        let closers = fixed.iter().copied().map(Some).chain(iter::once(None));
        openings
            .zip(closers)
            .map(|(opening, closer)| self.closed(opening, closer))
            .collect()
    }

    #[must_use]
    pub fn closed(self, opening: Stretch<'day>, closer: Option<LegIndex>) -> Stretch<'day> {
        let end = closer.map_or(self.day.legs().len(), usize::from);
        let cutoffs = (usize::from(opening.opener())..end)
            .map(LegIndex::new)
            .flat_map(|index| self.cutoffs_on(index))
            .collect();
        let finish = closer.map_or_else(
            || self.finish(),
            |index| self.planned(Moment::Boarding, index),
        );
        let ending = closer.map_or_else(|| self.check_in(), |index| self.closing(index));
        opening.closed(closer, ending, finish).holding(cutoffs)
    }

    #[must_use]
    pub fn closing(self, index: LegIndex) -> Option<Deadline<'day>> {
        let leg = self.day.leg(index)?;
        let place = self
            .place_before(index)
            .unwrap_or_else(|| leg.line_text(LegLine::Way));
        let reach = self.planned(Moment::Boarding, index);
        leg.boarding_margin()
            .map(|margin| {
                Deadline::fixed(DeadlineKind::Ride, place, index)
                    .timed(
                        leg.listed(End::Start),
                        ListedTime::Planned(PlanTime::new(margin.target(leg.time(End::Start)))),
                    )
                    .reached_at(reach, leg.mode())
                    .urged(margin.haste())
            })
            .or_else(|| {
                leg.tour().map(|tour| {
                    Deadline::fixed(DeadlineKind::Tour, place, index)
                        .timed(leg.listed(End::Start), tour.check_in())
                        .reached_at(reach, Mode::Walk)
                })
            })
    }

    #[must_use]
    pub fn check_in(self) -> Option<Deadline<'day>> {
        let night = self.day.night()?;
        let last = LegIndex::new(self.day.legs().len().saturating_sub(1));
        Some(
            Deadline::fixed(DeadlineKind::CheckIn, &night[Caption::Name], last)
                .timed(night.listed_check_in(), night.listed_check_in())
                .vouched_by(night.as_ref())
                .reached_at(self.finish(), Mode::Walk),
        )
    }

    #[must_use]
    pub fn cutoffs_on(self, index: LegIndex) -> Vec<Deadline<'day>> {
        let base = self.planned(Moment::Start, index);
        self.day
            .leg(index)
            .filter(|leg| Self::is_walk(leg))
            .map(|leg| {
                leg.cutoff()
                    .map(|cutoff| {
                        Deadline::cutoff(
                            cutoff,
                            leg.line_text(LegLine::Reach),
                            index,
                            self.planned(Moment::Arrival, index),
                        )
                    })
                    .into_iter()
                    .chain(Self::stop_cutoffs(leg, index, base))
                    .collect()
            })
            .unwrap_or_default()
    }

    #[must_use]
    pub fn stop_cutoffs(leg: &'day Leg, index: LegIndex, base: Minutes) -> Vec<Deadline<'day>> {
        leg.along()
            .iter()
            .filter_map(|stop| {
                let reach = base + leg.planned_after(stop.distance_along());
                stop.cutoff()
                    .map(|cutoff| Deadline::cutoff(cutoff, stop.as_ref(), index, reach))
            })
            .collect()
    }

    #[must_use]
    pub fn planned_stops(self) -> Vec<PlannedStop<'day>> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| Self::is_walk(leg))
            .flat_map(|(position, leg)| {
                let index = LegIndex::new(position);
                let base = self.planned(Moment::Start, index);
                let along = leg
                    .along()
                    .iter()
                    .filter(|stop| stop.dwell_minutes() > Minutes::ZERO)
                    .map(move |stop| {
                        let after = leg.planned_after(stop.distance_along());
                        PlannedStop::new(
                            index,
                            stop.dwell_minutes(),
                            stop.as_ref(),
                            Coordinate::from(stop),
                        )
                        .reached_at(base + after)
                        .placed_at(stop.distance_along(), leg.time(End::Start).later_by(after))
                    });
                let dwell = (leg.paused(Pause::AtEnd) > Minutes::ZERO).then(|| {
                    PlannedStop::new(
                        index,
                        leg.paused(Pause::AtEnd),
                        leg.line_text(LegLine::Reach),
                        leg.point(End::Finish),
                    )
                    .reached_at(self.planned(Moment::Arrival, index))
                    .placed_at(leg.length(), leg.time(End::Finish))
                });
                along.chain(dwell).collect::<Vec<PlannedStop<'day>>>()
            })
            .collect()
    }

    #[expect(
        clippy::redundant_closure_for_method_calls,
        reason = "the landing type is private to the day module"
    )]
    fn place_before(self, index: LegIndex) -> Option<&'day RubyText> {
        index
            .previous()
            .and_then(|before| self.day.leg(before))
            .map_or_else(
                || self.day.landing().map(|landing| landing.airport()),
                |before| Some(Self::left_at(before)),
            )
    }

    fn left_at(before: &'day Leg) -> &'day RubyText {
        if Self::is_walk(before) {
            before.line_text(LegLine::Reach)
        } else {
            before
                .alighting()
                .unwrap_or_else(|| before.line_text(LegLine::Reach))
        }
    }

    #[must_use]
    pub fn is_walk(leg: &Leg) -> bool {
        leg.mode() == Mode::Walk
    }
}
