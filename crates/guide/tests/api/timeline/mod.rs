mod backup;
mod course;
mod halt;
mod here;
mod planned_stop;
mod resume;
mod stretch;

use crate::fixture::Fixture;
use guide::clock::Minutes;
use guide::clock::Pause;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Coordinate;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::itinerary::Day;
use guide::leg::LegIndex;
use guide::timeline::BufferState;
use guide::timeline::DeadlineKind;
use guide::timeline::Halt;
use guide::timeline::Margin;

const DAY_THREE: usize = 3;
const MILL_RACE_PATH: LegIndex = LegIndex::new(5);
const TRAIN: LegIndex = LegIndex::new(7);
const LUNCH: LegIndex = LegIndex::new(11);
const LATER: Minutes = Minutes::new(10);
const WITHIN: Minutes = Minutes::new(25);
const OVER: Minutes = Minutes::new(50);

struct Scene<'day> {
    day: &'day Day,
}

impl<'day> Scene<'day> {
    fn walking(&self, index: LegIndex, delay: Minutes) -> Guidance<'day> {
        let leg = self.day.leg(index).unwrap();
        let middle = leg.line().length() * 0.5;
        let now = leg
            .time(End::Start)
            .later_by(leg.planned_after(middle) + delay);
        let progress = LegProgress::confirmed(self.day.number(), index, Minutes::ZERO);
        let here = leg.line().point_along(middle);
        Guidance::new(self.day, progress, self.moment(now), here)
    }

    fn at_lunch(&self, spent: Minutes) -> Guidance<'day> {
        let leg = self.day.leg(LUNCH).unwrap();
        let spot = leg
            .along()
            .last()
            .map_or(leg.point(End::Finish), Coordinate::from);
        let progress = LegProgress::confirmed(self.day.number(), LUNCH, Minutes::ZERO);
        let reached = leg.time(End::Finish).later_by(-leg.paused(Pause::Along));
        let halt = Halt::tracked(None, spot, self.moment(reached));
        let now = self.moment(reached.later_by(spent));
        Guidance::new(self.day, progress, now, Some(spot)).paused(Some(halt))
    }

    fn moment(&self, time: guide::clock::ClockTime) -> TripTime {
        TripTime::on_date_at(self.day.zone(), self.day.date(), time)
    }
}

#[test]
fn the_next_ride_binds_with_its_margin_and_lost_minutes_come_off_one_for_one() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let scene = Scene {
        day: trip.day(DAY_THREE),
    };
    let on_time = scene
        .walking(MILL_RACE_PATH, Minutes::ZERO)
        .buffer()
        .unwrap();
    let late = scene.walking(MILL_RACE_PATH, LATER).buffer().unwrap();
    let departure = scene
        .day
        .leg(TRAIN)
        .map(|leg| leg.time(End::Start))
        .unwrap();
    assert_eq!(on_time.deadline().kind(), DeadlineKind::Ride);
    assert_eq!(
        on_time.deadline().window().start,
        Margin::TICKET_TO_BUY.target(departure)
    );
    assert_eq!(late.spare(), on_time.spare() - LATER);
}

#[test]
fn a_ride_that_will_be_missed_shows_the_backup_and_how_late_it_reaches_the_inn() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let scene = Scene {
        day: trip.day(DAY_THREE),
    };
    let planned = scene
        .walking(LegIndex::default(), Minutes::ZERO)
        .buffer()
        .unwrap();
    let short = scene.walking(LegIndex::default(), planned.spare() + LATER);
    let buffer = short.buffer().unwrap();
    assert_eq!(buffer.state(), BufferState::Short);
    assert!(
        buffer
            .backup()
            .is_some_and(|backup| backup.after_check_in().is_some())
    );
}

#[test]
fn a_planned_stop_uses_its_booked_minutes_before_the_buffer_shrinks() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let scene = Scene {
        day: trip.day(DAY_THREE),
    };
    let booked = scene
        .day
        .leg(LUNCH)
        .map(|leg| leg.paused(Pause::Along))
        .unwrap();
    let within = scene.at_lunch(WITHIN);
    let over = scene.at_lunch(OVER);
    let spare = |guidance: &Guidance<'_>| guidance.buffer().map(|buffer| buffer.spare());
    assert_eq!(within.stop().map(|stop| stop.spent()), Some(WITHIN));
    assert_eq!(
        spare(&over),
        spare(&within).map(|spare| spare - (OVER - booked))
    );
}
