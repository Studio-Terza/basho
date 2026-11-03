use guide::content::Content;
use guide::times::{ListedTime, PlanTime};
use guide::{LoopNumber, LoopStop};

#[derive(Clone, Debug)]
pub struct LoopRow {
    number: LoopNumber,
    name: Content,
    planned: ListedTime,
}

impl LoopRow {
    #[must_use]
    pub fn number(&self) -> LoopNumber {
        self.number
    }

    #[must_use]
    pub fn name(&self) -> &Content {
        &self.name
    }

    #[must_use]
    pub fn planned(&self) -> ListedTime {
        self.planned
    }
}

impl From<&LoopStop<'_>> for LoopRow {
    fn from(stop: &LoopStop<'_>) -> LoopRow {
        LoopRow {
            number: stop.number(),
            name: AsRef::<Content>::as_ref(stop).clone(),
            planned: ListedTime::from(*AsRef::<PlanTime>::as_ref(stop)),
        }
    }
}
