mod leg_progress;
mod schedule;
mod snap;
mod tracking;
mod walk_search;

use crate::fixture::{DaySource, Fixture};
use guide::clock::CalendarDate;
use guide::clock::ClockTime;
use guide::clock::TripTime;
use guide::geo::Coordinate;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::PositionSource;
use guide::guidance::Sighting;

const FIX_STEP: Distance = Distance::new(10.0);
const ARCS: &str = "[[arcs]]\nnumber = 1\ntitle = [\"A → B\"]\nstart = \"A\"\ndays = [3]\n";
const DAY: &str = include_str!("../itinerary/fixture_day.toml");
const ROUTES: &str = r#"{"legs":{"1":{"segments":[{"no_mapped_path":{"via":[[140.0,38.0],[140.004,38.0]]}}]},"2":{"segments":[{"no_mapped_path":{"via":[[140.004,38.0],[140.012,38.0]]}}]}}}"#;
const DATE: &str = "2027-05-03";
const TIME: &str = "08:02";
const ON_LEG_TWO: Coordinate = Coordinate::new(Degrees::new(38.0002), Degrees::new(140.008));
const FAR_AWAY: Coordinate = Coordinate::new(Degrees::new(38.01), Degrees::new(140.001));
const HALFWAY_KM: f64 = 0.35;
const CLOSE_KM: f64 = 0.05;
const FAR_METERS: f64 = 1000.0;
const FIRST_LEG: usize = 0;
const SECOND_LEG: usize = 1;

#[test]
fn a_fix_that_barely_moved_keeps_the_last_one_and_a_real_move_passes() {
    let here = Coordinate::new(Degrees::new(38.37), Degrees::new(141.06));
    let moved = |east| Sighting::Fresh(here.offset_by(Distance::new(east), Distance::default()));
    let kept = Sighting::Fresh(here);
    assert_eq!(kept.kept_over(moved(3.0), FIX_STEP), kept);
    assert_eq!(kept.kept_over(moved(25.0), FIX_STEP), moved(25.0));
}

#[test]
fn a_gps_fix_on_a_later_leg_moves_you_onto_it() {
    let trip = Fixture::parse(ARCS, &[DaySource::new(DAY, ROUTES)]).unwrap();
    let now = TripTime::on_date_at(
        Fixture::zone(),
        CalendarDate::try_from(DATE.to_owned()).unwrap(),
        ClockTime::try_from(TIME.to_owned()).unwrap(),
    );
    let guidance = Guidance::new(
        trip.days().first().unwrap(),
        LegProgress::default(),
        now,
        Some(ON_LEG_TWO),
    );
    assert_eq!(usize::from(guidance.leg_index()), SECOND_LEG);
    assert!((guidance.along().kilometers().amount() - HALFWAY_KM).abs() < CLOSE_KM);
}

#[test]
fn a_fix_far_from_the_route_keeps_the_leg_and_reports_how_far_off() {
    let trip = Fixture::parse(ARCS, &[DaySource::new(DAY, ROUTES)]).unwrap();
    let now = TripTime::on_date_at(
        Fixture::zone(),
        CalendarDate::try_from(DATE.to_owned()).unwrap(),
        ClockTime::try_from(TIME.to_owned()).unwrap(),
    );
    let guidance = Guidance::new(
        trip.days().first().unwrap(),
        LegProgress::default(),
        now,
        Some(FAR_AWAY),
    );
    assert_eq!(usize::from(guidance.leg_index()), FIRST_LEG);
    assert!(
        guidance
            .off_route()
            .is_some_and(|distance| distance.meters() > FAR_METERS)
    );
}

#[test]
fn without_gps_the_estimate_stops_short_of_the_leg_end() {
    let trip = Fixture::parse(ARCS, &[DaySource::new(DAY, ROUTES)]).unwrap();
    let now = TripTime::on_date_at(
        Fixture::zone(),
        CalendarDate::try_from(DATE.to_owned()).unwrap(),
        ClockTime::try_from(TIME.to_owned()).unwrap(),
    );
    let guidance = Guidance::new(
        trip.days().first().unwrap(),
        LegProgress::default(),
        now,
        None,
    );
    assert!(
        guidance.source() == PositionSource::Estimate
            && Distance::from(guidance.left()).kilometers().amount() > CLOSE_KM
    );
}
