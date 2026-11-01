use crate::content::Content;
use crate::itinerary::day::goal::{Goal, Purpose};
use crate::leg::LegIndex;
use crate::place::RubyText;

#[derive(Clone, Debug)]
pub struct Ending {
    last: LegIndex,
    purpose: Purpose,
    name: RubyText,
    why: Content,
}

impl Ending {
    #[must_use]
    pub fn new(last: LegIndex, purpose: Purpose, name: RubyText, why: Content) -> Ending {
        Ending {
            last,
            purpose,
            name,
            why,
        }
    }

    #[must_use]
    pub fn last(&self) -> LegIndex {
        self.last
    }

    #[must_use]
    pub fn starting_at(self, first: LegIndex) -> Goal {
        Goal {
            first,
            last: self.last,
            purpose: self.purpose,
            name: self.name,
            why: self.why,
        }
    }
}
