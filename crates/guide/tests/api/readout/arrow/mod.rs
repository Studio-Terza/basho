use crate::fixture::Fixture;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Bearing;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::arrow::Aim;
use guide::guidance::arrow::Arrow;
use guide::leg::LegIndex;
use guide::readout::pointer::Pointer;

const SAMPLE_DAY: usize = 3;
const MILL_RACE: LegIndex = LegIndex::new(5);
const STATION_ROAD: LegIndex = LegIndex::new(6);
const FIRST_LANE: LegIndex = LegIndex::new(0);
const NEAR_END: Distance = Distance::new(10.0);
const ASTRAY: Distance = Distance::new(-1500.0);
const CLOSE: Distance = Distance::new(1.0);

#[test]
fn on_the_route_the_arrow_points_fifty_meters_ahead_on_the_line() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let road = day.leg(STATION_ROAD).unwrap();
    let kept = LegProgress::confirmed(day.number(), STATION_ROAD, Minutes::ZERO);
    let now = TripTime::on_date_at(day.zone(), day.date(), road.time(End::Start));
    let here = road.line().point_along(road.length() * 0.5).unwrap();
    let arrow = Guidance::new(day, kept, now, Some(here)).arrow().unwrap();
    let ahead = road.line().fix(here).unwrap().along() + Arrow::LOOK_AHEAD;
    let target = road.line().point_along(ahead).unwrap();
    assert_eq!(arrow.aim(), Aim::Ahead);
    assert!((arrow.pointer().distance() - here.distance_to(target)).abs() < CLOSE);
}

#[test]
fn near_the_end_of_a_leg_the_arrow_points_onto_the_next_walk() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let path = day.leg(MILL_RACE).unwrap();
    let road = day.leg(STATION_ROAD).unwrap();
    let here = path.line().point_along(path.length() - NEAR_END).unwrap();
    let along = path.line().fix(here).unwrap().along();
    let reach = along + Arrow::LOOK_AHEAD - path.length();
    let target = road.line().point_along(reach).unwrap();
    assert_eq!(
        Arrow::ahead(here, path, along, Some(road)).pointer(),
        Pointer::between(here, target)
    );
}

#[test]
fn off_the_route_the_arrow_points_back_to_it_from_the_phone_heading() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let lane = day.leg(FIRST_LANE).unwrap();
    let here = lane
        .point(End::Start)
        .offset_by(ASTRAY, Distance::default());
    let now = TripTime::on_date_at(day.zone(), day.date(), lane.time(End::Start));
    let guidance = Guidance::new(day, LegProgress::default(), now, Some(here));
    let arrow = guidance.arrow().unwrap();
    let heading = arrow.pointer().bearing();
    assert_eq!(arrow.aim(), Aim::BackToRoute);
    assert_eq!(arrow.pointer().distance(), guidance.off_route().unwrap());
    assert_eq!(arrow.angle(Some(heading)).turn(), Bearing::new(0.0));
}
