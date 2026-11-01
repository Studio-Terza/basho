mod trip_round;
mod trip_scope;

use std::rc::Rc;

use dioxus::prelude::*;
use guide::{Trip, TripError};

use crate::app::session::trip_shelf::TripShelf;

use crate::app::session::trip_shelf::trip_text::TripText;
use trip_round::TripRound;
use trip_scope::TripScope;

#[derive(Clone, Debug, Default)]
pub enum ShownTrip {
    #[default]
    Nothing,
    Failed(Rc<TripError>),
    Opened {
        round: TripRound,
        source: Rc<TripText>,
        trip: Rc<Trip>,
    },
}

impl ShownTrip {
    #[must_use]
    pub fn first(source: Result<TripText, TripError>) -> ShownTrip {
        source
            .and_then(|source| {
                let trip = source.trip()?;
                Ok(ShownTrip::Opened {
                    round: TripRound::default(),
                    source: Rc::new(source),
                    trip: Rc::new(trip),
                })
            })
            .unwrap_or_else(|error| ShownTrip::Failed(Rc::new(error)))
    }

    #[must_use]
    pub fn next(&self, source: &TripText, trip: Trip) -> ShownTrip {
        ShownTrip::Opened {
            round: self.round().advanced(),
            source: Rc::new(source.clone()),
            trip: Rc::new(trip),
        }
    }

    #[must_use]
    pub fn source(&self) -> Option<&TripText> {
        match self {
            ShownTrip::Opened { source, .. } => Some(source),
            ShownTrip::Nothing | ShownTrip::Failed(_) => None,
        }
    }

    #[must_use]
    pub fn trip(&self) -> Option<Rc<Trip>> {
        match self {
            ShownTrip::Opened { trip, .. } => Some(Rc::clone(trip)),
            ShownTrip::Nothing | ShownTrip::Failed(_) => None,
        }
    }

    pub fn view(&self, holder: TripShelf) -> Element {
        match self {
            ShownTrip::Opened { round, .. } => {
                let scope = TripScope::new(*round, holder);
                rsx! {
                    {std::iter::once(rsx! { Fragment { key: "{round}", {scope} } })}
                }
            }
            ShownTrip::Failed(error) => rsx! {
                main {
                    p { class: "error", "The guide data failed to load: {error}" }
                    {holder}
                }
            },
            ShownTrip::Nothing => rsx! {},
        }
    }

    fn round(&self) -> TripRound {
        match self {
            ShownTrip::Opened { round, .. } => *round,
            ShownTrip::Nothing | ShownTrip::Failed(_) => TripRound::default(),
        }
    }
}
