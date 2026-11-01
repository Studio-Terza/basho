use crate::clock::ClockTime;
use crate::end::End;
use crate::itinerary::Problem;
use crate::leg::leg_check::leg_rule::LegRule;
use crate::leg::{Branch, Leg, Mode};
use crate::leg_line::LegLine;
use crate::reading::Reading;
use crate::times::{ListedTime, PlanTime};

pub struct Deadlines;

impl Deadlines {
    fn kinds(leg: &Leg) -> impl Iterator<Item = Problem> {
        [
            (leg.mode() != Mode::Walk && leg.tour().is_some()).then_some(Problem::TourOnRide),
            (leg.mode() == Mode::Walk
                && (leg.pay().is_some()
                    || leg.margin_rule().is_some()
                    || leg.transfer().is_some()))
            .then_some(Problem::RideTermsOnWalk),
            (leg.mode() != Mode::Walk && leg.pay().is_none()).then_some(Problem::PayMissing),
            leg.tour()
                .filter(|tour| ClockTime::from(tour.check_in()) > leg.time(End::Start))
                .map(|tour| Problem::TourCheckInAfterStart {
                    check_in: tour.check_in(),
                }),
        ]
        .into_iter()
        .flatten()
    }
}

impl LegRule for Deadlines {
    fn problems(&self, leg: &Leg) -> Vec<Problem> {
        let own = leg
            .cutoff()
            .filter(|cutoff| leg.time(End::Finish) > cutoff.deadline())
            .map(|cutoff| Problem::PastDeadline {
                place: cutoff
                    .name()
                    .unwrap_or(leg.line_text(LegLine::Reach))
                    .text(Reading::Written),
                arrival: leg.listed(End::Finish),
                deadline: cutoff.listed_at(),
            });
        let stops = leg.along().iter().filter_map(|stop| {
            let cutoff = stop.cutoff()?;
            let arrival = leg
                .time(End::Start)
                .later_by(leg.planned_after(stop.distance_along()));
            (arrival > cutoff.deadline()).then(|| Problem::PastDeadline {
                place: cutoff
                    .name()
                    .unwrap_or(stop.as_ref())
                    .text(Reading::Written),
                arrival: ListedTime::Planned(PlanTime::new(arrival)),
                deadline: cutoff.listed_at(),
            })
        });
        let backups = leg
            .branches()
            .iter()
            .filter_map(Branch::backup_ride)
            .filter(|ride| ride.runs_out_of_order(leg.time(End::Start)))
            .map(|ride| Problem::BackupRideOutOfOrder {
                leaves: ride.listed(End::Start),
                arrives: ride.listed(End::Finish),
            });
        own.into_iter()
            .chain(stops)
            .chain(Self::kinds(leg))
            .chain(backups)
            .collect()
    }
}
