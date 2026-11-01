use crate::fixture::Fixture;
use std::time::UNIX_EPOCH;

use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::geo::Coordinate;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::timeline::Halt;

const START: Coordinate = Coordinate::new(Degrees::new(38.0), Degrees::new(140.0));
const WALK: Distance = Distance::new(80.0);
const LATER: Minutes = Minutes::new(5);
const JITTER: [Distance; 6] = [
    Distance::new(25.0),
    Distance::new(-15.0),
    Distance::new(30.0),
    Distance::new(-25.0),
    Distance::new(5.0),
    Distance::new(-30.0),
];

#[test]
fn a_walker_who_stays_put_through_thirty_metre_gps_jitter_has_stopped() {
    let first = TripTime::at_instant(UNIX_EPOCH, Fixture::zone());
    let later = first.later_by(LATER);
    let halt = Halt::tracked(None, START, first);
    let sitting = JITTER.iter().zip(JITTER.iter().rev()).enumerate().fold(
        halt,
        |halt, (minute, (east, north))| {
            let minute = Minutes::new(i64::try_from(minute).unwrap());
            let now = first.later_by(minute);
            Halt::tracked(Some(halt), START.offset_by(*east, *north), now)
        },
    );
    let walked = Halt::tracked(Some(halt), START.offset_by(WALK, WALK), later);
    assert_eq!(sitting.still_for(later), Some(LATER));
    assert_eq!(walked.still_for(later), None);
}
