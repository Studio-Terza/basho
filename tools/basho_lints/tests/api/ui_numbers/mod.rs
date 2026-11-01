use basho_lints::SourceRule;
use basho_lints::source_file::{RelativePath, SourceFile, SourceText};
use basho_lints::ui_numbers::UiNumbers;

const VIEW: &str = "
pub struct V;
impl V {
const GAP: f64 = 6.0;
fn half(width: f64) -> f64 { width * 0.5 + 1.0 }
fn rows() -> [u8; 4] { [0; 4] }
fn draw() { rsx! { circle { r: 4, cx: 0 } } }
}
";
const PATHS: [&str; 2] = [
    "crates/app/src/app/screen/v.rs",
    "crates/layout/src/scale/v.rs",
];
const REPORTED: [&str; 2] = [
    "crates/app/src/app/screen/v.rs:4: 6.0",
    "crates/app/src/app/screen/v.rs:7: 4",
];

#[test]
fn flags_ui_numbers_outside_the_measures_but_not_structure_or_lengths() {
    let files: Vec<SourceFile> = PATHS
        .iter()
        .map(|path| {
            SourceFile::parse(
                RelativePath::new((*path).to_owned()),
                SourceText::new(VIEW.to_owned()),
            )
            .expect("valid rust")
        })
        .collect();
    let report = UiNumbers::report(&files).to_string();
    assert_eq!(report.lines().skip(1).collect::<Vec<_>>(), REPORTED);
}
