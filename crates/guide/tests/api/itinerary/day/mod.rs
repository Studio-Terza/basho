mod goal;

use crate::fixture::Fixture;
use newtype::Counter;
use rayon::prelude::*;

use guide::ArrivalMark;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::Pause;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Coordinate;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::itinerary::Day;
use guide::leg::Leg;
use guide::leg::LegIndex;
use guide::leg::Mode;
use guide::timeline::Buffer;
use guide::timeline::Halt;
use guide::timeline::Stop;

const ON_PLAN: Minutes = Minutes::new(1);
const LATE_TRAIN: Minutes = Minutes::new(10);
const LATENESS: [Minutes; 3] = [Minutes::new(15), Minutes::new(45), Minutes::new(60 + 30)];

struct Sample {
    leg: LegIndex,
    time: ClockTime,
    here: Option<Coordinate>,
    since: Option<ClockTime>,
}

impl Sample {
    pub fn at_start(day: &Day, index: LegIndex, leg: &Leg) -> Sample {
        let stopped = index
            .previous()
            .and_then(|before| day.leg(before))
            .filter(|before| before.mode() == Mode::Walk)
            .filter(|before| before.paused(Pause::AtEnd) > Minutes::ZERO);
        Sample {
            leg: index,
            time: leg.time(End::Start),
            here: leg.line().points().first().copied(),
            since: stopped.map(|leg| leg.time(End::Finish)),
        }
    }

    pub fn halfway(index: LegIndex, leg: &Leg) -> Sample {
        let half = leg.line().length() * 0.5;
        let time = leg.time(End::Start).later_by(leg.planned_after(half));
        let stop = leg
            .along()
            .iter()
            .filter(|stop| stop.dwell_minutes() > Minutes::ZERO)
            .filter(|stop| stop.distance_along() <= half)
            .find(|stop| half - stop.distance_along() <= Stop::REACH);
        Sample {
            leg: index,
            time,
            here: leg.line().point_along(half),
            since: stop.map(|stop| time.later_by(-stop.dwell_minutes())),
        }
    }

    pub fn leaving(index: LegIndex, leg: &Leg) -> Sample {
        Sample {
            leg: index,
            time: leg.time(End::Start),
            here: leg.line().points().first().copied(),
            since: None,
        }
    }

    pub fn boarding(day: &Day, index: LegIndex, ride: &Leg) -> Vec<Sample> {
        let before = index
            .previous()
            .and_then(|before| day.leg(before).map(|leg| (before, leg)));
        before
            .map(|(position, leg)| {
                let visit = if leg.mode() == Mode::Walk
                    && leg.time(End::Finish).later_by(leg.paused(Pause::AtEnd))
                        < ride.time(End::Start)
                {
                    leg.paused(Pause::AtEnd)
                } else {
                    Minutes::ZERO
                };
                let from = leg
                    .time(End::Finish)
                    .later_by(visit)
                    .since_midnight()
                    .count();
                (from..ride.time(End::Start).since_midnight().count())
                    .map(|minute| Sample {
                        leg: position,
                        time: ClockTime::at_minutes(Minutes::new(minute)),
                        here: ride.boarding(),
                        since: None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn unseen(day: &Day, time: ClockTime) -> Option<Sample> {
        let position = day
            .legs()
            .iter()
            .rposition(|leg| leg.time(End::Start) <= time)?;
        Some(Sample {
            leg: LegIndex::new(position),
            time,
            here: None,
            since: None,
        })
    }

    pub fn at_end(index: LegIndex, leg: &Leg) -> Sample {
        Sample {
            leg: index,
            time: leg.time(End::Finish),
            here: Some(leg.point(End::Finish)),
            since: None,
        }
    }
}

struct Audit<'day> {
    day: &'day Day,
}

impl<'day> Audit<'day> {
    pub fn samples(&self) -> Vec<Sample> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() == Mode::Walk)
            .flat_map(|(position, leg)| {
                let index = LegIndex::new(position);
                [
                    Sample::at_start(self.day, index, leg),
                    Sample::halfway(index, leg),
                    Sample::leaving(index, leg),
                    Sample::at_end(index, leg),
                ]
            })
            .chain(self.boardings())
            .chain(self.unseen())
            .collect()
    }

    pub fn boardings(&self) -> Vec<Sample> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() != Mode::Walk)
            .flat_map(|(position, ride)| Sample::boarding(self.day, LegIndex::new(position), ride))
            .collect()
    }

    pub fn unseen(&self) -> Vec<Sample> {
        let first = self
            .day
            .leave_from()
            .or(self.day.legs().first().map(|leg| leg.time(End::Start)))
            .map_or(Minutes::ZERO, ClockTime::since_midnight);
        let last = self
            .day
            .legs()
            .last()
            .map_or(Minutes::ZERO, |leg| leg.time(End::Finish).since_midnight());
        (first.count()..last.count())
            .filter_map(|minute| {
                Sample::unseen(self.day, ClockTime::at_minutes(Minutes::new(minute)))
            })
            .collect()
    }

    pub fn backup_of(&self, ride: LegIndex) -> Option<&'day guide::leg::Branch> {
        let walked = ride.previous().filter(|before| {
            self.day
                .leg(*before)
                .is_some_and(|leg| leg.mode() == Mode::Walk)
        });
        [Some(ride), walked]
            .into_iter()
            .flatten()
            .filter_map(|index| self.day.leg(index))
            .flat_map(Leg::branches)
            .find(|branch| branch.backup_ride().is_some())
    }

    pub fn run_end(&self, ride: LegIndex) -> LegIndex {
        (usize::from(ride)..self.day.legs().len())
            .map(LegIndex::new)
            .take_while(|index| {
                self.day
                    .leg(*index)
                    .is_some_and(|leg| leg.mode() != Mode::Walk)
            })
            .last()
            .unwrap_or(ride)
    }

    pub fn planned_boarding(&self, ride: LegIndex) -> Option<Minutes> {
        let leg = self.day.leg(ride)?;
        let before = ride.previous().and_then(|before| self.day.leg(before));
        let visit = before
            .filter(|before| before.mode() == Mode::Walk)
            .map(|before| {
                before
                    .time(End::Finish)
                    .later_by(before.paused(Pause::AtEnd))
            })
            .filter(|leaves| *leaves < leg.time(End::Start));
        visit
            .or(before.map(|leg| leg.time(End::Finish)))
            .or(self.day.leave_from())
            .map(ClockTime::since_midnight)
    }

    pub fn brute(
        &self,
        ride: LegIndex,
        arrival: Minutes,
        waiting: Option<Minutes>,
    ) -> Option<Minutes> {
        let leg = self.day.leg(ride)?;
        let last = self.day.legs().last()?.time(End::Finish).since_midnight();
        if arrival <= leg.time(End::Start).since_midnight() {
            Some(last)
        } else {
            let backup = self.backup_of(ride)?.backup_ride()?;
            let target = waiting
                .map_or(
                    leg.boarding_margin()?.target(backup.time(End::Start)),
                    |_| backup.time(End::Start),
                )
                .since_midnight();
            let end = self.run_end(ride);
            let landed = self.day.leg(end)?.time(End::Finish).since_midnight();
            (arrival <= target)
                .then(|| backup.time(End::Finish).since_midnight() - landed)
                .and_then(|delay| self.delayed(end.incremented(), delay))
        }
    }

    pub fn delayed(&self, from: LegIndex, delay: Minutes) -> Option<Minutes> {
        let last = self.day.legs().last()?.time(End::Finish).since_midnight();
        let next = (usize::from(from)..self.day.legs().len())
            .map(LegIndex::new)
            .find(|index| {
                self.day
                    .leg(*index)
                    .is_some_and(|leg| leg.mode() != Mode::Walk)
            });
        next.map_or(Some(last + delay), |ride| {
            self.brute(ride, self.planned_boarding(ride)? + delay, None)
        })
    }

    pub fn clock_of(&self, minutes: Minutes) -> ClockTime {
        TripTime::on_date_at(self.day.zone(), self.day.date(), ClockTime::at_hour(0))
            .later_by(minutes)
            .time()
    }

    pub fn late_mismatches(&self) -> Vec<String> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() != Mode::Walk)
            .flat_map(|(position, ride)| {
                LATENESS.iter().filter_map(move |late| {
                    let index = LegIndex::new(position);
                    let arrival = self.planned_boarding(index)? + *late;
                    let sample = Sample {
                        leg: index.previous()?,
                        time: self.clock_of(arrival),
                        here: ride.boarding(),
                        since: None,
                    };
                    let shown = self
                        .live(&sample)
                        .arrival()
                        .and_then(|arrival| arrival.time(ArrivalMark::Arrives));
                    let expected = self
                        .brute(index, arrival, Some(arrival))
                        .map(|minutes| self.clock_of(minutes));
                    (arrival > ride.time(End::Start).since_midnight() && shown != expected).then(
                        || {
                            format!(
                                "day {} {} late {late}: inn shown {shown:?}, expected {expected:?}",
                                self.day.number(),
                                ride.listed(End::Start)
                            )
                        },
                    )
                })
            })
            .collect()
    }

    pub fn aboard_mismatches(&self) -> Vec<String> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter_map(|(position, ride)| {
                let index = LegIndex::new(position);
                let backup = self.backup_of(index)?.backup_ride()?;
                let span = backup.time(End::Finish).since_midnight()
                    - backup.time(End::Start).since_midnight();
                let middle = (backup.time(End::Start).since_midnight()
                    + Minutes::rounded(span.amount() * 0.5).unwrap_or_default())
                .max(ride.time(End::Finish).since_midnight() + LATE_TRAIN + ON_PLAN);
                let here = ride.line().point_along(ride.line().length() * 0.5);
                let progress = LegProgress::confirmed(self.day.number(), index, Minutes::ZERO);
                let now =
                    TripTime::on_date_at(self.day.zone(), self.day.date(), self.clock_of(middle));
                let guidance = Guidance::new(self.day, progress, now, here);
                let riding = Some(guidance.leg_index()).filter(|leg| *leg == index);
                let shown = riding
                    .and_then(|_| guidance.arrival())
                    .and_then(|arrival| arrival.time(ArrivalMark::Arrives));
                let end = self.run_end(index);
                let landed = self.day.leg(end)?.time(End::Finish).since_midnight();
                let expected = self
                    .delayed(
                        end.incremented(),
                        backup.time(End::Finish).since_midnight() - landed,
                    )
                    .map(|minutes| self.clock_of(minutes));
                (riding.is_some()
                    && middle < backup.time(End::Finish).since_midnight()
                    && shown != expected)
                    .then(|| {
                        format!(
                            "day {} on the {} backup: inn shown {shown:?}, expected {expected:?}",
                            self.day.number(),
                            backup.listed(End::Start)
                        )
                    })
            })
            .collect()
    }

    pub fn stranded(&self) -> Vec<String> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() != Mode::Walk)
            .filter_map(|(position, ride)| {
                let sample = Sample {
                    leg: LegIndex::new(position).previous()?,
                    time: ride.time(End::Start).later_by(ON_PLAN),
                    here: ride.boarding(),
                    since: None,
                };
                let buffer = self.live_buffer(&sample)?;
                let spare = buffer.spare();
                (buffer.deadline().event() == ride.time(End::Start) && spare >= Minutes::ZERO).then(
                    || {
                        format!(
                            "day {} {}: {spare} to spare after it left",
                            self.day.number(),
                            ride.listed(End::Start)
                        )
                    },
                )
            })
            .collect()
    }

    pub fn closer(&self, sample: &Sample) -> Option<LegIndex> {
        let legs = self.day.legs();
        let begun = legs
            .get(usize::from(sample.leg))
            .filter(|leg| leg.mode() != Mode::Walk || leg.tour().is_some())
            .filter(|leg| sample.time >= leg.time(End::Start))
            .map_or(sample.leg, |_| sample.leg.incremented());
        (usize::from(begun)..legs.len())
            .map(LegIndex::new)
            .find(|index| {
                self.day
                    .leg(*index)
                    .is_some_and(|leg| leg.mode() != Mode::Walk || leg.tour().is_some())
            })
    }

    pub fn closing_spare(&self, closer: Option<LegIndex>, time: ClockTime) -> Option<Minutes> {
        let Some(index) = closer else {
            let last = self.day.legs().last()?.time(End::Finish);
            return Some(self.day.night()?.check_in_by().since_midnight() - last.since_midnight());
        };
        let leg = self.day.leg(index)?;
        let before = index.previous().and_then(|before| self.day.leg(before));
        let visit = before
            .filter(|before| before.mode() == Mode::Walk)
            .map(|before| {
                before
                    .time(End::Finish)
                    .later_by(before.paused(Pause::AtEnd))
            })
            .filter(|leaves| *leaves < leg.time(End::Start));
        let arrival = visit
            .or(before.map(|leg| leg.time(End::Finish)))
            .or(self.day.leave_from())
            .unwrap_or(leg.time(End::Start));
        let target = leg
            .boarding_margin()
            .map(|margin| margin.target(leg.time(End::Start)))
            .or_else(|| leg.tour().map(|tour| ClockTime::from(tour.check_in())))?;
        Some(if time >= arrival {
            leg.time(End::Start).since_midnight() - time.since_midnight()
        } else {
            target.since_midnight() - arrival.since_midnight()
        })
    }

    pub fn planned_spare(&self, sample: &Sample) -> Option<Minutes> {
        let closer = self.closer(sample);
        let end = closer.map_or(self.day.legs().len(), usize::from);
        let cutoffs: Vec<Minutes> = (usize::from(sample.leg)..end)
            .filter_map(|position| self.day.legs().get(position))
            .flat_map(|leg| {
                let own = leg
                    .cutoff()
                    .filter(|_| leg.time(End::Finish) >= sample.time)
                    .map(|cutoff| {
                        cutoff.deadline().since_midnight() - leg.time(End::Finish).since_midnight()
                    });
                let stops = leg.along().iter().filter_map(|stop| {
                    let arrival = stop
                        .expected()
                        .map(ClockTime::from)
                        .filter(|arrival| *arrival >= sample.time)?;
                    Some(stop.cutoff()?.deadline().since_midnight() - arrival.since_midnight())
                });
                own.into_iter().chain(stops).collect::<Vec<Minutes>>()
            })
            .collect();
        cutoffs
            .into_iter()
            .chain(self.closing_spare(closer, sample.time))
            .min()
    }

    pub fn live(&self, sample: &Sample) -> Guidance<'day> {
        let at = |time: ClockTime| TripTime::on_date_at(self.day.zone(), self.day.date(), time);
        let halt = sample
            .since
            .zip(sample.here)
            .map(|(since, here)| Halt::tracked(None, here, at(since)));
        let progress = LegProgress::confirmed(self.day.number(), sample.leg, Minutes::ZERO);
        Guidance::new(self.day, progress, at(sample.time), sample.here).paused(halt)
    }

    pub fn live_buffer(&self, sample: &Sample) -> Option<Buffer<'day>> {
        self.live(sample).buffer()
    }

    pub fn mismatches(&self) -> Vec<String> {
        self.samples()
            .iter()
            .filter_map(|sample| {
                let planned = self.planned_spare(sample)?;
                let live = self.live_buffer(sample).map(|buffer| buffer.spare())?;
                ((planned - live).abs() > ON_PLAN || live < Minutes::ZERO).then(|| {
                    format!(
                        "day {} leg {} at {}: planned {planned}, shown {live}",
                        self.day.number(),
                        usize::from(sample.leg),
                        sample.time.since_midnight()
                    )
                })
            })
            .collect()
    }
}

#[test]
fn a_walker_on_plan_is_shown_the_planned_spare_and_never_short() {
    let trip = Fixture::bundled().unwrap();
    let wrong: Vec<String> = trip
        .days()
        .par_iter()
        .flat_map_iter(|day| {
            let audit = Audit { day };
            audit
                .mismatches()
                .into_iter()
                .chain(audit.stranded())
                .chain(audit.late_mismatches())
                .chain(audit.aboard_mismatches())
        })
        .collect();
    assert_eq!(wrong, Vec::<String>::new());
}
