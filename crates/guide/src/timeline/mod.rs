mod backup;
mod buffer;
mod buffer_state;
pub mod course;
mod deadline;
mod deadline_kind;
mod footing;
mod halt;
mod here;
mod hold;
mod juncture;
mod moment;
mod planned_stop;
mod resume;
mod stance;
mod stop;
mod stretch;
mod stretch_index;
mod wait;
use crate::geo::Coordinate;
use newtype::Counter;
use std::iter;

use crate::clock::{ClockTime, Minutes, TripTime};
use crate::end::End;
use crate::geo::Distance;
use crate::itinerary::Day;
use crate::leg::{Branch, Leg, LegIndex};
use crate::times::{ListedTime, PlanTime};
use course::Course;
use footing::Footing;
use here::Here;
use hold::Hold;
use juncture::Juncture;
use moment::Moment;
use planned_stop::PlannedStop;
use resume::Resume;
use stretch::Stretch;
use stretch_index::StretchIndex;
use wait::Wait;

pub use backup::Backup;
pub use buffer::{Buffer, LiveGap};
pub use buffer_state::BufferState;
pub use deadline_kind::DeadlineKind;
pub use halt::Halt;
pub use stance::Stance;
pub use stop::Stop;

#[derive(Clone, Debug)]
pub struct Timeline<'day> {
    course: Course<'day>,
    stretches: Vec<Stretch<'day>>,
    stops: Vec<PlannedStop<'day>>,
    now: TripTime,
    clock: Option<Minutes>,
    here: Option<Here>,
    floor: Option<Minutes>,
    footing: Footing,
    delay: Option<Minutes>,
    stop: Option<Stop<'day>>,
}

impl<'day> Timeline<'day> {
    const LATE_TRAIN: Minutes = Minutes::new(10);

    #[must_use]
    pub fn new(day: &'day Day, now: TripTime) -> Timeline<'day> {
        let course = Course::new(day);
        Timeline {
            course,
            stretches: course.stretches(),
            stops: course.planned_stops(),
            now,
            clock: (now.trip_date() == day.date()).then(|| now.minutes_into(day.date())),
            here: None,
            floor: None,
            footing: Footing::OnFoot,
            delay: None,
            stop: None,
        }
    }

    #[must_use]
    pub fn placed(self, leg: LegIndex, along: Distance, stance: Stance) -> Timeline<'day> {
        let next = leg.incremented();
        let course = self.course;
        let live = self.clock.filter(|_| stance != Stance::Preview);
        let stance = self.meeting(leg, along, stance);
        let hold = course.hold(leg);
        let stretch = match (stance, hold) {
            (Stance::Done, _) => None,
            (Stance::Preview, _) => Some(StretchIndex::default()),
            (Stance::Missed | Stance::Waiting, Hold::Ride | Hold::Tour) => {
                self.find(|stretch| stretch.closer() == Some(leg))
            }
            (Stance::Moving, Hold::Ride) => self.find(|stretch| stretch.opener() == next),
            _ => self.find(|stretch| stretch.holds(leg)),
        };
        let reach = match (stance, hold) {
            (Stance::Missed | Stance::Waiting, Hold::Ride | Hold::Tour) => {
                course.planned(Moment::Boarding, leg)
            }
            (Stance::Moving, Hold::Ride) => course.planned(Moment::Start, next),
            (Stance::Waiting, _) => course.planned(Moment::Start, leg),
            _ => course.reach_at(leg, along),
        };
        let spot = match (stance, hold) {
            (Stance::Moving, Hold::Ride) => None,
            (Stance::Moving, Hold::Free | Hold::Tour) => Some(along),
            _ => Some(Distance::default()),
        };
        let halted = self
            .stop
            .map(|halt| halt.planned())
            .filter(|_| spot.is_some());
        let stop = halted.or_else(|| spot.and_then(|along| self.stop_at(leg, along)));
        let stretch = match (halted, stance) {
            (Some(planned), Stance::Moving)
                if planned.leg().incremented() != leg
                    || course.day().leg(leg).and_then(Leg::tour).is_none() =>
            {
                self.find(|stretch| stretch.holds(planned.leg()))
            }
            _ => stretch,
        };
        let reach = match (halted, stance) {
            (Some(planned), Stance::Moving) if planned.leg() != leg => planned.visit().end,
            _ => reach,
        };
        let waiting = match (stance, hold) {
            (Stance::Waiting, Hold::Free) => course.day().leg(leg).map(|leg| leg.time(End::Start)),
            _ => None,
        };
        let arrived = match (stance, hold) {
            (Stance::Waiting, Hold::Ride | Hold::Tour) => course.clock(Juncture::Boarding, leg),
            _ => None,
        };
        let spent = self
            .stop
            .filter(|halt| {
                stop.is_some_and(|planned| planned.visit().start == halt.planned().visit().start)
            })
            .map(|halt| halt.spent())
            .or_else(|| {
                stop.zip(live)
                    .map(|(planned, clock)| self.inferred(&planned, clock))
            });
        let most = match stance {
            Stance::Missed => Some(reach),
            Stance::Preview | Stance::Waiting | Stance::Moving | Stance::Done => None,
        };
        let floor = match (stance, hold) {
            (Stance::Waiting | Stance::Missed, Hold::Ride | Hold::Tour) => live,
            _ => None,
        };
        let aboard = match (stance, hold) {
            (Stance::Moving, Hold::Ride) => live.and_then(|clock| self.aboard(leg, clock, along)),
            _ => None,
        };
        let here = stretch
            .and_then(|index| {
                live.map(|clock| {
                    self.live_here(index, reach, clock)
                        .settled(stop.as_ref(), spent, waiting, arrived)
                        .capped(most)
                })
            })
            .or_else(|| stretch.map(|index| self.planned_here(index)));
        let here = aboard.map_or(here, |resume| self.aboard_here(resume));
        let early = live
            .zip(course.day().legs().first())
            .filter(|(clock, first)| *clock < first.time(End::Start).since_midnight());
        let footing = match (stance, hold, aboard, early) {
            (Stance::Moving, Hold::Ride, ..) | (_, _, Some(_), _) | (.., Some(_)) => {
                Footing::Seated
            }
            _ => Footing::OnFoot,
        };
        Timeline {
            here,
            floor,
            footing,
            ..self
        }
    }

    #[must_use]
    pub fn delayed(self, delay: Option<Minutes>) -> Timeline<'day> {
        Timeline { delay, ..self }
    }

    #[must_use]
    pub fn inferred(&self, stop: &PlannedStop<'day>, clock: Minutes) -> Minutes {
        let since = clock - stop.planned().since_midnight();
        self.delay.map_or_else(
            || {
                if since >= stop.booked() {
                    stop.booked()
                } else {
                    Minutes::ZERO
                }
            },
            |delay| since - delay,
        )
    }

    #[must_use]
    pub fn paused(self, halt: Option<Halt>, current: LegIndex) -> Timeline<'day> {
        let stop = halt.filter(|_| self.clock.is_some()).and_then(|halt| {
            let spent = halt.still_for(self.now)?;
            self.stops
                .iter()
                .filter(|stop| usize::from(stop.leg()).abs_diff(usize::from(current)) <= 1)
                .filter(|stop| Coordinate::from(*stop).distance_to(halt.location()) <= Stop::AT)
                .min_by(|left, right| {
                    let here = halt.location();
                    Coordinate::from(*left)
                        .distance_to(here)
                        .meters()
                        .total_cmp(&Coordinate::from(*right).distance_to(here).meters())
                })
                .map(|planned| Stop::new(*planned, spent))
        });
        Timeline { stop, ..self }
    }

    #[must_use]
    pub fn buffer(&self) -> Option<Buffer<'day>> {
        let start = self.start()?;
        self.binding_at(start, start, LegIndex::default(), LegIndex::new(usize::MAX))
    }

    #[must_use]
    pub fn buffer_through(&self, first: LegIndex, last: LegIndex) -> Option<Buffer<'day>> {
        let start = self.start()?;
        self.onward(start)
            .into_iter()
            .filter_map(|resume| self.binding_at(resume, start, first, last))
            .min_by_key(Buffer::spare)
    }

    #[must_use]
    pub fn binding_at(
        &self,
        resume: Resume,
        start: Resume,
        first: LegIndex,
        last: LegIndex,
    ) -> Option<Buffer<'day>> {
        let stretch = self.ahead(resume)?;
        let here = self.here.filter(|_| resume == start).unwrap_or_else(|| {
            Here::new(resume.stretch(), stretch.start(), resume.time(), self.clock)
        });
        let floor = self.floor.filter(|_| resume == start);
        let buffer = stretch.binding(here, floor, first, last)?;
        Some(
            buffer
                .seated(self.footing)
                .backed_by(self.backup_for(&buffer)),
        )
    }

    #[must_use]
    pub fn ahead(&self, resume: Resume) -> Option<Stretch<'day>> {
        let stretch = self.stretches.get(usize::from(resume.stretch()))?;
        let day = self.course.day();
        let gone = stretch
            .closer()
            .filter(|ride| self.course.hold(*ride) == Hold::Ride)
            .and_then(|ride| day.leg(ride))
            .filter(|ride| {
                self.clock
                    .is_some_and(|clock| clock > ride.time(End::Start).since_midnight())
            });
        let next = gone.and_then(|ride| {
            let backup = stretch
                .closer()
                .and_then(|index| self.course.backup_branch(index))?;
            let leaves = backup.backup_ride()?.listed(End::Start);
            let target = ride.boarding_margin()?.target(ClockTime::from(leaves));
            stretch
                .end()
                .map(|end| end.timed(leaves, ListedTime::Planned(PlanTime::new(target))))
        });
        Some(next.map_or_else(|| stretch.clone(), |end| stretch.clone().ending(end)))
    }

    #[must_use]
    pub fn stop(&self) -> Option<Stop<'day>> {
        self.stop
    }

    #[must_use]
    pub fn arrival(&self) -> Option<TripTime> {
        self.finish_from(self.start()?)
    }

    #[must_use]
    pub fn holds_now(&self, leg: LegIndex) -> bool {
        self.here
            .and_then(|here| self.stretches.get(usize::from(here.stretch())))
            .is_some_and(|stretch| stretch.holds(leg))
    }

    #[must_use]
    pub fn meeting(&self, leg: LegIndex, along: Distance, stance: Stance) -> Stance {
        self.course
            .day()
            .leg(leg)
            .filter(|tour| {
                tour.tour().is_some() && stance == Stance::Moving && along <= Stop::REACH
            })
            .filter(|tour| {
                self.clock
                    .is_none_or(|clock| clock < tour.time(End::Start).since_midnight())
            })
            .map_or(stance, |_| Stance::Waiting)
    }

    #[must_use]
    pub fn stop_at(&self, leg: LegIndex, along: Distance) -> Option<PlannedStop<'day>> {
        let day = self.course.day();
        let length = |index: LegIndex| day.leg(index).map_or(Distance::default(), Leg::length);
        let near: Vec<&PlannedStop<'day>> = self
            .stops
            .iter()
            .filter(|stop| {
                (stop.leg() == leg && (stop.along() - along).abs() <= Stop::AT)
                    || (along <= Stop::AT
                        && Some(stop.leg()) == leg.previous()
                        && length(stop.leg()) - stop.along() <= Stop::AT)
            })
            .collect();
        let clock = self.clock.unwrap_or_default();
        near.iter()
            .find(|stop| clock < stop.planned().since_midnight() + stop.booked())
            .or(near.last())
            .map(|stop| **stop)
    }

    #[must_use]
    pub fn aboard(&self, ride: LegIndex, clock: Minutes, along: Distance) -> Option<Resume> {
        let leg = self
            .course
            .day()
            .leg(ride)
            .filter(|leg| clock > leg.time(End::Finish).since_midnight() + Self::LATE_TRAIN)?;
        let backup = self
            .course
            .backup_branch(ride)?
            .backup_ride()
            .filter(|backup| backup.time(End::Start).since_midnight() <= clock)?;
        let span =
            backup.time(End::Finish).since_midnight() - backup.time(End::Start).since_midnight();
        let share = ((clock - backup.time(End::Start).since_midnight()).amount()
            / span.amount().max(1.0))
        .clamp(0.0, 1.0);
        let expected = leg.length() * share;
        Some(leg).filter(|leg| (along - expected).abs() < (leg.length() - along).abs())?;
        let end = self.course.run_end(ride);
        let landed =
            backup.time(End::Finish).since_midnight() + self.course.wait(Wait::Station, end);
        self.resumed_at(end.incremented(), landed.max(clock))
    }

    #[must_use]
    pub fn aboard_here(&self, resume: Resume) -> Option<Here> {
        let stretch = self.stretches.get(usize::from(resume.stretch()))?;
        Some(Here::new(
            resume.stretch(),
            stretch.start(),
            resume.time(),
            self.clock,
        ))
    }

    #[must_use]
    pub fn start(&self) -> Option<Resume> {
        let here = self.here?;
        let stretch = self.stretches.get(usize::from(here.stretch()))?;
        Some(Resume::new(here.stretch(), here.projected(stretch.start())))
    }

    #[must_use]
    pub fn onward(&self, from: Resume) -> Vec<Resume> {
        iter::successors(Some(from), |resume| self.step(*resume)).collect()
    }

    #[must_use]
    pub fn step(&self, resume: Resume) -> Option<Resume> {
        let stretch = self.stretches.get(usize::from(resume.stretch()))?;
        let closer = stretch.closer()?;
        let leg = self.course.day().leg(closer)?;
        let arrival = resume.arrival_in(stretch);
        match self.course.hold(closer) {
            Hold::Ride => {
                let waiting = self
                    .floor
                    .filter(|_| self.here.map(Here::stretch) == Some(resume.stretch()));
                self.boarded(closer, arrival, waiting)
            }
            Hold::Free | Hold::Tour => {
                self.resumed_at(closer, arrival.max(leg.time(End::Start).since_midnight()))
            }
        }
    }

    #[must_use]
    pub fn boarded(
        &self,
        ride: LegIndex,
        arrival: Minutes,
        waiting: Option<Minutes>,
    ) -> Option<Resume> {
        let leg = self.course.day().leg(ride)?;
        if arrival <= leg.time(End::Start).since_midnight() {
            self.resumed_at(
                self.course.hold(ride).resume_after(ride),
                self.course.clock(Juncture::Resume, ride)?.since_midnight(),
            )
        } else {
            let backup = self.reachable(
                ride,
                |limit: ClockTime| arrival <= limit.since_midnight(),
                waiting,
            )?;
            let end = self.course.run_end(ride);
            self.resumed_at(
                end.incremented(),
                backup.backup_ride()?.time(End::Finish).since_midnight()
                    + self.course.wait(Wait::Station, end),
            )
        }
    }

    #[must_use]
    pub fn reachable(
        &self,
        ride: LegIndex,
        reaches_by: impl Fn(ClockTime) -> bool,
        waiting: Option<Minutes>,
    ) -> Option<&'day Branch> {
        let leg = self.course.day().leg(ride)?;
        let branch = self.course.backup_branch(ride)?;
        let backup = branch.backup_ride()?;
        let target = leg.boarding_margin()?.target(backup.time(End::Start));
        let limit = waiting.map_or(target, |_| backup.time(End::Start));
        reaches_by(limit).then_some(branch)
    }

    #[must_use]
    pub fn resumed_at(&self, opener: LegIndex, time: Minutes) -> Option<Resume> {
        self.find(|stretch| stretch.opener() == opener)
            .map(|index| Resume::new(index, time))
    }

    #[must_use]
    pub fn finish_from(&self, from: Resume) -> Option<TripTime> {
        let last = self.onward(from).last().copied()?;
        let stretch = self
            .stretches
            .get(usize::from(last.stretch()))
            .filter(|stretch| stretch.closer().is_none())?;
        let midnight = TripTime::on_date_at(
            self.course.day().zone(),
            self.course.day().date(),
            ClockTime::at_hour(0),
        );
        Some(midnight.later_by(last.arrival_in(stretch)))
    }

    fn find(&self, wanted: impl Fn(&Stretch<'day>) -> bool) -> Option<StretchIndex> {
        self.stretches
            .iter()
            .position(wanted)
            .map(StretchIndex::new)
    }

    fn live_here(&self, index: StretchIndex, reach: Minutes, clock: Minutes) -> Here {
        let opens = self
            .stretches
            .get(usize::from(index))
            .and_then(Stretch::opens)
            .map_or(clock, |opens| opens.since_midnight().max(clock));
        Here::new(index, reach, opens, Some(clock))
    }

    fn planned_here(&self, index: StretchIndex) -> Here {
        let stretch = self.stretches.get(usize::from(index));
        let first = self
            .course
            .day()
            .legs()
            .first()
            .map(|leg| leg.time(End::Start));
        let opens = stretch
            .and_then(Stretch::opens)
            .or(first)
            .map_or(Minutes::ZERO, ClockTime::since_midnight);
        Here::new(
            index,
            stretch.map_or(Minutes::ZERO, Stretch::start),
            opens,
            None,
        )
    }

    fn backup_for(&self, buffer: &Buffer<'day>) -> Option<Backup> {
        let day = self.course.day();
        let deadline = buffer.deadline();
        let ride = Some(deadline.leg()).filter(|ride| self.course.hold(*ride) == Hold::Ride)?;
        let leg = day.leg(ride)?;
        let midnight = TripTime::on_date_at(day.zone(), day.date(), ClockTime::at_hour(0));
        let check_in = day
            .night()
            .map(|night| midnight.later_by(night.check_in_by().since_midnight()));
        let backup = self
            .reachable(
                ride,
                |limit| buffer.reaches_by(limit),
                buffer.boarding().or(self.floor),
            )
            .and_then(Branch::backup_ride)
            .filter(|_| deadline.event() == leg.time(End::Start));
        match (backup, buffer.reaches_by(deadline.event())) {
            (Some(backup), _) => {
                let end = self.course.run_end(ride);
                let onward = backup.time(End::Finish).since_midnight()
                    + self.course.wait(Wait::Station, end);
                let arrives = self
                    .resumed_at(end.incremented(), onward)
                    .and_then(|resume| self.finish_from(resume));
                Some(Backup::ride(backup.listed(End::Start), arrives, check_in))
            }
            (None, false) => Some(Backup::none(check_in)),
            (None, true) => None,
        }
    }
}
