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
        match self {
            FeatureKind::Road => Label::new("the road"),
            FeatureKind::River => Label::new("the river"),
            FeatureKind::Stream => Label::new("the stream"),
            FeatureKind::Railway => Label::new("the railway"),
            FeatureKind::Station => Label::new("the station"),
            FeatureKind::Lake => Label::new("the lake"),
            FeatureKind::Coast => Label::new("the coast"),
            FeatureKind::Peak => Label::new("the mountain"),
            FeatureKind::Place => Label::new("the town"),
            FeatureKind::Other => Label::new("the landmark"),
        }
    }
}
