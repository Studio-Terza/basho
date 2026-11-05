mod aim;
mod goal_plan;
mod purpose;
mod reason;

use crate::content::Content;
use crate::end::End;
use crate::itinerary::day::Caption;
use crate::leg::LegIndex;
use crate::ruby_text::RubyText;
use std::ops::Index;

pub use aim::Aim;
pub use goal_plan::GoalPlan;
pub use purpose::Purpose;
pub use reason::Reason;

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
