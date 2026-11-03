use crate::fixture::Fixture;
use crate::walker::Walker;
use std::time::UNIX_EPOCH;

use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::day_number::DayNumber;
use guide::end::End;
use guide::geo::Coordinate;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::guidance::LegProgress;
use guide::guidance::Phase;
use guide::leg::LegIndex;

const DAY: u8 = 8;
const LEG: usize = 15;
const LATE: i64 = 12;
const ALONG: Distance = Distance::new(230.0);
const SEEN: Coordinate = Coordinate::new(Degrees::new(38.5), Degrees::new(140.25));
const EVENINGS: [usize; 2] = [1, 3];
const INSIDE: Distance = Distance::new(15.0);
const INN_DOOR: End = End::Finish;
const OLDER_FORMAT: &str = "day 8 leg 16 behind 30 started 12";

#[test]
fn kept_progress_reads_back_as_it_was_written() {
    let kept = LegProgress::confirmed(DayNumber::new(DAY), LegIndex::new(LEG), Minutes::new(LATE))
        .placed(ALONG, TripTime::at_instant(UNIX_EPOCH, Fixture::zone()))
        .seen_at(SEEN);
    assert_eq!(kept.to_string().parse::<LegProgress>(), Ok(kept));
    assert!(
        LegProgress::default()
            .to_string()
            .parse::<LegProgress>()
            .is_err()
    );
}

#[test]
fn progress_kept_with_two_delays_reads_back_as_the_offset_at_the_step_start() {
    assert_eq!(
        OLDER_FORMAT.parse::<LegProgress>().ok(),
        Some(LegProgress::confirmed(
            DayNumber::new(DAY),
            LegIndex::new(LEG),
            Minutes::new(LATE)
        ))
    );
}

#[test]
fn at_the_inn_the_day_is_done_even_hours_later() {
    let trip = Fixture::bundled_days(&EVENINGS);
    let busy: Vec<String> = EVENINGS
        .iter()
        .filter_map(|number| {
            let day = trip.day(*number);
            let last = LegIndex::new(day.legs().len() - 1);
            let inn = day.leg(last).unwrap();
            let night = ClockTime::try_from("22:00".to_owned()).unwrap();
            let walker = Walker::fresh(day).strolled(last, inn.length(), Minutes::ZERO);
            let guidance = walker.fix(night, inn.point(INN_DOOR).offset_by(INSIDE, INSIDE));
            (guidance.phase() != Phase::Done || guidance.recovery().is_some())
                .then(|| format!("day {number}: {:?}", guidance.status()))
        })
        .collect();
    assert_eq!(busy, Vec::<String>::new());
}
