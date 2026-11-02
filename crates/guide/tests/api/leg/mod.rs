mod cutoff;

use crate::fixture::Fixture;
use guide::clock::ClockTime;
use guide::end::End;
use guide::leg::LegIndex;

const RIDES_DAY: usize = 4;
const RAPID_AFTER_THE_SHUTTLE: LegIndex = LegIndex::new(5);
const BUY_AT_THE_JUNCTION: &str = "09:45";

#[test]
fn a_ticket_bought_after_a_ride_keeps_the_stricter_buy_margin() {
    let trip = Fixture::bundled_days(&[RIDES_DAY]);
    let train = trip.day(RIDES_DAY).leg(RAPID_AFTER_THE_SHUTTLE).unwrap();
    let target = train
        .boarding_margin()
        .map(|margin| margin.target(train.time(End::Start)));
    assert_eq!(
        target,
        ClockTime::try_from(BUY_AT_THE_JUNCTION.to_owned()).ok()
    );
}

#[test]
fn every_walk_or_transfer_reaches_the_next_ride_by_its_margin() {
    let trip = Fixture::bundled().unwrap();
    let short: Vec<ClockTime> = trip
        .days()
        .iter()
        .flat_map(|day| day.legs().iter().zip(day.legs().iter().skip(1)))
        .filter(|(before, ride)| {
            ride.boarding_margin().is_some_and(|margin| {
                before.time(End::Finish) > margin.target(ride.time(End::Start))
            })
        })
        .map(|(_, ride)| ride.time(End::Start))
        .collect();
    assert_eq!(short, Vec::<ClockTime>::new());
}
