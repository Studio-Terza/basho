use std::fmt;

use dioxus::prelude::*;
use wiki::EntryId;

use crate::app::visit::pane::Pane;
use crate::app::visit::tab::Tab;
use crate::app::visit::turning::Turning;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Screen {
    #[default]
    Map,
    Today,
    Wiki(EntryId),
    Reports,
    Trip,
}

impl Screen {
    pub const STYLE: Asset = asset!("/assets/main.css");
    pub const TABS: [Screen; 2] = [Screen::Map, Screen::Today];

    #[must_use]
    pub fn wiki_entry(&self) -> Option<EntryId> {
        match self {
            Screen::Wiki(id) => Some(*id),
            Screen::Map | Screen::Today | Screen::Reports | Screen::Trip => None,
        }
    }

    #[must_use]
    pub fn turning(&self) -> Turning {
        match self {
            Screen::Map => Turning::Steps,
            Screen::Today => Turning::Days,
            Screen::Wiki(_) => Turning::History,
            Screen::Reports | Screen::Trip => Turning::Still,
        }
    }

    #[must_use]
    pub fn tab(&self) -> Tab {
        match self {
            Screen::Map => Tab::Map,
            Screen::Today | Screen::Wiki(_) | Screen::Reports | Screen::Trip => Tab::Today,
        }
    }

    #[must_use]
    pub fn pane(&self) -> Pane {
        match self {
            Screen::Map => Pane::Map,
            Screen::Today | Screen::Wiki(_) => Pane::Today,
            Screen::Trip => Pane::Trip,
            Screen::Reports => Pane::Reports,
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Screen::Map => formatter.write_str("map"),
            Screen::Today => formatter.write_str("today"),
            Screen::Wiki(id) => write!(formatter, "wiki page {}", usize::from(*id)),
            Screen::Reports => formatter.write_str("bug reports"),
            Screen::Trip => formatter.write_str("trip"),
        }
    }
}
