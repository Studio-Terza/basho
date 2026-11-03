use crate::content::Content;
use crate::words::{Label, StarName};
use crate::{LoopStop, SightLoop};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reason {
    Star(Vec<StarName>),
    Loop(SightLoop),
    Sight,
    Landmark,
    Meal,
    Supplies,
}

impl Reason {
    #[must_use]
    pub fn label(&self) -> Label {
        Label::new(match self {
            Reason::Star(_) => "Star",
            Reason::Loop(_) => "Loop",
            Reason::Sight => "Sight",
            Reason::Landmark => "Landmark",
            Reason::Meal => "Meal",
            Reason::Supplies => "Shop",
        })
    }

    #[must_use]
    pub fn claims(&self) -> &[StarName] {
        match self {
            Reason::Star(points) => points,
            Reason::Loop(sights) => sights.as_ref(),
            Reason::Sight | Reason::Landmark | Reason::Meal | Reason::Supplies => &[],
        }
    }

    #[must_use]
    pub fn stops(&self) -> Vec<LoopStop<'_>> {
        match self {
            Reason::Loop(sights) => Vec::from(sights),
            Reason::Star(_)
            | Reason::Sight
            | Reason::Landmark
            | Reason::Meal
            | Reason::Supplies => Vec::new(),
        }
    }

    pub fn sight_names(&self) -> impl Iterator<Item = &Content> {
        match self {
            Reason::Loop(sights) => Some(sights.names()),
            Reason::Star(_)
            | Reason::Sight
            | Reason::Landmark
            | Reason::Meal
            | Reason::Supplies => None,
        }
        .into_iter()
        .flatten()
    }
}
