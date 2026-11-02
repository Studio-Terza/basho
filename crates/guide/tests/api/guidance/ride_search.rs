use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::end::End;
use guide::geo::Distance;
use guide::geo::Share;
use guide::guidance::Phase;
use guide::leg::LegIndex;
use guide::readout::Status;
use newtype::Counter;

const DAY_THREE: usize = 3;
const STATION_ROAD: LegIndex = LegIndex::new(6);
const FERN_TRAIN: LegIndex = LegIndex::new(7);
const PULLING_OUT: Distance = Distance::new(200.0);
const NEAR_THE_STATION: Distance = Distance::new(300.0);
const BESIDE_THE_LINE: [usize; 2] = [2, 4];
const STATIONS: [&str; 2] = ["green", "Elm Central station"];
const SINCE_DEPARTURE: Minutes = Minutes::new(4);
const AFTER_RIDES: [usize; 3] = [3, 3, 4];
const ALIGHTING: [&str; 3] = ["Hollow", "lower landing", "hall stop"];
const BEFORE_DEPARTURE: Minutes = Minutes::new(3);
const WAITING: &str = "11:10";
const JUST_LEFT: &str = "11:18";

#[test]
fn the_first_fixes_of_a_train_leaving_the_station_are_riding_not_missed() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let ride = day.leg(FERN_TRAIN).unwrap();
    let waiting = ClockTime::try_from(WAITING.to_owned()).unwrap();
    let station = ride.boarding().unwrap();
    let moving = ride.line().point_along(PULLING_OUT).unwrap();
    let guidance = Walker::walked(day, STATION_ROAD, waiting)
        .stepped(waiting, station)
        .fix(ClockTime::try_from(JUST_LEFT.to_owned()).unwrap(), moving);
    assert!(moving.distance_to(station) < NEAR_THE_STATION);
    assert_eq!(guidance.missed_ride(), None);
    assert!(guidance.leg_index() == FERN_TRAIN && guidance.phase() == Phase::Moving);
}

#[test]
fn lost_beside_the_railway_is_off_route_with_an_arrow_before_and_after_the_train() {
    let trip = Fixture::bundled_days(&BESIDE_THE_LINE);
    let lost: Vec<_> = BESIDE_THE_LINE
        .iter()
        .zip(STATIONS)
        .flat_map(|(number, station)| {
            let day = trip.day(*number);
            Walker::lost_by_the_line(day, station, &[-BEFORE_DEPARTURE, SINCE_DEPARTURE])
        })
        .collect();
    let wrong: Vec<String> = lost
        .iter()
        .filter(|guidance| {
            guidance
                .off_route()
                .and(guidance.arrow())
                .and(guidance.recovery())
                .is_none()
        })
        .map(|guidance| format!("{} {:?}", guidance.leg_index(), guidance.status()))
        .collect();
    assert_eq!(lost.len(), BESIDE_THE_LINE.len() * 2);
    assert_eq!(wrong, Vec::<String>::new());
}

#[test]
fn after_the_ride_the_walk_from_the_stop_is_walking_not_riding() {
    let trip = Fixture::bundled_days(&AFTER_RIDES);
    let wrong: Vec<String> = AFTER_RIDES
        .iter()
        .zip(ALIGHTING)
        .filter_map(|(number, place)| {
            let day = trip.day(*number);
            let ride = Walker::reaching(day, place);
            let leg = day.leg(ride).unwrap();
            let walker = Walker::fresh(day).stepped(
                leg.time(End::Start).later_by(-BEFORE_DEPARTURE),
                leg.boarding().unwrap(),
            );
            let walk = ride.incremented();
            let along = day.leg(walk).unwrap().length() * Share::HALF;
            let guidance = walker.fix_along(walk, along, Minutes::ZERO);
            (guidance.leg_index() != walk || guidance.status() != Some(Status::OnRoute))
                .then(|| format!("day {number} {place}: {:?}", guidance.status()))
        })
        .collect();
    assert_eq!(wrong, Vec::<String>::new());
}
