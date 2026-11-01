use crate::fixture::Fixture;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::itinerary::Day;
use guide::leg::Leg;
use guide::leg::LegIndex;
use guide::leg::Mode;

const LATE: Minutes = Minutes::new(60);
const SAME_SPOT: Distance = Distance::new(30.0);

struct LateWalker<'trip> {
    day: &'trip Day,
}

impl LateWalker<'_> {
    fn misplaced(&self) -> Vec<String> {
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
            .filter(|(_, leg)| leg.mode() == Mode::Walk && !leg.line().is_sketch())
            .filter_map(|(position, leg)| {
                let here = leg.line().point_along(leg.length() * 0.5)?;
                let guidance =
                    Guidance::new(self.day, LegProgress::default(), evening?, Some(here));
                let found = guidance.leg().map(Leg::line)?.fix(here)?;
                (guidance.missed_ride().is_none()
                    && (found.off_route() > SAME_SPOT || guidance.off_route().is_some()))
                .then(|| {
                    format!(
                        "day {} leg {}: placed on leg {}",
                        self.day.number(),
                        LegIndex::new(position),
                        guidance.leg_index()
                    )
                })
            })
            .collect()
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
