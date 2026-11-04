use guide::clock::ClockTime;
use guide::end::End;
use guide::itinerary::Day;
use guide::leg::{LegIndex, Mode};

pub struct Departure;

impl Departure {
    pub fn of(day: &Day, time: &str) -> LegIndex {
        let departs = ClockTime::try_from(time.to_owned()).unwrap();
        let found = day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() != Mode::Walk)
            .find(|(_, leg)| leg.time(End::Start) == departs)
            .map(|(index, _)| index);
        LegIndex::new(found.unwrap())
    }
}
