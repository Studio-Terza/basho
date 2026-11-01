use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fact, ReportText};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Capture {
    time: ReportText,
    facts: BTreeMap<Fact, ReportText>,
    decisions: Vec<ReportText>,
}

impl Capture {
    #[must_use]
    pub fn new(time: ReportText) -> Capture {
        Capture {
            time,
            facts: BTreeMap::new(),
            decisions: Vec::new(),
        }
    }

    #[must_use]
    pub fn knowing(self, fact: Fact, text: Option<ReportText>) -> Capture {
        Capture {
            facts: self
                .facts
                .into_iter()
                .chain(text.map(|text| (fact, text)))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn after_decisions(self, decisions: Vec<ReportText>) -> Capture {
        Capture { decisions, ..self }
    }

    #[must_use]
    pub fn time(&self) -> &ReportText {
        &self.time
    }

    #[must_use]
    pub fn fact(&self, fact: Fact) -> Option<&ReportText> {
        self.facts.get(&fact)
    }

    #[must_use]
    pub fn decisions(&self) -> &[ReportText] {
        &self.decisions
    }
}
