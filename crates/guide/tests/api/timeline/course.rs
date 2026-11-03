use crate::fixture::Fixture;
use guide::DeadlineKind;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::leg::Leg;
use guide::leg::LegIndex;
use guide::leg::margin::Margin;

const DAY_THREE: usize = 3;
const DAY_FOUR: usize = 4;
const PAST_THE_MUSEUM: LegIndex = LegIndex::new(10);
const TO_THE_CAFE: LegIndex = LegIndex::new(11);
const LOCAL_TRAIN_WALK: LegIndex = LegIndex::new(6);
const SHINKANSEN: LegIndex = LegIndex::new(8);

#[test]
fn once_the_walker_passes_a_closing_time_the_next_deadline_binds() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let binding = |index: LegIndex| {
        let leg = day.leg(index).unwrap();
        let now = TripTime::on_date_at(day.zone(), day.date(), leg.time(End::Start));
        let progress = LegProgress::confirmed(day.number(), index, Minutes::ZERO);
        Guidance::new(day, progress, now, leg.line().points().first().copied())
            .buffer()
            .map(|buffer| buffer.deadline().kind())
    };
    assert_eq!(binding(PAST_THE_MUSEUM), Some(DeadlineKind::Closes));
    assert_eq!(binding(TO_THE_CAFE), Some(DeadlineKind::CheckIn));
}

#[test]
fn a_shinkansen_after_a_stopover_keeps_its_eticket_margin() {
    let trip = Fixture::bundled_days(&[DAY_FOUR]);
    let day = trip.day(DAY_FOUR);
    let shinkansen = day.leg(SHINKANSEN).unwrap();
    let ride = guide::guidance::timeline::course::Course::new(day)
        .closing(SHINKANSEN)
        .unwrap();
    assert_eq!(
        ride.window().start,
        Margin::TICKET_READY.target(shinkansen.time(End::Start))
    );
    assert!(
        day.leg(LOCAL_TRAIN_WALK)
            .map(Leg::boarding_margin)
            .is_some_and(|margin| margin.is_none())
    );
}
