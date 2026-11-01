use crate::app::history::{History, Mark};

#[derive(Clone, Debug, PartialEq)]
pub struct Turn {
    history: History,
    mark: Mark,
}

impl Turn {
    #[must_use]
    pub fn new(history: History, mark: Mark) -> Turn {
        Turn { history, mark }
    }

    #[must_use]
    pub fn history(&self) -> &History {
        &self.history
    }

    #[must_use]
    pub fn mark(&self) -> &Mark {
        &self.mark
    }
}
