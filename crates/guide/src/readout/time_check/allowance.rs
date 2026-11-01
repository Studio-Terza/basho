use crate::clock::Minutes;
use crate::end::End;
use crate::geo::Share;
use crate::leg::{Leg, Mode};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Allowance {
    share: Share,
    floor: Minutes,
}

impl Allowance {
    pub const WALK: Allowance = Allowance {
        share: Share::new(0.15),
        floor: Minutes::new(3),
    };
    pub const RIDE: Allowance = Allowance {
        share: Share::new(0.0),
        floor: Minutes::new(3),
    };

    #[must_use]
    pub fn for_leg(leg: &Leg) -> Minutes {
        let rule = match leg.mode() {
            Mode::Walk => Allowance::WALK,
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Allowance::RIDE,
        };
        let planned = leg
            .time(End::Start)
            .minutes_until(leg.time(End::Finish))
            .max(Minutes::ZERO);
        Minutes::rounded(planned.amount() * rule.share.fraction())
            .unwrap_or_default()
            .max(rule.floor)
    }
}
