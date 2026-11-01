use basho_lints::SourceRule;
use basho_lints::class_sources::ClassSources;
use basho_lints::source_file::{RelativePath, SourceFile, SourceText};

const CLOSED: &str = "
enum Kind { Walk, Ride }
impl Kind {
fn class(self) -> &'static str { match self { Kind::Walk => \"walk\", Kind::Ride => \"ride\" } }
}
impl V {
fn draw(&self, kind: Kind) -> Element {
    let pose: PoseStyle = self.session.menu_style();
    rsx! {
        div { class: \"step {kind.class()}\", id: \"x-{self.n}\", \"aria-label\": \"{self.n}\", style: \"{pose}\", role: \"group\",
            span { class: self.kind.class(), lang: \"ja\", \"data-overlay\": \"menu\", style: \"display: block\", disabled: reach.flag(), href: \"#{self.n}\", onclick: move |_| go(self.n) }
        }
    }
}
}
";
const OPEN: &str = "
struct DayName { number: DayNumber }
impl Classed for DayName {}
impl From<DayName> for &'static str {
fn from(name: DayName) -> &'static str { \"day-row\" }
}
impl DayName {
pub fn class(self) -> &'static str {
    if self.number == DayNumber::new(7) { \"day-row far\" } else { \"day-row\" }
}
}
impl V {
fn draw(&self, number: DayNumber, name: DayName) -> Element {
    let far: &'static str = Box::leak(format!(\"d{number}\").into_boxed_str());
    let scale = if number == DayNumber::new(2) { \"0\" } else { \"1\" };
    rsx! {
        li { class: if self.current { \"menu-day current\" } else { \"menu-day\" }, id: \"menu-day-{number}\", style: \"--mark-scale: {scale}\", role: if number == DayNumber::new(1) { \"note\" } else { \"listitem\" },
            button { class: name.class(),
                span { class: if number == DayNumber::new(6) { \"day-when far\" } else { \"day-when\" },
                    span { class: \"mark {far}\", style: if number == DayNumber::new(8) { \"font-size: 0\" } else { \"\" }, \"data-day\": \"{number}\" }
                    p { class: \"time-check\", hidden: step.leg().minutes() <= Minutes::new(2), disabled: target.is_none() }
                    svg { path { transform: \"rotate({turned})\" } }
                }
            }
        }
    }
}
}
";
const OPEN_COUNT: usize = 13;
const PATH: &str = "crates/app/src/app/screen/v.rs";

#[test]
fn closed_markup_passes_and_instance_data_is_refused() {
    let parsed = |text: &str| {
        SourceFile::parse(
            RelativePath::new(PATH.to_owned()),
            SourceText::new(text.to_owned()),
        )
        .unwrap()
    };
    let closed = ClassSources::report(&[parsed(CLOSED)]);
    let open = ClassSources::report(&[parsed(OPEN)]).to_string();
    assert!(closed.is_clean(), "{closed}");
    assert_eq!(open.lines().skip(1).count(), OPEN_COUNT, "{open}");
}
