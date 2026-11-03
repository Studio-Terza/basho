use guide::map_frame::Pixels;

use crate::app::history::{ScrollTop, StepFolds};
use crate::app::visit::Screen;
use crate::app::visit::Visit;

#[derive(Clone, Debug, PartialEq)]
pub struct Mark {
    screen: Screen,
    visit: Visit,
    folds: StepFolds,
    top: ScrollTop,
}

impl Mark {
    #[must_use]
    pub fn new(screen: Screen, visit: Visit, folds: StepFolds, top: ScrollTop) -> Mark {
        Mark {
            screen,
            visit,
            folds,
            top,
        }
    }

    #[must_use]
    pub fn screen(&self) -> Screen {
        self.screen
    }

    #[must_use]
    pub fn visit(&self) -> Visit {
        self.visit
    }

    #[must_use]
    pub fn folds(&self) -> &StepFolds {
        &self.folds
    }

    #[must_use]
    pub fn top(&self) -> ScrollTop {
        self.top
    }

    #[must_use]
    pub fn top_pixels(&self) -> Pixels {
        *self.top.as_ref()
    }
}
