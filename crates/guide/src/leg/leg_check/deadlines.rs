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
            leg.tour()
                .filter(|_| leg.mode() != Mode::Walk)
                .map(|_| Problem::TourOnRide),
            leg.pay()
                .map(|_| Problem::RideTermsOnWalk)
                .or_else(|| leg.margin_rule().map(|_| Problem::RideTermsOnWalk))
                .or_else(|| leg.transfer().map(|_| Problem::RideTermsOnWalk))
                .filter(|_| leg.mode() == Mode::Walk),
            (leg.mode() != Mode::Walk)
                .then_some(Problem::PayMissing)
                .filter(|_| leg.pay().is_none()),
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
            .filter_map(|ride| ride.runs_out_of_order(leg.time(End::Start)))
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
