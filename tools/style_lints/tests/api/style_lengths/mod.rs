use basho_lints::source_file::{RelativePath, SourceText};
use style_lints::SheetRule;
use style_lints::style_lengths::StyleLengths;
use style_lints::style_sheet::StyleSheet;

const SHEET: &str = ".a { padding: 6px var(--gap-snug); color: #1f1d1a; }
.b { transition: opacity 0.32s; inset: 0px; width: 2rem; }";
const REPORTED: [&str; 2] = ["main.css:1: 6px", "main.css:2: 0.32s"];

#[test]
fn flags_raw_lengths_and_durations_but_not_zero_rem_or_colours() {
    let report = StyleLengths::report(&StyleSheet::parse(
        RelativePath::new("main.css".to_owned()),
        SourceText::new(SHEET.to_owned()),
    ))
    .to_string();
    assert_eq!(report.lines().skip(1).collect::<Vec<_>>(), REPORTED);
}
