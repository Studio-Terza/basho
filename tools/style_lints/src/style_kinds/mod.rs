mod boundary;
mod picked;

use crate::SheetRule;
use crate::style_sheet::StyleSheet;
use basho_lints::Report;
use basho_lints::Violation;
use boundary::SelectorParts;
use picked::Picked;

pub struct StyleKinds;

impl SheetRule for StyleKinds {
    fn report(sheet: &StyleSheet) -> Report {
        Report::new(
            "a style rule targets a kind of element and its shared states, never one instance: no ids, valued attributes, positions among siblings or other instance pickers",
            sheet
                .rules()
                .iter()
                .flat_map(|rule| {
                    SelectorParts::of_rule(rule)
                        .refused()
                        .into_iter()
                        .map(|part| {
                            Violation::new(
                                rule.line().site(sheet.path()),
                                Picked::new(rule.selector(), part),
                            )
                        })
                })
                .collect(),
        )
    }
}
