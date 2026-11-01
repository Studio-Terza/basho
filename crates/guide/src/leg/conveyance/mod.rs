mod train;
mod walk;

use crate::words::Label;
use serde::Deserialize;
use strum::EnumDiscriminants;

pub use train::{Headsign, LineName, Platform, Train, TrainKind};
use walk::Walk;
pub use walk::{ReasonKind, RouteReason};

#[derive(Debug, Deserialize, EnumDiscriminants)]
#[strum_discriminants(name(Mode))]
#[serde(rename_all = "lowercase")]
pub enum Conveyance {
    Walk(Walk),
    Train(Train),
    Bus,
    Boat,
    Car,
}

impl Conveyance {
    #[must_use]
    pub fn train(&self) -> Option<&Train> {
        match self {
            Conveyance::Train(train) => Some(train),
            Conveyance::Walk(_) | Conveyance::Bus | Conveyance::Boat | Conveyance::Car => None,
        }
    }

    #[must_use]
    pub fn walk(&self) -> Option<&Walk> {
        match self {
            Conveyance::Walk(walk) => Some(walk),
            Conveyance::Train(_) | Conveyance::Bus | Conveyance::Boat | Conveyance::Car => None,
        }
    }
}

impl Mode {
    #[must_use]
    pub fn label(self) -> Label {
        Label::new(match self {
            Mode::Walk => "Walk",
            Mode::Train => "Train",
            Mode::Bus => "Bus",
            Mode::Boat => "Boat",
            Mode::Car => "Car",
        })
    }
}
