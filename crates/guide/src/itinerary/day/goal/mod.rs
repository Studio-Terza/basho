mod aim;
mod ending;
mod goal_plan;
mod loop_check;
mod purpose;
mod reason;
mod sight_loop;

use crate::content::Content;
use crate::end::End;
use crate::itinerary::day::Caption;
use crate::leg::LegIndex;
use crate::place::RubyText;
use std::ops::Index;

pub use aim::Aim;
use ending::Ending;
pub use goal_plan::GoalPlan;
use loop_check::LoopCheck;
pub use purpose::Purpose;
pub use reason::Reason;
pub use sight_loop::{LoopNumber, LoopStop, SightLoop};

#[derive(derive_more::AsRef, Clone, Debug)]
pub struct Goal {
    first: LegIndex,
    last: LegIndex,
    purpose: Purpose,
    name: RubyText,
    #[as_ref]
    why: Content,
}

impl Goal {
    #[must_use]
    pub fn leg(&self, end: End) -> LegIndex {
        match end {
            End::Start => self.first,
            End::Finish => self.last,
        }
    }

    #[must_use]
    pub fn purpose(&self) -> &Purpose {
        &self.purpose
    }

    #[must_use]
    pub fn holds(&self, leg: LegIndex) -> bool {
        (self.first..=self.last).contains(&leg)
    }
}

impl Index<Caption> for Goal {
    type Output = RubyText;

    fn index(&self, caption: Caption) -> &RubyText {
        match caption {
            Caption::Name => &self.name,
        }
    }
}
