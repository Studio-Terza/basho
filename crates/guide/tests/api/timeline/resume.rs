use crate::fixture::Fixture;
use guide::BufferState;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::leg::LegIndex;

const DAY_THREE: usize = 3;
const DAY_FOUR: usize = 4;
const SHINKANSEN: LegIndex = LegIndex::new(8);
const ON_BOARD: &str = "17:42";
const BEFORE_LEAVING: &str = "05:00";

#[test]
fn a_tight_spare_on_a_train_is_plain_not_do_not_stop() {
    let trip = Fixture::bundled_days(&[DAY_FOUR]);
    let day = trip.day(DAY_FOUR);
    let shinkansen = day.leg(SHINKANSEN).unwrap();
    let here = shinkansen
        .line()
        .point_along(shinkansen.line().length() * 0.5);
    let now = ClockTime::try_from(ON_BOARD.to_owned()).unwrap();
    let progress = LegProgress::confirmed(day.number(), SHINKANSEN, Minutes::ZERO);
    let buffer = Guidance::new(
        day,
        progress,
        TripTime::on_date_at(day.zone(), day.date(), now),
        here,
    )
    .buffer();
    assert!(buffer.is_some_and(|buffer| buffer.spare() < BufferState::TIGHT_BELOW));
    assert_eq!(
        buffer.map(|buffer| buffer.state()),
        Some(BufferState::Comfortable)
    );
}

#[test]
fn a_tight_spare_before_the_day_starts_is_plain_not_do_not_stop() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let now = ClockTime::try_from(BEFORE_LEAVING.to_owned()).unwrap();
    let guidance = Guidance::new(
        day,
        LegProgress::default(),
        TripTime::on_date_at(day.zone(), day.date(), now),
        None,
    );
    let buffer = guidance.buffer();
    assert!(buffer.is_some_and(|buffer| buffer.spare() < BufferState::TIGHT_BELOW));
    assert_eq!(
        buffer.map(|buffer| buffer.state()),
        Some(BufferState::Comfortable)
    );
}
