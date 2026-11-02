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

#[test]
fn a_manual_set_is_never_moved_back_by_what_the_tracker_sees() {
    for sequence in Sequences::of_length(SEQUENCE_LENGTH) {
        sequence
            .iter()
            .fold(Tracking::Automatic, |tracking, fix| match *fix {
                Fix::ByHand(placing, baseline) => {
                    let pinned = Tracking::pinned(placing, baseline);
                    assert_eq!(pinned.floor(), Some(placing));
                    pinned
                }
                Fix::Observe(seen, source) => {
                    let observed = tracking.observing(seen, source);
                    assert!(observed.floor() >= tracking.floor());
                    if let Some(floor) = tracking.floor() {
                        assert_eq!(observed.floor(), Some(floor.max(seen)));
                        assert!(tracking.holding(seen, source).unwrap_or(seen) >= floor);
                    }
                    observed
                }
            });
    }
}

#[test]
fn an_automatic_tracker_stays_automatic_whatever_it_sees() {
    for seen in Placings::all() {
        assert_eq!(
            Tracking::Automatic.observing(seen, PositionSource::Gps),
            Tracking::Automatic
        );
        assert_eq!(Tracking::Automatic.holding(seen, PositionSource::Gps), None);
    }
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

#[test]
fn guidance_never_shows_a_step_before_the_manual_set_and_follows_the_tracker_past_it() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    for floor in Placings::on_the_day() {
        for clock in CLOCKS {
            let now = Clock::on(day, clock);
            for leg in TRACKER_LEGS {
                let kept = LegProgress::confirmed(day.number(), LegIndex::new(leg), Minutes::ZERO);
                let reading = Guidance::new(day, kept, now, None);
                let pinned = LegProgress::pinned_at(day, floor, now, reading.placing());
                let held = reading.held(pinned);
                assert!(held.placing() >= floor, "{floor} at {clock}");
                if reading.placing() >= floor {
                    assert_eq!(held.placing(), reading.placing());
                }
            }
        }
    }
}

#[test]
fn a_manual_set_holds_through_a_confirmed_step_that_would_go_back() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let now = Clock::on(day, CLOCKS[0]);
    for floor in Placings::on_the_day() {
        let pinned = LegProgress::pinned_at(day, floor, now, floor);
        let behind = LegProgress::confirmed(
            day.number(),
            LegIndex::new(DAY_FIRST_LEG - 1),
            Minutes::ZERO,
        );
        let kept = Guidance::new(day, behind, now, None).confirmed(pinned);
        assert!(kept.tracking().floor() >= Some(floor));
        let shown = Guidance::new(day, kept, now, None).held(kept);
        assert!(shown.placing() >= floor);
    }
}

#[test]
fn a_manual_set_reads_back_as_it_was_written() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let now = Clock::on(day, CLOCKS[2]);
    for floor in Placings::on_the_day() {
        let pinned = LegProgress::pinned_at(day, floor, now, Placings::on_the_day()[0]);
        assert_eq!(pinned.to_string().parse::<LegProgress>(), Ok(pinned));
    }
}
