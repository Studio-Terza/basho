use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::words::Label;
use report::{Fact, Report, ReportText};

pub struct ReportEntry<'view, Noted> {
    report: &'view Report,
    noted: Noted,
}

impl<'view, Noted: Fn(&ReportText) + 'static> ReportEntry<'view, Noted> {
    pub fn new(report: &'view Report, noted: Noted) -> Self {
        ReportEntry { report, noted }
    }

    fn fact(&self, name: Label, fact: Fact) -> Element {
        self.report
            .capture()
            .fact(fact)
            .map_or_else(|| rsx! {}, |text| rsx! { p { "{name}: {text}" } })
    }
}

impl<Noted: Fn(&ReportText) + 'static> IntoDynNode for ReportEntry<'_, Noted> {
    fn into_dyn_node(self) -> DynamicNode {
        let number = self.report.number();
        let screen = self.fact(Label::new("Screen"), Fact::Screen);
        let step = self.fact(Label::new("Step"), Fact::Step);
        let position = self.fact(Label::new("Position"), Fact::Position);
        let tracker = self.fact(Label::new("Tracker"), Fact::Tracker);
        let noted = self.noted;
        let capture = self.report.capture();
        let decisions = capture.decisions().len();
        rsx! {
            li { class: "report", key: "{number}",
                h2 { "Report {number}" }
                p { class: "report-time", "{capture.time()}" }
                {screen}
                {step}
                {position}
                {tracker}
                p { class: "report-decisions", "Tracker changes kept: {decisions}" }
                label { class: "report-note-label",
                    "Note (optional)"
                    textarea { class: "report-note", "aria-label": "Note for report {number}",
                        oninput: move |event| noted(&ReportText::new(event.value())),
                        "{self.report.note()}"
                    }
                }
            }
        }
        .into_dyn_node()
    }
}
