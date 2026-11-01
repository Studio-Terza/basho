use std::fmt;

use serde::Deserialize;

use crate::clock::ClockTime;
use crate::times::fact_time::FactTime;
use crate::times::plan_time::PlanTime;
use crate::times::shown_time::ShownTime;
use crate::words::Spelling;

#[derive(derive_more::From, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(untagged)]
pub enum ListedTime {
    Stated(FactTime),
    Planned(PlanTime),
}

impl ListedTime {
    #[must_use]
    pub fn missing(self, word: Spelling) -> Option<Spelling> {
        match self {
            ListedTime::Stated(fact) if fact.source().word().lowered() == word => None,
            ListedTime::Stated(_) | ListedTime::Planned(_) => Some(word),
        }
    }

    #[must_use]
    pub fn worded(self, word: Spelling) -> Spelling {
        Spelling::new(
            self.missing(word)
                .map_or_else(|| self.to_string(), |word| format!("{word} {self}")),
        )
    }
}

impl From<ListedTime> for ShownTime {
    fn from(time: ListedTime) -> ShownTime {
        match time {
            ListedTime::Stated(fact) => ShownTime::Stated(fact),
            ListedTime::Planned(plan) => ShownTime::Planned(plan),
        }
    }
}

impl From<ListedTime> for ClockTime {
    fn from(time: ListedTime) -> ClockTime {
        match time {
            ListedTime::Stated(fact) => ClockTime::from(fact),
            ListedTime::Planned(plan) => ClockTime::from(plan),
        }
    }
}

impl fmt::Display for ListedTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListedTime::Stated(fact) => fact.fmt(formatter),
            ListedTime::Planned(plan) => plan.fmt(formatter),
        }
    }
}
