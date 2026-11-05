use crate::fixture::Fixture;
use guide::clock::{ClockTime, Minutes, TripTime};
use guide::guidance::tracking::{Placing, Stage, Tracking};
use guide::guidance::{Guidance, LegProgress, PositionSource};
use guide::itinerary::Day;
use guide::leg::LegIndex;

const HILL_DAY: usize = 3;
const DAY_FIRST_LEG: usize = 1;
const DAY_LAST_LEG: usize = 15;
const TRACKER_LEGS: [usize; 3] = [3, 7, 11];
const CLOCKS: [&str; 4] = ["07:30", "10:00", "12:30", "15:00"];
const FIRST_LEG: usize = 8;
const LAST_LEG: usize = 20;
const STAGES: [Stage; 2] = [Stage::Boarding, Stage::Under];
const SEQUENCE_LENGTH: usize = 3;
const SOURCES: [PositionSource; 2] = [PositionSource::Gps, PositionSource::Estimate];

#[derive(Clone, Copy, Debug)]
enum Fix {
    Observe(Placing, PositionSource),
    ByHand(Placing, Placing),
}

struct Placings;

impl Placings {
    fn all() -> Vec<Placing> {
        (FIRST_LEG..LAST_LEG)
            .flat_map(|leg| {
                STAGES
                    .iter()
                    .map(move |stage| Placing::new(LegIndex::new(leg), *stage))
            })
            .collect()
    }

    fn on_the_day() -> Vec<Placing> {
        (DAY_FIRST_LEG..DAY_LAST_LEG)
            .flat_map(|leg| {
                STAGES
                    .iter()
                    .map(move |stage| Placing::new(LegIndex::new(leg), *stage))
            })
            .collect()
    }

    fn few() -> Vec<Placing> {
        Self::all().into_iter().step_by(5).collect()
    }
}

struct Sequences;

impl Sequences {
    fn of_length(length: usize) -> Vec<Vec<Fix>> {
        let few = Placings::few();
        let alphabet: Vec<Fix> = few
            .iter()
            .flat_map(|placing| {
                SOURCES
                    .iter()
                    .map(|source| Fix::Observe(*placing, *source))
                    .chain(few.iter().map(|baseline| Fix::ByHand(*placing, *baseline)))
            })
            .collect();
        (0..length).fold(vec![Vec::new()], |sequences, _| {
            sequences
                .iter()
                .flat_map(|sequence| {
                    alphabet.iter().map(move |fix| {
                        let mut longer = sequence.clone();
                        longer.push(*fix);
                        longer
                    })
                })
                .collect()
        })
    }
}

struct Floors {
    reached: Option<Placing>,
    wanted: Option<Placing>,
}

struct Holding {
    held: Option<Placing>,
    floor: Option<Placing>,
}

struct Step {
    fix: Fix,
    before: Tracking,
}

impl Step {
    fn report(&self) -> String {
        format!("{:?} after {:?}", self.fix, self.before)
    }
}

impl Fix {
    fn after(self, before: Tracking) -> Tracking {
        match self {
            Fix::ByHand(placing, baseline) => Tracking::pinned(placing, baseline),
            Fix::Observe(seen, source) => before.observing(seen, source),
        }
    }

    fn floors(self, before: Tracking) -> Floors {
        match self {
            Fix::ByHand(placing, _) => Floors {
                reached: self.after(before).floor(),
                wanted: Some(placing),
            },
            Fix::Observe(seen, _) => Floors {
                reached: self.after(before).floor(),
                wanted: before.floor().map(|floor| floor.max(seen)),
            },
        }
    }

    fn held(self, before: Tracking) -> Holding {
        match self {
            Fix::ByHand(..) => Holding {
                held: None,
                floor: None,
            },
            Fix::Observe(seen, source) => Holding {
                held: Some(before.holding(seen, source).unwrap_or(seen)),
                floor: before.floor(),
            },
        }
    }

    fn steps(sequence: &[Fix]) -> Vec<Step> {
        sequence
            .iter()
            .scan(Tracking::Automatic, |tracking, fix| {
                let before = *tracking;
                *tracking = fix.after(before);
                Some(Step { fix: *fix, before })
            })
            .collect()
    }
}

#[test]
fn a_manual_set_is_never_moved_back_by_what_the_tracker_sees() {
    let steps: Vec<Step> = Sequences::of_length(SEQUENCE_LENGTH)
        .iter()
        .flat_map(|sequence| Fix::steps(sequence))
        .collect();
    let misplaced: Vec<String> = steps
        .iter()
        .filter(|step| {
            let floors = step.fix.floors(step.before);
            floors.reached != floors.wanted
        })
        .map(Step::report)
        .collect();
    let slipped: Vec<String> = steps
        .iter()
        .filter(|step| {
            let holding = step.fix.held(step.before);
            holding.held < holding.floor
        })
        .map(Step::report)
        .collect();
    assert_eq!(misplaced, Vec::<String>::new());
    assert_eq!(slipped, Vec::<String>::new());
}

#[test]
fn an_automatic_tracker_stays_automatic_whatever_it_sees() {
    let moved: Vec<Placing> = Placings::all()
        .into_iter()
        .filter(|seen| {
            Tracking::Automatic.observing(*seen, PositionSource::Gps) != Tracking::Automatic
        })
        .collect();
    let holding: Vec<Placing> = Placings::all()
        .into_iter()
        .filter(|seen| {
            Tracking::Automatic
                .holding(*seen, PositionSource::Gps)
                .is_some()
        })
        .collect();
    assert_eq!(moved, Vec::<Placing>::new());
    assert_eq!(holding, Vec::<Placing>::new());
}

#[test]
fn a_gps_reading_at_or_below_where_the_set_was_made_is_ignored_even_below_the_set() {
    let baseline = Placings::all()[6];
    let floor = Placings::all()[2];
    let pinned = Tracking::pinned(floor, baseline);
    assert_eq!(pinned.holding(baseline, PositionSource::Gps), Some(floor));
    assert_eq!(pinned.holding(baseline, PositionSource::Estimate), None);
    assert_eq!(
        pinned.holding(Placings::all()[7], PositionSource::Gps),
        None
    );
}

struct Clock;

impl Clock {
    fn on(day: &Day, written: &str) -> TripTime {
        let time = ClockTime::try_from(written.to_owned()).expect("a written time");
        TripTime::on_date_at(day.zone(), day.date(), time)
    }
}

struct Shown {
    floor: Placing,
    clock: &'static str,
    reading: Placing,
    held: Placing,
}

impl Shown {
    fn at(day: &Day, floor: Placing, clock: &'static str, leg: usize) -> Self {
        let now = Clock::on(day, clock);
        let kept = LegProgress::confirmed(day.number(), LegIndex::new(leg), Minutes::ZERO);
        let reading = Guidance::new(day, kept, now, None);
        let pinned = LegProgress::pinned_at(day, floor, now, reading.placing());
        Self {
            floor,
            clock,
            reading: reading.placing(),
            held: reading.held(pinned).placing(),
        }
    }

    fn report(&self) -> String {
        format!(
            "{} at {}: reading {}, shown {}",
            self.floor, self.clock, self.reading, self.held
        )
    }
}

#[test]
fn guidance_never_shows_a_step_before_the_manual_set_and_follows_the_tracker_past_it() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let shown: Vec<Shown> = Placings::on_the_day()
        .into_iter()
        .flat_map(|floor| {
            CLOCKS
                .into_iter()
                .flat_map(move |clock| TRACKER_LEGS.into_iter().map(move |leg| (floor, clock, leg)))
        })
        .map(|(floor, clock, leg)| Shown::at(day, floor, clock, leg))
        .collect();
    let before_the_set: Vec<String> = shown
        .iter()
        .filter(|shown| shown.held < shown.floor)
        .map(Shown::report)
        .collect();
    let behind_the_tracker: Vec<String> = shown
        .iter()
        .filter(|shown| shown.reading >= shown.floor)
        .filter(|shown| shown.held != shown.reading)
        .map(Shown::report)
        .collect();
    assert_eq!(before_the_set, Vec::<String>::new());
    assert_eq!(behind_the_tracker, Vec::<String>::new());
}

#[test]
fn a_manual_set_holds_through_a_confirmed_step_that_would_go_back() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let now = Clock::on(day, CLOCKS[0]);
    let held: Vec<(Placing, Option<Placing>, Placing)> = Placings::on_the_day()
        .into_iter()
        .map(|floor| {
            let pinned = LegProgress::pinned_at(day, floor, now, floor);
            let behind = LegProgress::confirmed(
                day.number(),
                LegIndex::new(DAY_FIRST_LEG - 1),
                Minutes::ZERO,
            );
            let kept = Guidance::new(day, behind, now, None).confirmed(pinned);
            let shown = Guidance::new(day, kept, now, None).held(kept);
            (floor, kept.tracking().floor(), shown.placing())
        })
        .collect();
    let unkept: Vec<Placing> = held
        .iter()
        .filter(|(floor, kept, _)| *kept < Some(*floor))
        .map(|(floor, ..)| *floor)
        .collect();
    let unshown: Vec<Placing> = held
        .iter()
        .filter(|(floor, _, shown)| shown < floor)
        .map(|(floor, ..)| *floor)
        .collect();
    assert_eq!(unkept, Vec::<Placing>::new());
    assert_eq!(unshown, Vec::<Placing>::new());
}

#[test]
fn a_manual_set_reads_back_as_it_was_written() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let now = Clock::on(day, CLOCKS[2]);
    let changed: Vec<Placing> = Placings::on_the_day()
        .into_iter()
        .filter(|floor| {
            let pinned = LegProgress::pinned_at(day, *floor, now, Placings::on_the_day()[0]);
            pinned.to_string().parse::<LegProgress>() != Ok(pinned)
        })
        .collect();
    assert_eq!(changed, Vec::<Placing>::new());
}
