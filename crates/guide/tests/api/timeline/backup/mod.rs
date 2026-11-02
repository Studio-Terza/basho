use crate::fixture::Fixture;
use guide::BackupMark;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::leg::Leg;
use guide::leg::LegIndex;
use guide::leg::Mode;

const DAY_THREE: usize = 3;
const DEPARTS: &str = "11:17";
const BEFORE_IT_LEAVES: &str = "11:15";
const INN: &str = "17:32";

struct Departure;

impl Departure {
    pub fn of(day: &guide::itinerary::Day, time: &str) -> LegIndex {
        let departs = ClockTime::try_from(time.to_owned()).unwrap();
        let found = day
            .legs()
            .iter()
            .position(|leg| leg.mode() != Mode::Walk && leg.time(End::Start) == departs);
        LegIndex::new(found.unwrap())
    }
}

#[test]
fn the_backup_line_at_the_station_gives_the_real_inn_time_after_the_next_boat() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let train = Departure::of(day, DEPARTS);
    let station = day.leg(train).and_then(Leg::boarding);
    let now = ClockTime::try_from(BEFORE_IT_LEAVES.to_owned()).unwrap();
    let progress = LegProgress::confirmed(day.number(), train.previous().unwrap(), Minutes::ZERO);
    let guidance = Guidance::new(
        day,
        progress,
        TripTime::on_date_at(day.zone(), day.date(), now),
        station,
    );
    let arrives = guidance
        .buffer()
        .and_then(|buffer| buffer.backup())
        .and_then(|backup| backup.time(BackupMark::Arrives));
    assert_eq!(arrives, ClockTime::try_from(INN.to_owned()).ok());
}
