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
            .filter(|trigger| {
                trigger
                    .parse::<Moments>()
                    .is_ok_and(|moments| !moments.is_empty())
            })
            .map(|trigger| Problem::UncuedTime { trigger });
        let departed = leg
            .branches()
            .iter()
            .filter(|branch| leg.mode() == Mode::Walk && branch.cue() == Some(Cue::Departed))
            .map(|_| Problem::DepartedOnWalk);
        uncued.chain(departed).collect()
    }
}
