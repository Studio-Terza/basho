mod capture;
mod fact;
mod report_book;
mod report_number;
mod report_text;

use serde::{Deserialize, Serialize};

pub use capture::Capture;
pub use fact::Fact;
pub use report_book::{ReportBook, ReportStore};
pub use report_number::ReportNumber;
pub use report_text::ReportText;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Report {
    number: ReportNumber,
    note: ReportText,
    capture: Capture,
}

impl Report {
    #[must_use]
    pub fn new(number: ReportNumber, capture: Capture) -> Report {
        Report {
            number,
            capture,
            note: ReportText::default(),
        }
    }

    #[must_use]
    pub fn noted(self, note: ReportText) -> Report {
        Report { note, ..self }
    }

    #[must_use]
    pub fn number(&self) -> ReportNumber {
        self.number
    }

    #[must_use]
    pub fn note(&self) -> &ReportText {
        &self.note
    }

    #[must_use]
    pub fn capture(&self) -> &Capture {
        &self.capture
    }
}
