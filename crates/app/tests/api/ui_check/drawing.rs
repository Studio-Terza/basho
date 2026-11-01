use std::sync::{Arc, LazyLock};

use guide::Overview;
use guide::PlaceCatalog;
use guide::Trip;
use guide::itinerary::Itinerary;

use basho::app::session::trip_shelf::TripInput;
use wiki::Wiki;

static BUILT_IN: LazyLock<Trip> = LazyLock::new(|| {
    TripInput::BUILT_IN
        .parse::<Trip>()
        .expect("the built-in trip loads")
});
#[cfg(test)]
static SECOND: LazyLock<Trip> = LazyLock::new(|| {
    Drawing::SECOND_TRIP
        .replace(Drawing::FIRST_NAME, Drawing::SECOND_NAME)
        .parse::<Trip>()
        .expect("the second trip loads")
});
static BUILT_IN_WIKI: LazyLock<Arc<Wiki>> = LazyLock::new(|| Drawing::BuiltIn.drafted());
#[cfg(test)]
static SECOND_WIKI: LazyLock<Arc<Wiki>> = LazyLock::new(|| Drawing::Second.drafted());

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Drawing {
    #[default]
    BuiltIn,
    #[cfg(test)]
    Second,
}

impl Drawing {
    #[cfg(test)]
    const SECOND_TRIP: &str = include_str!("../../../../../data/trips/sample.toml");
    #[cfg(test)]
    pub const FIRST_NAME: &str = "Eastfield";
    #[cfg(test)]
    const SECOND_NAME: &str = "Westfield";

    fn trip(self) -> &'static Trip {
        match self {
            Drawing::BuiltIn => &BUILT_IN,
            #[cfg(test)]
            Drawing::Second => &SECOND,
        }
    }

    pub fn itinerary(self) -> Arc<Itinerary> {
        Arc::clone(self.trip().itinerary())
    }

    pub fn wiki(self) -> Arc<Wiki> {
        Arc::clone(match self {
            Drawing::BuiltIn => &BUILT_IN_WIKI,
            #[cfg(test)]
            Drawing::Second => &SECOND_WIKI,
        })
    }

    fn drafted(self) -> Arc<Wiki> {
        let trip = self.trip();
        Arc::new(Wiki::new(
            trip.itinerary(),
            trip.places().places(),
            trip.overview().landmarks(),
        ))
    }

    pub fn places(self) -> Arc<PlaceCatalog> {
        Arc::clone(self.trip().places())
    }

    pub fn overview(self) -> Arc<Overview> {
        Arc::clone(self.trip().overview())
    }
}
