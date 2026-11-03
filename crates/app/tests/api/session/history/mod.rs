use std::cell::RefCell;

use dioxus::prelude::*;
use guide::plane::Pixels;
use wiki::{EntryId, EntryKey, Home};

use basho::app::history::StepFolds;
use basho::app::history::{History, Mark, ScrollTop};
use basho::app::session::{Session, SessionEvent};
use basho::app::step::{Spot, Step, StepNumber, Toward};
use basho::app::visit::Screen;
use basho::app::visit::{Focus, Landing, Visit};
use guide::day_number::DayNumber;

use crate::ui_check::{Drawing, Stage};

const STEP: StepNumber = StepNumber::from_position(1);
const DAY: DayNumber = DayNumber::new(1);
const LEFT_AT: f64 = 412.5;
const READ_TO: f64 = 90.0;

#[derive(Clone, Debug, Default, PartialEq)]
struct Seen {
    after_back: Option<Mark>,
    after_forward: Option<Screen>,
    target: Option<EntryId>,
}

thread_local! {
    static SEEN: RefCell<Seen> = const { RefCell::new(Seen { after_back: None, after_forward: None, target: None }) };
}

#[derive(Props, Clone, PartialEq)]
struct Walk {
    stage: Stage,
}

impl Walk {
    fn scrolled(top: f64) -> ScrollTop {
        ScrollTop::new(Pixels::new(top))
    }

    fn seen(self) -> Seen {
        let mut dom = VirtualDom::new_with_props(Self::render, self);
        dom.rebuild_in_place();
        SEEN.take()
    }

    fn render(walk: Walk) -> Element {
        let stage = walk.stage;
        let session = Session::staged(
            stage.drawing().itinerary(),
            stage.drawing().places(),
            stage.drawing().overview(),
            &stage,
        );
        use_hook(|| {
            let target = session.wiki().find(&EntryKey::Shops);
            if let Some(entry) = target {
                session.browse(Spot::new(DAY, STEP));
                SessionEvent::Follow {
                    entry,
                    top: Self::scrolled(LEFT_AT),
                }
                .applied(&session);
                SessionEvent::TurnHistory {
                    toward: Toward::Previous,
                    top: Self::scrolled(READ_TO),
                }
                .applied(&session);
                let after_back = Some(Mark::new(
                    session.screen(),
                    session.visit(),
                    StepFolds::default(),
                    session.landing_top().unwrap_or_default(),
                ));
                SessionEvent::TurnHistory {
                    toward: Toward::Next,
                    top: Self::scrolled(LEFT_AT),
                }
                .applied(&session);
                SEEN.set(Seen {
                    after_back,
                    after_forward: Some(session.screen()),
                    target,
                });
            }
        });
        rsx! {}
    }
}

#[test]
fn back_returns_to_the_exact_screen_step_and_scroll_and_forward_undoes_it() {
    let itinerary = Drawing::BuiltIn.itinerary();
    let day = itinerary.day_number(DAY).expect("a day");
    let stage = Stage::on_screen(day, Screen::Today);
    let seen = Walk { stage }.seen();
    let back = seen.after_back.expect("went back");
    assert_eq!(back.screen(), Screen::Today);
    assert_eq!(back.visit().focus(), Focus::Browsing(Spot::new(DAY, STEP)));
    assert_eq!(back.top(), Walk::scrolled(LEFT_AT));
    assert_eq!(back.visit().landing(), Landing::Held);
    assert_eq!(seen.after_forward, seen.target.map(Screen::Wiki));
}

#[test]
fn leaving_a_page_after_going_back_drops_the_pages_ahead() {
    let browsed =
        Visit::default().jumped(Focus::Browsing(Spot::new(DAY, STEP)), Landing::Step(STEP));
    let here = Mark::new(
        Screen::Today,
        browsed,
        StepFolds::default(),
        Walk::scrolled(LEFT_AT),
    );
    let there = Mark::new(
        Screen::Map,
        Visit::default(),
        StepFolds::default(),
        Walk::scrolled(READ_TO),
    );
    let turn = History::default()
        .left(here.clone())
        .turned(Toward::Previous, there.clone())
        .expect("one page behind");
    assert_eq!(turn.mark(), &here);
    assert_eq!(turn.history().can_turn(Toward::Next), Some(Toward::Next));
    assert_eq!(turn.history().left(there).can_turn(Toward::Next), None);
}

#[test]
fn a_step_entry_opens_today_at_that_step_and_a_day_entry_at_its_top() {
    let itinerary = Drawing::BuiltIn.itinerary();
    let day = itinerary.day_number(DAY).expect("a day");
    let first_step = Step::of_day(day).into_iter().next().expect("a step");
    let leg = first_step.index();
    assert_eq!(
        Landing::of_home(Home::Step { day: DAY, leg }, &itinerary),
        Landing::Step(first_step.number())
    );
    assert_eq!(Landing::of_home(Home::Day(DAY), &itinerary), Landing::Top);
}
