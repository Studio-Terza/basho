use crate::guidance::Guidance;
use crate::guidance::pressing::Pressing;
use crate::leg::{Branch, LegIndex};

#[derive(Clone, Copy, Debug)]
pub struct Plan<'day> {
    leg: LegIndex,
    branch: &'day Branch,
    pressing: Pressing,
}

impl<'day> Plan<'day> {
    #[must_use]
    pub fn leg(self) -> LegIndex {
        self.leg
    }

    #[must_use]
    pub fn new(guidance: &Guidance<'day>, leg: LegIndex, branch: &'day Branch) -> Plan<'day> {
        let now = guidance.now();
        let today = (now.trip_date() == guidance.day().date()).then(|| now.time());
        let departed = guidance
            .missed_ride()
            .filter(|ride| *ride == leg)
            .map_or(Pressing::Later, |_| Pressing::Now);
        let pressing = branch
            .cue()
            .map_or(Pressing::Later, |cue| cue.pressing(today, departed));
        Plan {
            leg,
            branch,
            pressing,
        }
    }

    #[must_use]
    pub fn branch(self) -> &'day Branch {
        self.branch
    }

    #[must_use]
    pub fn pressing(self) -> Pressing {
        self.pressing
    }
}
