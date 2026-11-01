use guide::guidance::PositionSource;
use guide::guidance::tracking::{Placing, Stage, Tracking};
use guide::leg::LegIndex;

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
