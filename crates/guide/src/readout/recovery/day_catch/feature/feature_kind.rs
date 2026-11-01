use crate::words::Label;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeatureKind {
    Road,
    River,
    Stream,
    Railway,
    Station,
    Lake,
    Coast,
    Peak,
    Place,
    #[serde(other)]
    Other,
}

impl FeatureKind {
    #[must_use]
    pub fn phrase(self) -> Label {
        Label::new(match self {
            FeatureKind::Road => "the road",
            FeatureKind::River => "the river",
            FeatureKind::Stream => "the stream",
            FeatureKind::Railway => "the railway",
            FeatureKind::Station => "the station",
            FeatureKind::Lake => "the lake",
            FeatureKind::Coast => "the coast",
            FeatureKind::Peak => "the mountain",
            FeatureKind::Place => "the town",
            FeatureKind::Other => "the landmark",
        })
    }
}
