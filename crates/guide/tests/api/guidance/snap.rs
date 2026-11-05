use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::time_check::TimeCheck;
use guide::itinerary::Day;
use guide::leg::Leg;
use guide::leg::LegIndex;
use guide::leg::Mode;

const LATE: Minutes = Minutes::new(60);
const VISITS: [[usize; 2]; 4] = [[3, 2], [3, 12], [3, 13], [4, 1]];
const STATION_DAY: usize = 3;
const FERN_TRAIN: usize = 7;
const AFTER_DEPARTURE: &str = "11:25";
const SAME_SPOT: Distance = Distance::new(30.0);

struct LateWalker<'trip> {
    day: &'trip Day,
}

struct Placed<'trip> {
    day: &'trip Day,
    position: usize,
    drift: Distance,
    guidance: Guidance<'trip>,
}

impl Placed<'_> {
    fn report(&self) -> String {
        format!(
            "day {} leg {}: placed on leg {}",
            self.day.number(),
            LegIndex::new(self.position),
            self.guidance.leg_index()
        )
    }
}

impl<'trip> LateWalker<'trip> {
    fn placed(&self) -> Vec<Placed<'trip>> {
        let evening = self.day.legs().last().map(|leg| {
            TripTime::on_date_at(
                self.day.zone(),
                self.day.date(),
                leg.time(End::Finish).later_by(LATE),
            )
        });
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| (leg.mode(), leg.line().is_sketch()) == (Mode::Walk, false))
            .filter_map(|(position, leg)| {
                let here = leg.line().point_along(leg.length() * 0.5)?;
                let guidance =
                    Guidance::new(self.day, LegProgress::default(), evening?, Some(here));
                let found = guidance.leg().map(Leg::line)?.fix(here)?;
                Some(Placed {
                    day: self.day,
                    position,
                    drift: Distance::from(found.off_route()),
                    guidance,
                })
            })
            .filter(|placed| placed.guidance.missed_ride().is_none())
            .collect()
    }

    fn misplaced(&self) -> Vec<String> {
        let placed = self.placed();
        let drifted = placed.iter().filter(|placed| placed.drift > SAME_SPOT);
        let lost = placed
            .iter()
            .filter(|placed| placed.guidance.off_route().is_some());
        drifted.chain(lost).map(Placed::report).collect()
    }
}

#[test]
fn a_walker_far_behind_the_timetable_is_found_on_the_leg_they_are_walking() {
    let trip = Fixture::bundled().unwrap();
    let misplaced = trip
        .days()
        .iter()
        .flat_map(|day| LateWalker { day }.misplaced())
        .collect::<Vec<String>>();
    assert_eq!(misplaced, Vec::<String>::new());
}

#[test]
fn standing_at_the_station_after_the_train_left_means_it_was_missed() {
    let trip = Fixture::bundled_days(&[STATION_DAY]);
    let day = trip.day(STATION_DAY);
    let train = day.legs().get(FERN_TRAIN).unwrap();
    let time = ClockTime::try_from(AFTER_DEPARTURE.to_owned()).unwrap();
    let now = TripTime::on_date_at(day.zone(), day.date(), time);
    let guidance = Guidance::new(day, LegProgress::default(), now, train.boarding());
    assert_eq!(guidance.missed_ride().map(usize::from), Some(FERN_TRAIN));
}

#[test]
fn a_planned_visit_is_planned_time_and_never_shows_late() {
    let trip = Fixture::bundled_days(&VISITS.map(|[number, _]| number));
    let checks: Vec<(usize, usize, Option<TimeCheck>)> = VISITS
        .iter()
        .flat_map(|[number, arrival]| {
            let day = trip.day(*number);
            Walker::fresh(day)
                .visited(LegIndex::new(*arrival))
                .into_iter()
                .map(move |check| (*number, *arrival, check))
        })
        .collect();
    let late: Vec<String> = checks
        .iter()
        .filter_map(|(number, arrival, check)| {
            check
                .and_then(TimeCheck::late_by)
                .map(|over| format!("day {number} after leg {arrival}: LATE {over}"))
        })
        .collect();
    assert_eq!(checks.len(), VISITS.len() * 2);
    assert_eq!(late, Vec::<String>::new());
}
