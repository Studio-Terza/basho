use basho_lints::SourceRule;
use basho_lints::ownership::Ownership;
use basho_lints::source_file::{RelativePath, SourceFile, SourceText};

const VIEW: &str = "
impl View {
fn render() {
    let open = use_signal(|| false);
    rsx! { div { ontouchstart: move |_| () } }
}
}
";
const PATHS: [&str; 2] = [
    "crates/app/src/app/screen/touch_surface/mod.rs",
    "crates/app/src/app/session/mod.rs",
];
const REPORTED: [&str; 2] = [
    "crates/app/src/app/session/mod.rs:5: ontouch outside crates/app/src/app/screen/touch_surface/",
    "crates/app/src/app/screen/touch_surface/mod.rs:4: use_signal outside crates/app/src/app/session/",
];

#[test]
fn touches_and_signals_only_in_their_owners() {
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
    let report = Ownership::report(&files).to_string();
    assert_eq!(report.lines().skip(1).collect::<Vec<_>>(), REPORTED);
}
