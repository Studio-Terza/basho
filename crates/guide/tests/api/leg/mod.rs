mod cutoff;

use crate::fixture::Fixture;
use guide::clock::ClockTime;
use guide::end::End;

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
