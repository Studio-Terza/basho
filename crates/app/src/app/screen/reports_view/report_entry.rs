use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use report::{Fact, Report, ReportText};

pub struct ReportEntry<'view, Noted> {
    report: &'view Report,
    noted: Noted,
}

impl<'view, Noted: Fn(&ReportText) + 'static> ReportEntry<'view, Noted> {
    pub fn new(report: &'view Report, noted: Noted) -> Self {
        ReportEntry { report, noted }
    }

    fn fact(&self, name: &'static str, fact: Fact) -> Element {
        self.report
            .capture()
            .fact(fact)
            .map_or_else(|| rsx! {}, |text| rsx! { p { "{name}: {text}" } })
    }
}

impl<Noted: Fn(&ReportText) + 'static> IntoDynNode for ReportEntry<'_, Noted> {
    fn into_dyn_node(self) -> DynamicNode {
        let number = self.report.number();
        let screen = self.fact("Screen", Fact::Screen);
        let step = self.fact("Step", Fact::Step);
        let position = self.fact("Position", Fact::Position);
        let tracker = self.fact("Tracker", Fact::Tracker);
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
