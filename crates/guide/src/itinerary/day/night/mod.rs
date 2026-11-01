mod short_name;

use crate::english_name::EnglishName;
use serde::Deserialize;
use std::ops::Index;

use crate::clock::ClockTime;
use crate::content::Content;
use crate::evidence::Evidence;
use crate::geo::Coordinate;
use crate::itinerary::day::Caption;
use crate::place::{Fact, RubyText};
use crate::times::{ListedTime, PlanTime};
use short_name::ShortName;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Night {
    name: RubyText,
    #[as_ref]
    english: EnglishName,
    short: ShortName,
    #[as_ref]
    at: Coordinate,
    #[as_ref]
    evidence: Evidence,
    #[as_ref]
    why: Content,
    check_in_by: Option<ListedTime>,
    #[serde(default)]
    notes: Vec<Fact>,
}

impl Night {
    pub const USUAL_CHECK_IN: ClockTime = ClockTime::at_hour(18);

    #[must_use]
    pub fn short(&self) -> &ShortName {
        &self.short
    }

    #[must_use]
    pub fn check_in_by(&self) -> ClockTime {
        ClockTime::from(self.listed_check_in())
    }

    #[must_use]
    pub fn listed_check_in(&self) -> ListedTime {
        self.check_in_by
            .unwrap_or(ListedTime::Planned(PlanTime::new(Self::USUAL_CHECK_IN)))
    }

    #[must_use]
    pub fn notes(&self) -> &[Fact] {
        &self.notes
    }
}

impl Index<Caption> for Night {
    type Output = RubyText;

    fn index(&self, caption: Caption) -> &RubyText {
        match caption {
            Caption::Name => &self.name,
        }
    }
}
