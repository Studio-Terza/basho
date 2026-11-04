use crate::words::Label;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonKind {
    PointOfInterest,
    AvoidsRoad,
    Shorter,
    BashoPath,
    Quieter,
}

impl From<&ReasonKind> for Label {
    fn from(kind: &ReasonKind) -> Label {
        match kind {
            ReasonKind::PointOfInterest => Label::new("Point of interest"),
            ReasonKind::AvoidsRoad => Label::new("Avoids a road"),
            ReasonKind::Shorter => Label::new("Shorter"),
            ReasonKind::BashoPath => Label::new("Bashō's path"),
            ReasonKind::Quieter => Label::new("Quieter"),
        }
    }
}
