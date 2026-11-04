use basho_lints::source_file::{RelativePath, SourceText};
use style_lints::SheetRule;
use style_lints::style_kinds::StyleKinds;
use style_lints::style_sheet::StyleSheet;

const ALLOWED: &str = ":root { --a: 0; }
.a:hover, .a:focus-visible, .b:disabled, .c:checked { color: red; }
[lang=\"ja\"] { color: red; }
.day-map > svg[role=\"img\"] { color: red; }
.ask[open] summary, .e::before, .e::-webkit-details-marker { color: red; }
html:has(.screen.fit) body, .step.focused:not(.current) .step-head { color: red; }
.f:is(.g, .h) .i, .j:where(.k:hover) { color: red; }";
const REFUSED: &str = "#step-3.current .step-n { color: red; }
#menu-day-3.current .day-when { color: red; }
.menu-day.current [aria-label=\"Day 9 goal 3\"] .goal-n { color: red; }
.a:nth-child(3), .a:nth-last-child(2), .a:nth-of-type(2n), .a:nth-last-of-type(1) { color: red; }
.b:first-child, .b:last-child, .b:only-child { color: red; }
.c:first-of-type, .c:last-of-type, .c:only-of-type { color: red; }
.d:has(#x), .d:not(:first-child), .d:is([data-day=\"3\"]), .d:where(.e:last-child) { color: red; }
.f:target, .f:empty, .f:lang(ja) { color: red; }
@layer probe { html:has(.screen.fit) #menu-day-3.current .mark { font-size: 0; } }
.menu-day { html:has(.screen.fit) &#menu-day-4.current .mark { font-size: 0; } }
.menu [role=\"[\"], html:has(.screen.fit) #menu-day-5.current .mark { font-size: 0; }
html:has(.screen.fit) .menu-day.current:has(+ .menu-day):not(:has(+ .menu-day + .menu-day)) .mark { font-size: 0; }
.g ~ .h { color: red; }
@scope (.menu) { .mark { color: red; } }
/* { */ #in-comment-trap { color: red; }
.time-check[hidden], [data-overlay] .mark { color: red; }";
const SHEET: &str = "main.css";
const AFTER_SELECTOR: &str = ": ";
const PICKERS: [&str; 31] = [
    "#step-3",
    "#menu-day-3",
    "[aria-label=…]",
    ":nth-child",
    ":nth-last-child",
    ":nth-of-type",
    ":nth-last-of-type",
    ":first-child",
    ":last-child",
    ":only-child",
    ":first-of-type",
    ":last-of-type",
    ":only-of-type",
    "#x",
    ":first-child",
    "[data-day=…]",
    ":last-child",
    ":target",
    ":empty",
    ":lang",
    "#menu-day-3",
    "nested",
    "#menu-day-5",
    "the",
    "the",
    "the",
    "the",
    "an",
    "#in-comment-trap",
    "[hidden=…]",
    "[data-overlay=…]",
];
const REAL_SHEET: &str = include_str!("../../../../../crates/app/assets/main.css");
const ANCHORS: &str = "#menu-day-3 .mark { color: red; }
.menu-day[aria-label=\"Day 3\"] { color: red; }
#goal-2 .goal-n { color: red; }
#step-4 .step-n { color: red; }
[data-day=\"3\"] { color: red; }";
const ANCHOR_COUNT: usize = 5;

#[test]
fn kind_and_state_selectors_pass() {
    let report = StyleKinds::report(&StyleSheet::parse(
        RelativePath::new(SHEET.to_owned()),
        SourceText::new(ALLOWED.to_owned()),
    ));
    assert_eq!(report.unclean().map(|report| report.to_string()), None);
}

#[test]
fn every_instance_picker_is_refused_once() {
    let report = StyleKinds::report(&StyleSheet::parse(
        RelativePath::new(SHEET.to_owned()),
        SourceText::new(REFUSED.to_owned()),
    ))
    .to_string();
    let found: Vec<&str> = report
        .lines()
        .skip(1)
        .filter_map(|line| line.rsplit(AFTER_SELECTOR).next()?.split(' ').next())
        .collect();
    assert_eq!(found, PICKERS, "{report}");
}

#[test]
fn the_real_sheet_reaches_no_id_aria_or_data_anchor() {
    let real = StyleKinds::report(&StyleSheet::parse(
        RelativePath::new(SHEET.to_owned()),
        SourceText::new(REAL_SHEET.to_owned()),
    ));
    let anchors = StyleKinds::report(&StyleSheet::parse(
        RelativePath::new(SHEET.to_owned()),
        SourceText::new(ANCHORS.to_owned()),
    ))
    .to_string();
    assert_eq!(real.unclean().map(|report| report.to_string()), None);
    assert_eq!(anchors.lines().skip(1).count(), ANCHOR_COUNT, "{anchors}");
}
