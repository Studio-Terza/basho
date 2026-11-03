use std::cell::RefCell;

use dioxus::prelude::*;
use guide::day_number::DayNumber;
use guide::leg::LegIndex;
use guide::map_frame::Pixels;
use report::{Fact, ReportText};
use wiki::EntryKey;

use basho::app::history::{Fold, ScrollTop};
use basho::app::screen::Screen;
use basho::app::session::{Session, SessionEvent};
use basho::app::step::{DayPoint, Step, StepNumber, Toward};

use crate::ui_check::{Drawing, Stage};

const DAY: DayNumber = DayNumber::new(1);
const READ_TO: f64 = 90.0;
const LATER_STEP: usize = 1;

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
    FollowAStepAndGoBack,
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

    fn fold_of(session: &Session, step: StepNumber) -> Fold {
        let day = session.itinerary().day_number(DAY).expect("a day");
        let state = DayPoint::locate(&session.guidance(day), &Step::of_day(day))
            .seen_from(day, session.day_at(session.now()))
            .state_of(step);
        session.fold(DAY, step, state)
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
                Visit::FollowAStepAndGoBack => {
                    let step = session
                        .wiki()
                        .find(&EntryKey::Step {
                            day: DAY,
                            leg: LegIndex::new(LATER_STEP),
                        })
                        .expect("the later step has a page");
                    let later = StepNumber::from_position(LATER_STEP);
                    seen.folds.push(Self::fold_of(&session, later));
                    SessionEvent::Follow {
                        entry: step,
                        top: Self::scrolled(),
                    }
                    .applied(&session);
                    seen.folds.push(Self::fold_of(&session, later));
                    SessionEvent::TurnHistory {
                        toward: Toward::Previous,
                        top: Self::scrolled(),
                    }
                    .applied(&session);
                    seen.folds.push(Self::fold_of(&session, later));
                }
            }
            SEEN.set(seen);
        });
        rsx! {}
    }
}

#[test]
fn a_report_filed_from_a_wiki_page_names_it_and_back_returns_to_it() {
    let trip = Drawing::BuiltIn.itinerary();
    let wiki = Drawing::BuiltIn.wiki();
    let day = trip.day_number(DAY).expect("a day");
    let page = wiki.find(&EntryKey::Shops).expect("the shops page");
    let stage = Stage::on_screen(day, Screen::Wiki(page)).keeping_reports_in(
        report::ReportStore::in_home(storage_fake::MemoryStorage::home()),
    );
    let seen = Walk::seen(stage, Visit::ReportFromAWikiPage);
    assert_eq!(seen.on_reports, Some(Screen::Reports));
    assert_eq!(seen.after_back, seen.wiki_page);
    let said = seen.reported_screen.expect("the report names the screen");
    assert!(said.to_string().contains("Shops"), "{said}");
}

#[test]
fn back_after_following_a_step_link_puts_the_step_fold_as_it_was() {
    let trip = Drawing::BuiltIn.itinerary();
    let day = trip.day_number(DAY).expect("a day");
    let seen = Walk::seen(
        Stage::on_screen(day, Screen::Today),
        Visit::FollowAStepAndGoBack,
    );
    let [before, during, after] = seen.folds[..] else {
        panic!("three folds, not {:?}", seen.folds);
    };
    assert_ne!(before, during);
    assert_eq!(after, before);
}
