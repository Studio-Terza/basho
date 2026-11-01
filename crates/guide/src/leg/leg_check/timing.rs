use crate::clock::Minutes;
use crate::clock::Pause;
use crate::end::End;
use crate::itinerary::Problem;
use crate::leg::Leg;
use crate::leg::leg_check::leg_rule::LegRule;

pub struct Timing;

impl Timing {
    const SLACK_MINUTES: Minutes = Minutes::new(3);
}

impl LegRule for Timing {
    fn problems(&self, leg: &Leg) -> Vec<Problem> {
        let stopped = leg.paused(Pause::Along);
        let start = leg.time(End::Start);
        let end = leg.time(End::Finish);
        let planned = start.minutes_until(end);
        let accounted = leg.minutes() + stopped;
        ((planned - accounted).abs() > Self::SLACK_MINUTES)
            .then(|| Problem::Timing {
                start: leg.listed(End::Start),
                end: leg.listed(End::Finish),
                planned,
                minutes: leg.minutes(),
                stopped,
                accounted,
            })
            .into_iter()
            .collect()
    }
}
