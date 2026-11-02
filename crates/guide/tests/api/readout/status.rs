use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::LegProgress;
use guide::guidance::Sighting;
use guide::leg::LegIndex;
use guide::leg::Mode;
use guide::readout::Status;

const SAMPLE_DAY: usize = 3;
const STATION_ROAD: LegIndex = LegIndex::new(6);
const LOCAL_TRAIN: LegIndex = LegIndex::new(7);
const FIRST_LANE: LegIndex = LegIndex::new(0);
const EARLY: &str = "11:07";
const ASTRAY: Distance = Distance::new(1500.0);
const SILENCE: Minutes = Minutes::new(6);
const DEPARTS: End = End::Start;
const ARRIVES: End = End::Finish;

#[test]
fn a_walker_at_the_station_before_the_train_is_waiting_until_it_leaves() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let arrival = ClockTime::try_from(EARLY.to_owned()).unwrap();
    let train = day.leg(LOCAL_TRAIN).unwrap();
    let guidance =
        Walker::walked(day, STATION_ROAD, arrival).fix(arrival, train.boarding().unwrap());
    assert_eq!(
        guidance.status(),
        Some(Status::Waiting {
            until: train.listed(DEPARTS)
        })
    );
}

#[test]
fn a_fix_on_the_train_line_is_riding_the_train() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let train = day.leg(LOCAL_TRAIN).unwrap();
    let walked = day.leg(STATION_ROAD).map(|leg| leg.time(ARRIVES)).unwrap();
    let on_board = train.line().point_along(train.length() * 0.5).unwrap();
    let guidance = Walker::walked(day, STATION_ROAD, walked).fix(train.time(DEPARTS), on_board);
    assert_eq!(guidance.status(), Some(Status::Riding(Mode::Train)));
}

#[test]
fn far_from_the_route_is_off_route_until_the_gps_goes_quiet() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let lane = day.leg(FIRST_LANE).unwrap();
    let at = lane.point(DEPARTS).offset_by(-ASTRAY, Distance::default());
    let since = TripTime::on_date_at(day.zone(), day.date(), lane.time(DEPARTS));
    let fresh = Sighting::Fresh(at).guidance(day, LegProgress::default(), since);
    let now = TripTime::on_date_at(day.zone(), day.date(), lane.time(DEPARTS).later_by(SILENCE));
    let quiet = Sighting::Stale { at, since }.guidance(day, LegProgress::default(), now);
    assert!(matches!(fresh.status(), Some(Status::OffRoute(distance)) if distance > ASTRAY * 0.5));
    assert_eq!(quiet.status(), Some(Status::NoGps(Some(SILENCE))));
}
