mod boundary;

use crate::SheetRule;
use crate::style_sheet::StyleSheet;
use basho_lints::Report;

pub struct StyleLengths;

impl SheetRule for StyleLengths {
    fn report(sheet: &StyleSheet) -> Report {
        Self::scan(sheet.path(), sheet.text())
    }
}
