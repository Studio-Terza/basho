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
        Label::new(match kind {
            ReasonKind::PointOfInterest => "Point of interest",
            ReasonKind::AvoidsRoad => "Avoids a road",
            ReasonKind::Shorter => "Shorter",
            ReasonKind::BashoPath => "Bashō's path",
            ReasonKind::Quieter => "Quieter",
        })
    }
}
