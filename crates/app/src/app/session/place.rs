use std::cell::RefCell;

use guide::guidance::LegProgress;

use crate::app::history::History;
use crate::app::screen::Screen;
use crate::app::session::visit::Focus;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Place {
    screen: Screen,
    focus: Focus,
    progress: LegProgress,
    history: History,
}

thread_local! {
    static KEPT: RefCell<Place> = RefCell::new(Place::default());
}

impl Place {
    pub fn new(screen: Screen, focus: Focus, progress: LegProgress, history: History) -> Place {
        Place {
            screen,
            focus,
            progress,
            history,
        }
    }

    pub fn kept() -> Place {
        KEPT.with_borrow(Place::clone)
    }

    pub fn keep(self) {
        KEPT.set(self);
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn focus(&self) -> Focus {
        self.focus
    }

    pub fn progress(&self) -> LegProgress {
        self.progress
    }
}
