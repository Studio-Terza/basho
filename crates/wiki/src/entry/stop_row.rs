use guide::content::Content;
use guide::day_number::DayNumber;
use guide::leg::Cutoff;
use guide::place::Fact;
use guide::times::ListedTime;

use crate::EntryId;
use crate::entry::Opening;

#[derive(Clone, Debug)]
pub struct StopRow {
    name: Content,
    entry: Option<EntryId>,
    day: DayNumber,
    expected: Option<ListedTime>,
    cutoff: Option<Cutoff>,
    detail: Option<Fact>,
    opening: Option<Opening>,
}

impl StopRow {
    #[must_use]
    pub fn new(name: Content, entry: Option<EntryId>, day: DayNumber) -> StopRow {
        StopRow {
            name,
            entry,
            day,
            expected: None,
            cutoff: None,
            detail: None,
            opening: None,
        }
    }

    #[must_use]
    pub fn arriving(self, expected: Option<ListedTime>, opening: Option<Opening>) -> StopRow {
        StopRow {
            expected,
            opening,
            ..self
        }
    }

    #[must_use]
    pub fn told(self, cutoff: Option<Cutoff>, detail: Option<Fact>) -> StopRow {
        StopRow {
            cutoff,
            detail,
            ..self
        }
    }

    #[must_use]
    pub fn stop_name(&self) -> &Content {
        &self.name
    }

    #[must_use]
    pub fn stop_entry(&self) -> Option<EntryId> {
        self.entry
    }

    #[must_use]
    pub fn visit_day(&self) -> DayNumber {
        self.day
    }

    #[must_use]
    pub fn expected(&self) -> Option<ListedTime> {
        self.expected
    }

    #[must_use]
    pub fn closing(&self) -> Option<&Cutoff> {
        self.cutoff.as_ref()
    }

    #[must_use]
    pub fn detail(&self) -> Option<&Fact> {
        self.detail.as_ref()
    }

    #[must_use]
    pub fn opening(&self) -> Option<Opening> {
        self.opening
    }
}
