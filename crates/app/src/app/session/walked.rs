use std::cell::RefCell;

use guide::timeline::Halt;

use guide::readout::Trail;

use crate::app::session::tracker_log::DecisionLog;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Walked {
    trail: Trail,
    halt: Option<Halt>,
    decisions: DecisionLog,
}

thread_local! {
    static KEPT: RefCell<Walked> = RefCell::new(Walked::default());
}

impl Walked {
    pub fn new(trail: Trail, halt: Option<Halt>, decisions: DecisionLog) -> Walked {
        Walked {
            trail,
            halt,
            decisions,
        }
    }

    pub fn kept() -> Walked {
        KEPT.with_borrow(Walked::clone)
    }

    pub fn keep(self) {
        KEPT.set(self);
    }

    pub fn trail(&self) -> &Trail {
        &self.trail
    }

    pub fn halt(&self) -> Option<Halt> {
        self.halt
    }

    pub fn decisions(&self) -> &DecisionLog {
        &self.decisions
    }
}
