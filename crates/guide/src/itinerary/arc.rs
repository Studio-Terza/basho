use crate::words::Spelling;
use serde::Deserialize;

use crate::content::Content;
use crate::day_number::DayNumber;
use crate::itinerary::arc_number::ArcNumber;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arc {
    number: ArcNumber,
    title: Content,
    start: Spelling,
    days: Vec<DayNumber>,
}

impl Arc {
    #[must_use]
    pub fn number(&self) -> ArcNumber {
        self.number
    }

    #[must_use]
    pub fn title(&self) -> &Content {
        &self.title
    }

    #[must_use]
    pub fn start(&self) -> &Spelling {
        &self.start
    }

    #[must_use]
    pub fn day_numbers(&self) -> &[DayNumber] {
        &self.days
    }
}
