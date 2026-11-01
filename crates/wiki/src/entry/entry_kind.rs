use guide::leg::StopKind;
use strum::{Display, EnumIter};

use crate::entry::Heading;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display)]
pub enum EntryKind {
    Place,
    Inn,
    Shop,
    Meal,
    Thing,
    Evening,
    #[strum(serialize = "Planned meal")]
    PlannedMeal,
    Day,
    Goal,
    Step,
    Guide,
}

impl EntryKind {
    #[must_use]
    pub fn of_stop(stop: StopKind) -> Option<EntryKind> {
        match stop {
            StopKind::Shop => Some(EntryKind::Shop),
            StopKind::Food => Some(EntryKind::Meal),
            StopKind::Sight => Some(EntryKind::Place),
            StopKind::Landmark | StopKind::Service => None,
        }
    }

    #[must_use]
    pub fn index_heading(self) -> Option<Heading> {
        match self {
            EntryKind::Day => Some(Heading::Days),
            EntryKind::Goal => Some(Heading::Goals),
            EntryKind::Step => Some(Heading::Steps),
            EntryKind::Place => Some(Heading::Places),
            EntryKind::Inn => Some(Heading::Inns),
            EntryKind::Shop => Some(Heading::Shops),
            EntryKind::Meal => Some(Heading::Restaurants),
            EntryKind::PlannedMeal => Some(Heading::Meals),
            EntryKind::Thing => Some(Heading::Things),
            EntryKind::Evening => Some(Heading::Evenings),
            EntryKind::Guide => None,
        }
    }
}
