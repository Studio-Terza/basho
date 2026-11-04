mod day_catch;

use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::Trip;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::Sighting;
use guide::leg::LegIndex;
use guide::readout::pointer::Pointer;
use rayon::prelude::*;

const SAMPLE_DAY: usize = 3;
const FIRST_LANE: LegIndex = LegIndex::new(0);
const ASTRAY: Distance = Distance::new(600.0);
const SILENCE: Minutes = Minutes::new(6);
const SIDES: [Distance; 2] = [Distance::new(-300.0), Distance::new(300.0)];
const SET_OFF: End = End::Start;

#[test]
fn off_the_route_the_panel_points_to_the_route_and_two_features_nearest_first() {
    let trip = Fixture::BUNDLED.parse::<Trip>().unwrap();
    let day = trip
        .itinerary()
        .days()
        .iter()
        .find(|day| usize::from(day.number().number()) == SAMPLE_DAY)
        .unwrap();
    let lane = day.leg(FIRST_LANE).unwrap();
    let here = lane.point(SET_OFF).offset_by(Distance::default(), ASTRAY);
    let now = TripTime::on_date_at(day.zone(), day.date(), lane.time(SET_OFF));
    let guidance = Guidance::new(day, LegProgress::default(), now, Some(here));
    let recovery = guidance.recovery().unwrap();
    let distances: Vec<Distance> = recovery
        .toward()
        .iter()
        .map(|sight| sight.pointer().distance())
        .collect();
    assert_eq!(
        recovery.route().map(Pointer::distance),
        guidance.off_route()
    );
    assert_eq!(distances.len(), 2, "{distances:?}");
    assert!(distances[0] <= distances[1], "{distances:?}");
}

#[test]
fn on_the_route_and_on_time_there_is_no_panel_but_a_quiet_gps_dates_it() {
    let trip = Fixture::BUNDLED.parse::<Trip>().unwrap();
    let day = trip
        .itinerary()
        .days()
        .iter()
        .find(|day| usize::from(day.number().number()) == SAMPLE_DAY)
        .unwrap();
    let lane = day.leg(FIRST_LANE).unwrap();
    let since = TripTime::on_date_at(day.zone(), day.date(), lane.time(SET_OFF));
    let on_route =
        Sighting::Fresh(lane.point(SET_OFF)).guidance(day, LegProgress::default(), since);
    let at = lane.point(SET_OFF).offset_by(Distance::default(), ASTRAY);
    let now = TripTime::on_date_at(day.zone(), day.date(), lane.time(SET_OFF).later_by(SILENCE));
    let quiet = Sighting::Stale { at, since }.guidance(day, LegProgress::default(), now);
    assert!(on_route.recovery().is_none());
    assert_eq!(
        quiet
            .recovery()
            .and_then(|recovery| recovery.fix_time())
            .map(ClockTime::from),
        Some(lane.time(SET_OFF))
    );
}

#[test]
fn off_the_route_every_catching_feature_shown_lies_toward_the_route() {
    let trip = Fixture::bundled().unwrap();
    let misleading: Vec<String> = trip
        .days()
        .par_iter()
        .flat_map_iter(|day| SIDES.map(|aside| Walker::misleading_sights(day, aside)))
        .flatten()
        .collect();
    assert_eq!(misleading, Vec::<String>::new());
}
