pub mod style_kinds;
pub mod style_lengths;
pub mod style_motion;
pub mod style_sheet;

use style_sheet::StyleSheet;

pub trait SheetRule {
    fn report(sheet: &StyleSheet) -> basho_lints::Report;
}
