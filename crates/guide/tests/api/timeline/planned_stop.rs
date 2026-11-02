use crate::fixture::Fixture;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::Sighting;
use guide::itinerary::Day;
use guide::leg::LegIndex;
use guide::readout::TimeCheck;
use guide::timeline::BufferState;
use guide::timeline::Halt;

const DAY_THREE: usize = 3;
const DAY_FOUR: usize = 4;
const HALL: LegIndex = LegIndex::new(1);
const SHRINE: LegIndex = LegIndex::new(3);
const HILL_SHRINE: LegIndex = LegIndex::new(2);
const BUS_GONE: &str = "09:31";
const HALL_BUS_DUE: &str = "08:31";
const DURING: &str = "09:50";
const STALE_SINCE: &str = "09:40";
const ASIDE: Distance = Distance::new(250.0);

struct Visit<'day> {
    day: &'day Day,
    leg: LegIndex,
}

impl<'day> Visit<'day> {
    pub fn at(&self, time: &str) -> TripTime {
        TripTime::on_date_at(
            self.day.zone(),
            self.day.date(),
            ClockTime::try_from(time.to_owned()).unwrap(),
        )
    }

    pub fn stale(&self, time: &str) -> Guidance<'day> {
        let progress = LegProgress::confirmed(self.day.number(), self.leg, Minutes::ZERO);
        let at = self.day.leg(self.leg).unwrap().point(End::Finish);
        let since = self.at(STALE_SINCE);
        Sighting::Stale { at, since }.guidance(self.day, progress, self.at(time))
    }

    pub fn aside(&self, time: &str) -> Guidance<'day> {
        let leg = self.day.leg(self.leg).unwrap();
        let progress = LegProgress::confirmed(self.day.number(), self.leg, Minutes::ZERO)
            .placed(leg.length(), self.at(STALE_SINCE));
        let at = leg.point(End::Finish);
        Guidance::new(
            self.day,
            progress,
            self.at(time),
            Some(at.offset_by(ASIDE, ASIDE)),
        )
    }

    pub fn halted(&self, time: &str) -> Guidance<'day> {
        let leg = self.day.leg(self.leg).unwrap();
        let progress = LegProgress::confirmed(self.day.number(), self.leg, Minutes::ZERO);
        let since = TripTime::on_date_at(self.day.zone(), self.day.date(), leg.time(End::Finish));
        Guidance::new(
            self.day,
            progress,
            self.at(time),
            Some(leg.point(End::Finish)),
        )
        .paused(Some(Halt::tracked(None, leg.point(End::Finish), since)))
    }
}

#[test]
fn a_walker_still_at_the_shrine_after_the_bus_left_is_short() {
    let trip = Fixture::bundled_days(&[DAY_FOUR]);
    let visit = Visit {
        day: trip.day(DAY_FOUR),
        leg: SHRINE,
    };
    let spare = visit.halted(BUS_GONE).buffer().map(|buffer| buffer.spare());
    assert!(
        spare.is_some_and(|spare| spare < Minutes::ZERO),
        "{spare:?}"
    );
}

#[test]
fn a_walker_ready_at_the_hall_after_the_target_is_waiting_not_short() {
    let trip = Fixture::bundled_days(&[DAY_FOUR]);
    let visit = Visit {
        day: trip.day(DAY_FOUR),
        leg: HALL,
    };
    let buffer = visit.halted(HALL_BUS_DUE).buffer();
    assert_eq!(
        buffer.map(|buffer| buffer.state()),
        Some(BufferState::Waiting)
    );
    assert_eq!(buffer.map(|buffer| buffer.spare()), Some(Minutes::new(4)));
}

#[test]
fn a_stale_or_wandering_walker_during_a_long_stop_keeps_a_buffer_and_is_not_late() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let visit = Visit {
        day: trip.day(DAY_THREE),
        leg: HILL_SHRINE,
    };
    assert!(visit.stale(DURING).buffer().is_some());
    let late = visit
        .aside(DURING)
        .time_check()
        .and_then(TimeCheck::late_by);
    assert!(late.is_none(), "{late:?}");
}
