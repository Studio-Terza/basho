use std::collections::BTreeSet;

mod notice;
mod report_error;
mod report_store;

use notice::Notice;
use report_error::ReportError;
pub use report_store::ReportStore;

use newtype::Counter;

use crate::{Capture, Report, ReportCount, ReportNumber, ReportText};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReportBook {
    store: Option<ReportStore>,
    reports: Vec<Report>,
    unreadable: ReportCount,
    last: ReportNumber,
    notice: Option<Notice>,
}

impl ReportBook {
    #[must_use]
    pub fn opened(store: Option<ReportStore>) -> ReportBook {
        let numbers: BTreeSet<ReportNumber> =
            store.iter().flat_map(ReportStore::stored_numbers).collect();
        let reports: Vec<Report> = store
            .iter()
            .flat_map(|store| numbers.iter().filter_map(|number| store.read(*number)))
            .collect();
        ReportBook {
            unreadable: ReportCount::new(numbers.len() - reports.len()),
            last: numbers.last().copied().unwrap_or_default(),
            store,
            reports,
            notice: None,
        }
    }

    #[must_use]
    pub fn reports(&self) -> &[Report] {
        &self.reports
    }

    #[must_use]
    pub fn unreadable(&self) -> Option<ReportCount> {
        Some(self.unreadable).filter(|count| *count != ReportCount::default())
    }

    #[must_use]
    pub fn notice(&self) -> Option<&Notice> {
        self.notice.as_ref()
    }

    #[must_use]
    pub fn captured(self, capture: Capture) -> ReportBook {
        let number = self.last.incremented();
        let report = Report::new(number, capture);
        let notice = self.kept(&report);
        ReportBook {
            last: number,
            notice,
            reports: self.reports.into_iter().chain([report]).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn noted(self, number: ReportNumber, note: &ReportText) -> ReportBook {
        let reports: Vec<Report> = self
            .reports
            .iter()
            .cloned()
            .map(|report| {
                if report.number() == number {
                    report.noted(note.clone())
                } else {
                    report
                }
            })
            .collect();
        let notice = reports
            .iter()
            .find(|report| report.number() == number)
            .and_then(|report| self.kept(report));
        ReportBook {
            reports,
            notice,
            ..self
        }
    }

    #[must_use]
    pub fn exported(self) -> ReportBook {
        let notice = Self::notice_after(
            self.store_or_none()
                .and_then(|store| store.export(self.reports.clone())),
            Some(Notice::Exported(ReportCount::new(self.reports.len()))),
        );
        ReportBook { notice, ..self }
    }

    fn kept(&self, report: &Report) -> Option<Notice> {
        Self::notice_after(
            self.store_or_none().and_then(|store| store.keep(report)),
            None,
        )
    }

    fn store_or_none(&self) -> Result<&ReportStore, ReportError> {
        self.store.as_ref().ok_or(ReportError::NoHome)
    }

    fn notice_after(result: Result<(), ReportError>, done: Option<Notice>) -> Option<Notice> {
        result.map_or_else(
            |error| Some(Notice::Trouble(ReportText::rendered(&error))),
            |()| done,
        )
    }
}
