use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::leg::LegIndex;
use guide::readout::Status;

const DAY_THREE: usize = 3;
const DAY_FOUR: usize = 4;
const STATION: &str = "Fern station";
const LANDING: &str = "the boat landing";
const SHORT_OF_THE_END: Distance = Distance::new(60.0);
const EARLY: Minutes = Minutes::new(20);
const SHUTTLE: LegIndex = LegIndex::new(4);
const BUS_RUNS_LATE: Minutes = Minutes::new(2);
const FIRST_RIDE: LegIndex = LegIndex::new(0);
const AFTER_FIRST_RIDE_LEFT: &str = "07:45";
const QUIET_HOURS: [&str; 2] = ["06:10", "09:20"];

#[test]
fn near_the_station_the_walker_waits_with_an_arrow_and_waits_for_a_late_bus() {
    let trip = Fixture::bundled_days(&[DAY_THREE, DAY_FOUR]);
    let day = trip.day(DAY_THREE);
    let short = |walk: LegIndex| {
        let along = day.leg(walk).unwrap().length() - SHORT_OF_THE_END;
        Walker::fresh(day)
            .strolled(walk, along, -EARLY)
            .fix_along(walk, along, -EARLY)
    };
    let four = trip.day(DAY_FOUR);
    let bus = four.leg(SHUTTLE).unwrap();
    let leaves = bus.time(End::Start);
    let kept = LegProgress::confirmed(four.number(), SHUTTLE, Minutes::ZERO);
    let late = leaves.later_by(BUS_RUNS_LATE);
    let stop = Walker::kept(four, kept).fix(late, bus.boarding().unwrap());
    assert!(matches!(
        short(Walker::reaching(day, STATION)).status(),
        Some(Status::Waiting { .. })
    ));
    assert!(short(Walker::reaching(day, LANDING)).arrow().is_some());
    assert_eq!(
        stop.status(),
        Some(Status::Waiting {
            until: bus.listed(End::Start)
        })
    );
}

#[test]
fn at_the_stop_after_the_first_ride_left_it_is_missed_and_the_spare_is_short() {
    let trip = Fixture::bundled_days(&[DAY_FOUR]);
    let day = trip.day(DAY_FOUR);
    let ride = day.leg(FIRST_RIDE).unwrap();
    let later = ClockTime::try_from(AFTER_FIRST_RIDE_LEFT.to_owned()).unwrap();
    let guidance = Walker::fresh(day).fix(later, ride.boarding().unwrap());
    assert_eq!(guidance.missed_ride(), Some(FIRST_RIDE));
    assert!(
        guidance
            .buffer()
            .is_some_and(|buffer| buffer.spare() < Minutes::ZERO)
    );
}

#[test]
fn without_gps_between_walking_steps_the_status_says_no_gps() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let shown: Vec<Option<Status>> = QUIET_HOURS
        .iter()
        .map(|hour| {
            let now = TripTime::on_date_at(
                day.zone(),
                day.date(),
                ClockTime::try_from((*hour).to_owned()).unwrap(),
            );
            Guidance::new(day, LegProgress::default(), now, None).status()
        })
        .collect();
    assert_eq!(shown, vec![Some(Status::NoGps(None)); QUIET_HOURS.len()]);
}
