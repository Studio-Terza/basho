use crate::fixture::Fixture;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::LegProgress;
use guide::guidance::PositionSource;
use guide::guidance::Sighting;
use guide::guidance::marker::Marker;
use guide::guidance::time_check::TimeCheck;
use guide::leg::LegIndex;
use guide::readout::Status;

const DAY_THREE: usize = 3;
const FIRST_WALK: LegIndex = LegIndex::new(0);
const ASTRAY: Distance = Distance::new(1500.0);
const SILENCE: Minutes = Minutes::new(6);
const CACHED: Minutes = Minutes::new(40);

#[test]
fn losing_gps_off_the_route_keeps_the_warning_at_the_last_fix() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let leg = day.leg(FIRST_WALK).unwrap();
    let at = leg.point(End::Start).offset_by(Distance::default(), ASTRAY);
    let since = TripTime::on_date_at(day.zone(), day.date(), leg.time(End::Start));
    let now = TripTime::on_date_at(
        day.zone(),
        day.date(),
        leg.time(End::Start).later_by(SILENCE),
    );
    let guidance = Sighting::Stale { at, since }.guidance(day, LegProgress::default(), now);
    assert!(guidance.off_route().is_some());
    assert_eq!(guidance.marker(), Some(at));
    assert_eq!(guidance.source().lost_for(now), Some(SILENCE));
    assert_eq!(guidance.progress(), None);
}

#[test]
fn losing_gps_on_the_route_estimates_but_still_shows_the_last_fix() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let leg = day.leg(FIRST_WALK).unwrap();
    let at = leg.point(End::Start);
    let since = TripTime::on_date_at(day.zone(), day.date(), leg.time(End::Start));
    let now = TripTime::on_date_at(
        day.zone(),
        day.date(),
        leg.time(End::Start).later_by(SILENCE),
    );
    let guidance = Sighting::Stale { at, since }.guidance(day, LegProgress::default(), now);
    assert!(guidance.off_route().is_none());
    assert_eq!(guidance.marking(), Some(Marker::LastFix(at)));
    assert_eq!(guidance.source(), PositionSource::GpsLost { since });
}

#[test]
fn a_cached_fix_at_reopening_is_read_at_its_own_time_and_shows_its_age() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let leg = day.leg(FIRST_WALK).unwrap();
    let since = TripTime::on_date_at(day.zone(), day.date(), leg.time(End::Start));
    let now = TripTime::on_date_at(
        day.zone(),
        day.date(),
        leg.time(End::Start).later_by(CACHED),
    );
    let reopened = Sighting::Stale {
        at: leg.point(End::Start),
        since,
    }
    .guidance(day, LegProgress::default(), now);
    assert_eq!(reopened.status(), Some(Status::NoGps(Some(CACHED))));
    assert!(reopened.arrow().is_none());
    assert!(reopened.progress().is_none());
    assert!(reopened.time_check().and_then(TimeCheck::late_by).is_none());
}
