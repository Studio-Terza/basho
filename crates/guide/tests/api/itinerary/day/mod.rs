mod goal;

use std::cmp::Ordering;
use std::time::SystemTime;

use chrono::NaiveDate;

use crate::fixture::Fixture;
use newtype::Counter;
use rayon::prelude::*;

use guide::clock::CalendarDate;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::Pause;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Coordinate;
use guide::guidance::ArrivalMark;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::timeline::Buffer;
use guide::guidance::timeline::Halt;
use guide::guidance::timeline::Stop;
use guide::itinerary::Day;
use guide::leg::Leg;
use guide::leg::LegIndex;
use guide::leg::Mode;

const ON_PLAN: Minutes = Minutes::new(1);
const LATE_TRAIN: Minutes = Minutes::new(10);
const LATENESS: [Minutes; 3] = [Minutes::new(15), Minutes::new(45), Minutes::new(60 + 30)];
const FIRST_DAY: usize = 1;
const RIDES_DAY: usize = 4;
const HOURS_PER_DAY: u16 = 24;
const READY_UTC_HOUR: u32 = 11;
const READY_UTC_MINUTE: u32 = 22;
const READY_SPARE: Minutes = Minutes::new(8);

#[derive(Debug, PartialEq)]
enum Kind {
    Walk,
    Tour,
    Ride,
}

impl Kind {
    fn of(leg: &Leg) -> Self {
        match leg.mode() {
            Mode::Walk => Self::on_foot(leg),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Kind::Ride,
        }
    }

    fn on_foot(leg: &Leg) -> Self {
        match leg.tour() {
            Some(_) => Kind::Tour,
            None => Kind::Walk,
        }
    }
}

struct Visit;

impl Visit {
    fn leaves(before: &Leg, ride: &Leg) -> Option<ClockTime> {
        match before.mode() {
            Mode::Walk => Self::after_pause(before, ride),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => None,
        }
    }

    fn after_pause(before: &Leg, ride: &Leg) -> Option<ClockTime> {
        Self::early(
            before
                .time(End::Finish)
                .later_by(before.paused(Pause::AtEnd)),
            ride.time(End::Start),
        )
    }

    fn early(leaves: ClockTime, departs: ClockTime) -> Option<ClockTime> {
        match leaves.cmp(&departs) {
            Ordering::Less => Some(leaves),
            Ordering::Equal | Ordering::Greater => None,
        }
    }
}

struct Landing {
    label: String,
    early: Minutes,
    late: Minutes,
    riding: LegIndex,
    of: LegIndex,
    shown: Option<ClockTime>,
    expected: Option<ClockTime>,
}

impl Landing {
    fn failures(landings: &[Landing]) -> Vec<String> {
        landings
            .iter()
            .filter(|landing| landing.riding == landing.of)
            .filter(|landing| landing.early < landing.late)
            .filter(|landing| landing.shown != landing.expected)
            .map(|landing| {
                format!(
                    "{}: inn shown {:?}, expected {:?}",
                    landing.label, landing.shown, landing.expected
                )
            })
            .collect()
    }
}

struct Stranded {
    label: String,
    event: ClockTime,
    departs: ClockTime,
    spare: Minutes,
}

impl Stranded {
    fn failures(strandings: &[Stranded]) -> Vec<String> {
        strandings
            .iter()
            .filter(|stranded| stranded.event == stranded.departs)
            .filter(|stranded| stranded.spare >= Minutes::ZERO)
            .map(|stranded| {
                format!(
                    "{}: {} to spare after it left",
                    stranded.label, stranded.spare
                )
            })
            .collect()
    }
}

struct Shown {
    label: String,
    planned: Minutes,
    live: Minutes,
}

impl Shown {
    fn failures(shown: &[Shown]) -> Vec<String> {
        let drifted = shown
            .iter()
            .filter(|shown| (shown.planned - shown.live).abs() > ON_PLAN);
        let short = shown.iter().filter(|shown| shown.live < Minutes::ZERO);
        drifted
            .chain(short)
            .map(|shown| {
                format!(
                    "{}: planned {}, shown {}",
                    shown.label, shown.planned, shown.live
                )
            })
            .collect()
    }
}

struct Closing {
    starts: ClockTime,
    target: ClockTime,
    arrival: ClockTime,
}

impl Closing {
    fn spare(&self, time: ClockTime) -> Minutes {
        match time.cmp(&self.arrival) {
            Ordering::Less => self.planned(),
            Ordering::Equal | Ordering::Greater => self.at(time),
        }
    }

    fn planned(&self) -> Minutes {
        self.target.since_midnight() - self.arrival.since_midnight()
    }

    fn at(&self, time: ClockTime) -> Minutes {
        self.starts.since_midnight() - time.since_midnight()
    }
}

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
                let from = Visit::leaves(leg, ride)
                    .unwrap_or(leg.time(End::Finish))
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
        let visit = before.and_then(|before| Visit::leaves(before, leg));
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
        self.day
            .leg(ride)
            .and_then(|leg| self.boarded(ride, leg, arrival, waiting))
    }

    pub fn boarded(
        &self,
        ride: LegIndex,
        leg: &Leg,
        arrival: Minutes,
        waiting: Option<Minutes>,
    ) -> Option<Minutes> {
        match arrival.cmp(&leg.time(End::Start).since_midnight()) {
            Ordering::Greater => self.after_backup(ride, leg, arrival, waiting),
            Ordering::Less | Ordering::Equal => self.day_end(),
        }
    }

    pub fn day_end(&self) -> Option<Minutes> {
        Some(self.day.legs().last()?.time(End::Finish).since_midnight())
    }

    fn after_backup(
        &self,
        ride: LegIndex,
        leg: &Leg,
        arrival: Minutes,
        waiting: Option<Minutes>,
    ) -> Option<Minutes> {
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
        let landings: Vec<Landing> = self
            .day
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
                    Some(Landing {
                        label: format!(
                            "day {} {} late {late}",
                            self.day.number(),
                            ride.listed(End::Start)
                        ),
                        early: ride.time(End::Start).since_midnight(),
                        late: arrival,
                        riding: index,
                        of: index,
                        shown,
                        expected,
                    })
                })
            })
            .collect();
        Landing::failures(&landings)
    }

    pub fn aboard_mismatches(&self) -> Vec<String> {
        let landings: Vec<Landing> = self
            .day
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
                let shown = guidance
                    .arrival()
                    .and_then(|arrival| arrival.time(ArrivalMark::Arrives));
                let end = self.run_end(index);
                let landed = self.day.leg(end)?.time(End::Finish).since_midnight();
                let expected = self
                    .delayed(
                        end.incremented(),
                        backup.time(End::Finish).since_midnight() - landed,
                    )
                    .map(|minutes| self.clock_of(minutes));
                Some(Landing {
                    label: format!(
                        "day {} on the {} backup",
                        self.day.number(),
                        backup.listed(End::Start)
                    ),
                    early: middle,
                    late: backup.time(End::Finish).since_midnight(),
                    riding: guidance.leg_index(),
                    of: index,
                    shown,
                    expected,
                })
            })
            .collect();
        Landing::failures(&landings)
    }

    pub fn stranded(&self) -> Vec<String> {
        let strandings: Vec<Stranded> = self
            .day
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
                Some(Stranded {
                    label: format!("day {} {}", self.day.number(), ride.listed(End::Start)),
                    event: buffer.deadline().event(),
                    departs: ride.time(End::Start),
                    spare: buffer.spare(),
                })
            })
            .collect();
        Stranded::failures(&strandings)
    }

    pub fn closer(&self, sample: &Sample) -> Option<LegIndex> {
        let legs = self.day.legs();
        let counts = |leg: &Leg| Kind::of(leg) != Kind::Walk;
        let begun = legs
            .get(usize::from(sample.leg))
            .filter(|leg| counts(leg))
            .filter(|leg| sample.time >= leg.time(End::Start))
            .map_or(sample.leg, |_| sample.leg.incremented());
        legs.iter()
            .enumerate()
            .skip(usize::from(begun))
            .find(|(_, leg)| counts(leg))
            .map(|(index, _)| LegIndex::new(index))
    }

    pub fn closing_spare(&self, closer: Option<LegIndex>, time: ClockTime) -> Option<Minutes> {
        closer.map_or_else(
            || self.night_spare(),
            |index| self.closing_spare_at(index, time),
        )
    }

    fn night_spare(&self) -> Option<Minutes> {
        let last = self.day.legs().last()?.time(End::Finish);
        Some(self.day.night()?.check_in_by().since_midnight() - last.since_midnight())
    }

    fn closing_spare_at(&self, index: LegIndex, time: ClockTime) -> Option<Minutes> {
        let leg = self.day.leg(index)?;
        let before = index.previous().and_then(|before| self.day.leg(before));
        let visit = before.and_then(|before| Visit::leaves(before, leg));
        let arrival = visit
            .or(before.map(|leg| leg.time(End::Finish)))
            .or(self.day.leave_from())
            .unwrap_or(leg.time(End::Start));
        let target = leg
            .boarding_margin()
            .map(|margin| margin.target(leg.time(End::Start)))
            .or_else(|| leg.tour().map(|tour| ClockTime::from(tour.check_in())))?;
        Some(
            Closing {
                starts: leg.time(End::Start),
                target,
                arrival,
            }
            .spare(time),
        )
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
        let shown: Vec<Shown> = self
            .samples()
            .iter()
            .filter_map(|sample| {
                Some(Shown {
                    label: format!(
                        "day {} leg {} at {}",
                        self.day.number(),
                        usize::from(sample.leg),
                        sample.time.since_midnight()
                    ),
                    planned: self.planned_spare(sample)?,
                    live: self.live_buffer(sample).map(|buffer| buffer.spare())?,
                })
            })
            .collect();
        Shown::failures(&shown)
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

#[test]
fn the_day_before_the_trip_shows_no_buffer_and_no_lateness() {
    let trip = Fixture::bundled_days(&[FIRST_DAY]);
    let first = trip.day(FIRST_DAY);
    let here = first.legs().first().map(|leg| leg.point(End::Start));
    let eve = NaiveDate::from(first.date())
        .pred_opt()
        .map(CalendarDate::from)
        .unwrap();
    let guidances: Vec<(TripTime, Guidance)> = (0..HOURS_PER_DAY)
        .map(|hour| TripTime::on_date_at(first.zone(), eve, ClockTime::at_hour(hour)))
        .map(|now| (now, Guidance::new(first, LegProgress::default(), now, here)))
        .collect();
    let buffered: Vec<TripTime> = guidances
        .iter()
        .filter(|(_, guidance)| guidance.buffer().is_some())
        .map(|(now, _)| *now)
        .collect();
    let checked: Vec<TripTime> = guidances
        .iter()
        .filter(|(_, guidance)| guidance.time_check().is_some())
        .map(|(now, _)| *now)
        .collect();
    assert_eq!(buffered, Vec::<TripTime>::new());
    assert_eq!(checked, Vec::<TripTime>::new());
}

#[test]
fn a_utc_moment_at_the_first_bus_shows_its_spare_in_trip_time() {
    let trip = Fixture::bundled_days(&[RIDES_DAY]);
    let day = trip.day(RIDES_DAY);
    let bus = day.legs().first().unwrap();
    let utc = NaiveDate::from(day.date())
        .and_hms_opt(READY_UTC_HOUR, READY_UTC_MINUTE, u32::default())
        .unwrap()
        .and_utc();
    let now = TripTime::at_instant(SystemTime::from(utc), day.zone());
    let guidance = Guidance::new(day, LegProgress::default(), now, bus.boarding());
    let buffer = guidance.buffer().unwrap();
    assert_eq!(buffer.deadline().event(), bus.time(End::Start));
    assert_eq!(buffer.spare(), READY_SPARE);
}
