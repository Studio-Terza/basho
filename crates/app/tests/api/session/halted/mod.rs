use std::cell::Cell;

use dioxus::prelude::*;
use guide::clock::Minutes;
use guide::guidance::timeline::Halt;

use basho::app::session::staged::Staged;
use basho::app::session::{Session, SessionEvent};

use crate::ui_check::{Drawing, Stage};

const WALKED: Minutes = Minutes::new(3 * 60);
const STOOD: std::time::Duration = std::time::Duration::from_mins(10);

thread_local! {
    static HALTED: Cell<Option<bool>> = const { Cell::new(None) };
}

#[derive(Props, Clone, PartialEq)]
struct Tracking {
    stage: Stage,
}

impl Tracking {
    fn halted(self) -> Option<bool> {
        let mut dom = VirtualDom::new_with_props(Self::render, self);
        dom.rebuild_in_place();
        HALTED.take()
    }

    fn render(tracking: Tracking) -> Element {
        let stage = tracking.stage;
        let drawing = stage.drawing();
        let session = Session::staged(
            drawing.itinerary(),
            drawing.places(),
            drawing.overview(),
            &stage,
        );
        use_hook(|| SessionEvent::TrackWalk.applied(&session));
        HALTED.set(Some(session.halted().is_some()));
        rsx! {}
    }
}

#[test]
fn a_walker_who_keeps_moving_is_not_halted_so_the_gps_keeps_its_best_accuracy() {
    let trip = Drawing::BuiltIn.itinerary();
    let walking = Stage::on_schedule(&trip, WALKED).unwrap();
    assert_eq!(Tracking { stage: walking }.halted(), Some(false));
}

#[test]
fn a_walker_who_has_stood_still_for_minutes_is_halted() {
    let trip = Drawing::BuiltIn.itinerary();
    let walking = Stage::on_schedule(&trip, WALKED).unwrap();
    let there = walking.position().unwrap();
    let halt = Halt::tracked(None, there, walking.now().earlier_by(STOOD));
    let standing = walking.halted(halt);
    assert_eq!(Tracking { stage: standing }.halted(), Some(true));
}
