mod report_entry;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use newtype::Newtype;
use report::ReportText;

use crate::app::history::ScrollTop;
use crate::app::session::{Session, SessionEvent};
use report_entry::ReportEntry;

#[derive(Newtype)]
pub struct ReportsView<'session> {
    session: &'session Session,
}

impl IntoDynNode for ReportsView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let back = self.session.clone();
        let exporter = self.session.clone();
        let book = self.session.report_book();
        rsx! {
            article { class: "reports",
                button { class: "day-back place-back", onclick: move |event| back.leave(ScrollTop::from(&event)), "‹ Back" }
                header {
                    h1 { "Bug reports" }
                    p { class: "reports-how", "To save a report, hold two fingers on any screen." }
                }
                button { class: "report-tool report-export",
                    onclick: move |_| SessionEvent::ExportReports.applied(&exporter),
                    "Export to Files"
                }
                {book.notice().map(|notice| rsx! { p { class: "reports-notice", "{notice}" } })}
                {book.unreadable().map(|count| rsx! {
                    p { class: "reports-notice", "PROBLEM: {count} saved reports cannot be read." }
                })}
                {book.reports().is_empty().then(|| rsx! {
                    p { class: "reports-empty", "No reports yet." }
                })}
                ol { class: "report-list",
                    {book.reports().iter().rev().map(|report| {
                        let noter = self.session.clone();
                        let number = report.number();
                        rsx! { {ReportEntry::new(report, move |text: &ReportText| noter.note_report(number, text))} }
                    })}
                }
            }
        }
        .into_dyn_node()
    }
}
