use basho_lints::source_file::{RelativePath, SourceText};
use style_lints::SheetRule;
use style_lints::style_motion::StyleMotion;
use style_lints::style_sheet::StyleSheet;

const SHEET: &str =
    ".menu { opacity: var(--shade); transition: opacity var(--follow) var(--ease); }
@starting-style { .menu { opacity: var(--rest-shade); } }
.menu.closing { opacity: 0; }
.menu-title { opacity: 0.5; }
.fade { transition: opacity 1s; transform: translateX(-100%); }
@keyframes slide-in { from { opacity: 0; } }
.page { animation: slide-in var(--slide); }";
const REPORTED: [&str; 6] = [
    "main.css:3: .menu.closing moves .menu from outside its own rule",
    "main.css:5: .fade transition: opacity 1s is not a pose var",
    "main.css:5: .fade transform: translateX(-100%) is not a pose var",
    "main.css:5: .fade transition: opacity 1s does not take its time from the pose",
    "main.css:6: @keyframes slide-in replays whenever its animation is reapplied",
    "main.css:7: .page sets animation",
];

#[test]
fn flags_keyframes_animations_state_rules_and_literal_motion() {
    let report = StyleMotion::report(&StyleSheet::parse(
        RelativePath::new("main.css".to_owned()),
        SourceText::new(SHEET.to_owned()),
    ))
    .to_string();
    assert_eq!(report.lines().skip(1).collect::<Vec<_>>(), REPORTED);
}
