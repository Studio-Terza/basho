use dioxus::prelude::*;
use wiki::entry::Entry;
use wiki::{EntryId, Home};

use crate::app::history::Fold;
use crate::app::history::{Mark, ScrollTop};
use crate::app::session::Session;
use crate::app::step::{DayPoint, Spot, Step, Toward};
use crate::app::visit::Landing;
use crate::app::visit::Screen;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HistoryEvent {
    Remember(ScrollTop),
    Follow { entry: EntryId, top: ScrollTop },
    OpenTab { screen: Screen, top: ScrollTop },
    Turn { toward: Toward, top: ScrollTop },
}

impl HistoryEvent {
    pub fn moved(self, session: &Session) {
        match self {
            HistoryEvent::Remember(top) => Self::remember(session, top),
            HistoryEvent::Follow { entry, top } => Self::followed(session, entry, top),
            HistoryEvent::OpenTab { screen, top } => Self::opened_tab(session, screen, top),
            HistoryEvent::Turn { toward, top } => Self::turned(session, toward, top),
        }
    }

    pub fn followed(session: &Session, entry: EntryId, top: ScrollTop) {
        Self::remember(session, top);
        session.wiki.entry(entry).and_then(Entry::home).map_or_else(
            || Self::read(session, entry),
            |home| Self::jump_home(session, home),
        );
    }

    pub fn opened_tab(session: &Session, screen: Screen, top: ScrollTop) {
        (*session.screen.peek() != screen).then(|| Self::remember(session, top));
        session.show(screen);
    }

    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    pub fn turned(session: &Session, toward: Toward, top: ScrollTop) {
        let turn = session
            .history
            .peek()
            .turned(toward, Self::mark(session, top));
        turn.into_iter().for_each(|turn| {
            let mut history = session.history;
            let mut screen = session.screen;
            let mut visit = session.visit;
            let mut landing = session.landing_top;
            let mark = turn.mark();
            let mut folds = session.folds;
            history.set(turn.history().clone());
            let held = visit.peek().jumped(mark.visit().focus(), Landing::Held);
            visit.set(held);
            folds.set(mark.folds().clone());
            screen.set(mark.screen());
            landing.set(Some(mark.top()));
        });
    }

    fn remember(session: &Session, top: ScrollTop) {
        let mut history = session.history;
        let left = history.peek().left(Self::mark(session, top));
        history.set(left);
    }

    fn mark(session: &Session, top: ScrollTop) -> Mark {
        Mark::new(
            *session.screen.peek(),
            *session.visit.peek(),
            session.folds.peek().clone(),
            top,
        )
    }

    fn read(session: &Session, entry: EntryId) {
        let mut landing = session.landing_top;
        let mut visit = session.visit;
        let jumped = {
            let before = *visit.peek();
            before.jumped(before.focus(), before.landing())
        };
        session.show(Screen::Wiki(entry));
        visit.set(jumped);
        landing.set(Some(ScrollTop::default()));
    }

    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    fn jump_home(session: &Session, home: Home) {
        let landing = Landing::of_home(home, session.itinerary());
        session.jump(home.home_day(), landing);
        landing.step(None).into_iter().for_each(|step| {
            let spot = Spot::new(home.home_day(), step);
            let mut folds = session.folds;
            let opened = session
                .itinerary()
                .day_number(spot.day())
                .map(|day| {
                    DayPoint::locate(&session.guidance(day), &Step::of_day(day))
                        .seen_from(day, session.day_at(session.now()))
                        .state_of(spot.step())
                })
                .filter(|state| folds.peek().fold(spot.day(), spot.step(), *state) == Fold::Closed)
                .map(|_| folds.peek().toggled(spot.day(), spot.step()));
            opened.into_iter().for_each(|opened| folds.set(opened));
        });
    }
}
