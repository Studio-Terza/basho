use crate::fixture::Fixture;
use guide::ArrivalMark;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::leg::LegIndex;

const DAY_THREE: usize = 3;
const LAST_SIGHT_WALK: LegIndex = LegIndex::new(14);
const LATE: Minutes = Minutes::new(60);
const CHECK_IN_CLOSES: &str = "15:00";

#[test]
fn an_hour_late_reaches_the_inn_after_check_in_closes() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let leg = day.leg(LAST_SIGHT_WALK).unwrap();
    let progress = LegProgress::confirmed(day.number(), LAST_SIGHT_WALK, LATE);
    let now = TripTime::on_date_at(day.zone(), day.date(), leg.time(End::Finish).later_by(LATE));
    let arrival = Guidance::new(day, progress, now, None).arrival().unwrap();
    assert_eq!(
        arrival.time(ArrivalMark::AfterCheckIn),
        ClockTime::try_from(CHECK_IN_CLOSES.to_owned()).ok()
    );
}
