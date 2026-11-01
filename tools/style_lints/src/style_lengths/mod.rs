mod length_literal;

use crate::SheetRule;
use crate::style_sheet::StyleSheet;
use basho_lints::LineNumber;
use basho_lints::Report;
use basho_lints::Violation;
use basho_lints::source_file::RelativePath;
use length_literal::LengthLiterals;
use newtype::Newtype;

pub struct StyleLengths;

impl StyleLengths {
    fn scan(path: &RelativePath, text: &str) -> Report {
        Report::new(
            "lengths and durations in CSS come from the Rust measures on :root; use a var(--…) token",
            text.lines()
                .enumerate()
                .flat_map(|(index, line)| {
                    line.parse::<LengthLiterals>()
                        .map_or_else(|never| match never {}, Newtype::into_inner)
                        .into_iter()
                        .map(move |literal| {
                            Violation::new(LineNumber::from_index(index).site(path), literal)
                        })
                })
                .collect(),
        )
    }
}

impl SheetRule for StyleLengths {
    fn report(sheet: &StyleSheet) -> Report {
        Self::scan(sheet.path(), sheet.text())
    }
}
