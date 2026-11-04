use basho::app::Position;
use std::cell::Cell;

use dioxus::prelude::*;
use guide::clock::Minutes;

use basho::app::session::{Session, SessionEvent};
use basho::app::step::{DayPoint, Spot, Step, StepNumber};

use crate::ui_check::{Drawing, Stage};

const WALKED: Minutes = Minutes::new(3 * 60);
const EARLIER: usize = 0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Outcome {
    before: Option<StepNumber>,
    after_done: Option<StepNumber>,
    after_here: Option<StepNumber>,
    after_tracking: Option<StepNumber>,
}

thread_local! {
    static OUTCOME: Cell<Outcome> = const { Cell::new(Outcome { before: None, after_done: None, after_here: None, after_tracking: None }) };
}

#[derive(Props, Clone, PartialEq)]
struct Pressing {
    stage: Stage,
}

impl Pressing {
    fn outcome(self) -> Outcome {
        let mut dom = VirtualDom::new_with_props(Self::render, self);
        dom.rebuild_in_place();
        OUTCOME.take()
    }

    fn current(session: &Session) -> Option<StepNumber> {
        session
            .day_at(session.now())
            .and_then(|day| DayPoint::locate(&session.guidance(day), &Step::of_day(day)).current())
    }

    fn render(pressing: Pressing) -> Element {
        let stage = pressing.stage;
        let session = Session::staged(
            stage.drawing().itinerary(),
            stage.drawing().places(),
            stage.drawing().overview(),
            &stage,
        );
        use_hook(|| {
            let day = session
                .day_at(session.now())
                .map(guide::itinerary::Day::number);
            let before = Self::current(&session);
            let spot = |step: StepNumber| day.map(|day| Spot::new(day, step));
            if let Some(current) = before.and_then(spot) {
                SessionEvent::MarkDone(current).applied(&session);
            }
            let after_done = Self::current(&session);
            if let Some(earlier) = spot(StepNumber::from_position(Position::new(EARLIER))) {
                SessionEvent::PlaceHere(earlier).applied(&session);
            }
            let after_here = Self::current(&session);
            SessionEvent::ConfirmProgress.applied(&session);
            SessionEvent::TrackWalk.applied(&session);
            OUTCOME.set(Outcome {
                before,
                after_done,
                after_here,
                after_tracking: Self::current(&session),
            });
        });
        rsx! {}
    }
}

#[test]
fn done_advances_the_current_step_and_i_am_here_moves_it_even_backward() {
    let trip = Drawing::BuiltIn.itinerary();
    let stage = Stage::on_schedule(&trip, WALKED).unwrap();
    let outcome = Pressing { stage }.outcome();
    assert!(outcome.before > Some(StepNumber::from_position(Position::new(EARLIER))));
    let next = outcome
        .before
        .map(|before| StepNumber::from_position(Position::new(usize::from(before) + 1)));
    assert_eq!(outcome.after_done, next);
    assert_eq!(
        outcome.after_here,
        Some(StepNumber::from_position(Position::new(EARLIER)))
    );
    assert_eq!(outcome.after_tracking, outcome.after_here);
}
