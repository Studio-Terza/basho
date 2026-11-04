mod mark;
mod scroll_top;
mod step_folds;
mod turn;

use crate::app::step::Toward;

pub use mark::Mark;
pub use scroll_top::ScrollTop;
pub use step_folds::{Fold, StepFolds};
pub use turn::Turn;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct History {
    behind: Vec<Mark>,
    ahead: Vec<Mark>,
}

impl History {
    #[must_use]
    pub fn left(&self, mark: Mark) -> History {
        History {
            behind: self.behind.iter().cloned().chain([mark]).collect(),
            ahead: Vec::new(),
        }
    }

    #[must_use]
    pub fn origin(&self) -> Option<Mark> {
        self.behind.last().cloned()
    }

    #[must_use]
    pub fn can_turn(&self, toward: Toward) -> Option<Toward> {
        self.source(toward).first().map(|_| toward)
    }

    #[must_use]
    pub fn turned(&self, toward: Toward, here: Mark) -> Option<Turn> {
        self.source(toward).split_last().map(|(mark, rest)| {
            let kept = rest.to_vec();
            let passed: Vec<Mark> = self
                .source(toward.reversed())
                .iter()
                .cloned()
                .chain([here])
                .collect();
            Turn::new(Self::oriented(toward, kept, passed), mark.clone())
        })
    }

    fn oriented(toward: Toward, kept: Vec<Mark>, passed: Vec<Mark>) -> History {
        match toward {
            Toward::Previous => History {
                behind: kept,
                ahead: passed,
            },
            Toward::Next => History {
                behind: passed,
                ahead: kept,
            },
        }
    }

    fn source(&self, toward: Toward) -> &[Mark] {
        match toward {
            Toward::Previous => &self.behind,
            Toward::Next => &self.ahead,
        }
    }
}
