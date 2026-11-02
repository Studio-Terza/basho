mod bug_report;
mod gesture;
mod halted;
mod history;
mod page_before;
mod progress_file;
mod set_now;
mod step_folds;
mod visit;
use std::cell::Cell;

use std::time::Duration;

use guide::clock::Minutes;

use dioxus::prelude::*;
use guide::clock::TripTime;
use guide::guidance::LegProgress;

use basho::app::session::staged::Staged;
use basho::app::session::{Session, SessionEvent};

use basho::app::step::Toward;

use crate::ui_check::{Drawing, Stage};
const LOCKED: Duration = Duration::from_hours(2);
const WALKED: Minutes = Minutes::new(3 * 60);

thread_local! {
    static CONFIRMED: Cell<Option<LegProgress>> = const { Cell::new(None) };
}

#[derive(Props, Clone, PartialEq)]
struct Unlock {
    stage: Stage,
    clock: TripTime,
}

impl Unlock {
    fn confirmed(self) -> Option<LegProgress> {
        let mut dom = VirtualDom::new_with_props(Self::render, self);
        dom.rebuild_in_place();
        CONFIRMED.take()
    }

    fn render(unlock: Unlock) -> Element {
        let Unlock { stage, clock } = unlock;
        let (itinerary, places, overview) = (
            stage.drawing().itinerary(),
            stage.drawing().places(),
            stage.drawing().overview(),
        );
        let session = Session::staged(itinerary, places, overview, &stage.clocked_at(clock));
        use_hook(|| SessionEvent::ConfirmProgress.applied(&session));
        CONFIRMED.set(Some(session.progress()));
        rsx! {}
    }
}

#[test]
fn the_first_fix_after_unlocking_is_judged_by_the_real_time_not_the_last_tick() {
    let trip = Drawing::BuiltIn.itinerary();
    let walking = Stage::on_schedule(&trip, WALKED).unwrap();
    let locked = walking
        .clone()
        .confirmed(LegProgress::default())
        .ticked_before(LOCKED);
    let unlocked = Unlock {
        stage: locked.clone(),
        clock: walking.now(),
    };
    let stale = Unlock {
        clock: locked.now(),
        stage: locked,
    };
    let step = |kept: LegProgress| (kept.leg(), kept.offset());
    let expected = Some(step(walking.progress()));
    assert_eq!(unlocked.confirmed().map(step), expected);
    assert_ne!(stale.confirmed().map(step), expected);
}

thread_local! {
    static TURNED: Cell<Option<bool>> = const { Cell::new(None) };
}

#[derive(Props, Clone, PartialEq)]
struct Turning {
    stage: Stage,
}

impl Turning {
    fn turns(self) -> Option<bool> {
        let mut dom = VirtualDom::new_with_props(Self::render, self);
        dom.rebuild_in_place();
        TURNED.take()
    }

    fn render(turning: Turning) -> Element {
        let stage = turning.stage;
        let drawing = stage.drawing();
        let session = Session::staged(
            drawing.itinerary(),
            drawing.places(),
            drawing.overview(),
            &stage,
        );
        TURNED.set(Some(session.page_turn(Toward::Next).is_some()));
        rsx! {}
    }
}

#[test]
fn a_swipe_steps_the_map_in_guide_mode_and_leaves_it_alone_in_explore() {
    let trip = Drawing::BuiltIn.itinerary();
    let walking = Stage::on_schedule(&trip, WALKED).expect("a walking stage");
    assert_eq!(
        Turning {
            stage: walking.clone()
        }
        .turns(),
        Some(true)
    );
    assert_eq!(
        Turning {
            stage: walking.exploring()
        }
        .turns(),
        Some(false)
    );
}
