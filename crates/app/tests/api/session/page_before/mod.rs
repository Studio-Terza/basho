use std::cell::RefCell;

use dioxus::prelude::*;
use guide::itinerary::DayNumber;
use guide::map_frame::Pixels;
use report::{Fact, ReportText};
use wiki::EntryKey;

use basho::app::history::{Fold, ScrollTop};
use basho::app::screen::Screen;
use basho::app::session::{Session, SessionEvent};

use crate::ui_check::{Drawing, Stage};

const DAY: DayNumber = DayNumber::new(1);
const READ_TO: f64 = 90.0;

#[derive(Clone, Debug, Default, PartialEq)]
struct Seen {
    wiki_page: Option<Screen>,
    on_reports: Option<Screen>,
    after_back: Option<Screen>,
    reported_screen: Option<ReportText>,
    folds: Vec<Fold>,
}

thread_local! {
    static SEEN: RefCell<Seen> = RefCell::new(Seen::default());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Visit {
    ReportFromAWikiPage,
}

#[derive(Props, Clone, PartialEq)]
struct Walk {
    stage: Stage,
    visit: Visit,
}

impl Walk {
    fn seen(stage: Stage, visit: Visit) -> Seen {
        let mut dom = VirtualDom::new_with_props(Self::render, Walk { stage, visit });
        dom.rebuild_in_place();
        SEEN.take()
    }

    fn scrolled() -> ScrollTop {
        ScrollTop::new(Pixels::new(READ_TO))
    }

    fn render(walk: Walk) -> Element {
        let Walk { stage, visit } = walk;
        let session = Session::staged(
            stage.drawing().itinerary(),
            stage.drawing().places(),
            stage.drawing().overview(),
            &stage,
        );
        use_hook(|| {
            let mut seen = Seen::default();
            match visit {
                Visit::ReportFromAWikiPage => {
                    seen.wiki_page = Some(session.screen());
                    SessionEvent::CaptureReport.applied(&session);
                    seen.on_reports = Some(session.screen());
                    seen.reported_screen = session
                        .report_book()
                        .reports()
                        .first()
                        .and_then(|report| report.capture().fact(Fact::Screen).cloned());
                    session.leave(Self::scrolled());
                    seen.after_back = Some(session.screen());
                }
            }
            SEEN.set(seen);
        });
        rsx! {}
    }
}

#[test]
fn a_report_filed_from_a_wiki_page_names_it_and_back_returns_to_it() {
    let reports = tempfile::tempdir().unwrap();
    let trip = Drawing::BuiltIn.itinerary();
    let wiki = Drawing::BuiltIn.wiki();
    let day = trip.day_number(DAY).expect("a day");
    let page = wiki.find(&EntryKey::Shops).expect("the shops page");
    let stage = Stage::on_screen(day, Screen::Wiki(page)).keeping_reports_in(
        report::ReportStore::located_at(
            reports.path().join("reports"),
            reports.path().join("Documents"),
        ),
    );
    let seen = Walk::seen(stage, Visit::ReportFromAWikiPage);
    assert_eq!(seen.on_reports, Some(Screen::Reports));
    assert_eq!(seen.after_back, seen.wiki_page);
    let said = seen.reported_screen.expect("the report names the screen");
    assert!(said.to_string().contains("Shops"), "{said}");
}
