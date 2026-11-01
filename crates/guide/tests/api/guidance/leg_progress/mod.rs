use crate::fixture::Fixture;
use std::time::UNIX_EPOCH;

use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::geo::Coordinate;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::guidance::LegProgress;
use guide::itinerary::DayNumber;
use guide::leg::LegIndex;

const DAY: u8 = 8;
const LEG: usize = 15;
const LATE: i64 = 12;
const ALONG: Distance = Distance::new(230.0);
const SEEN: Coordinate = Coordinate::new(Degrees::new(38.5), Degrees::new(140.25));
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
