use crate::LoopStop;
use crate::itinerary::day::goal::Reason;
use crate::leg::Mode;
use crate::words::{Label, StarName};

#[derive(Clone, Debug)]
pub enum Purpose {
    Ride(Mode),
    Inn,
    Aim(Reason),
}

impl Purpose {
    #[must_use]
    pub fn label(&self) -> Label {
        match self {
            Purpose::Ride(mode) => mode.label(),
            Purpose::Inn => Label::new("Inn"),
            Purpose::Aim(reason) => reason.label(),
        }
    }

    #[must_use]
    pub fn claims(&self) -> &[StarName] {
        match self {
            Purpose::Aim(reason) => reason.claims(),
            Purpose::Ride(_) | Purpose::Inn => &[],
        }
    }

    #[must_use]
    pub fn stops(&self) -> Vec<LoopStop<'_>> {
        match self {
            Purpose::Aim(reason) => reason.stops(),
            Purpose::Ride(_) | Purpose::Inn => Vec::new(),
        }
    }
}
