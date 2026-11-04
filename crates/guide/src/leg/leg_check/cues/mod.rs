mod moments;

use crate::itinerary::Problem;
use crate::leg::cue::Cue;
use crate::leg::leg_check::leg_rule::LegRule;
use crate::leg::{BranchLine, Leg, Mode};
use crate::reading::Reading;
use moments::Moments;

pub struct Cues;

impl LegRule for Cues {
    fn problems(&self, leg: &Leg) -> Vec<Problem> {
        let uncued = leg
            .branches()
            .iter()
            .filter(|branch| branch.cue().is_none())
            .map(|branch| branch.line_text(BranchLine::Trigger).text(Reading::Written))
            .filter_map(|trigger| {
                trigger
                    .parse::<Moments>()
                    .ok()
                    .and_then(|moments| moments.first())
                    .map(|_| Problem::UncuedTime { trigger })
            });
        let departed = leg
            .branches()
            .iter()
            .filter(|_| leg.mode() == Mode::Walk)
            .filter(|branch| branch.cue() == Some(Cue::Departed))
            .map(|_| Problem::DepartedOnWalk);
        uncued.chain(departed).collect()
    }
}
