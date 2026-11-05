mod content_list;
mod trip_error;
mod trip_file;

use crate::KnownNames;
pub use content_list::ContentList;

use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use crate::content::Content;
use crate::itinerary::{Finding, Itinerary};
use crate::overview::Overview;
use crate::place_catalog::PlaceCatalog;
use trip_file::TripFile;

pub use trip_error::TripError;

#[derive(Debug)]
pub struct Trip {
    itinerary: Arc<Itinerary>,
    places: Arc<PlaceCatalog>,
    overview: Arc<Overview>,
}

impl Trip {
    pub fn load(path: &Path) -> Result<Trip, TripError> {
        fs::read_to_string(path)
            .map_err(|error| TripError::Unreadable {
                path: path.to_path_buf(),
                error,
            })
            .and_then(|text| text.parse())
    }

    #[must_use]
    pub fn new(itinerary: Itinerary, places: PlaceCatalog, overview: Overview) -> Trip {
        Trip {
            itinerary: Arc::new(itinerary),
            places: Arc::new(places),
            overview: Arc::new(overview),
        }
    }

    #[must_use]
    pub fn contents<'trip>(
        itinerary: &'trip Itinerary,
        places: &'trip PlaceCatalog,
        overview: &'trip Overview,
    ) -> Vec<&'trip Content> {
        let arcs = itinerary.arcs().iter().map(crate::itinerary::Arc::title);
        let days = itinerary.days().iter().flat_map(ContentList::from);
        let cards = places.places().iter().flat_map(ContentList::from);
        let stars = overview
            .landmarks()
            .star_points()
            .iter()
            .flat_map(ContentList::from);
        arcs.chain(days).chain(cards).chain(stars).collect()
    }

    #[must_use]
    pub fn unresolved_names(&self) -> Vec<Finding> {
        KnownNames::new(
            &self.itinerary,
            self.places.places(),
            self.overview.landmarks(),
        )
        .unresolved(&Self::contents(
            &self.itinerary,
            &self.places,
            &self.overview,
        ))
    }

    #[must_use]
    pub fn itinerary(&self) -> &Arc<Itinerary> {
        &self.itinerary
    }

    #[must_use]
    pub fn places(&self) -> &Arc<PlaceCatalog> {
        &self.places
    }

    #[must_use]
    pub fn overview(&self) -> &Arc<Overview> {
        &self.overview
    }
}

impl FromStr for Trip {
    type Err = TripError;

    fn from_str(text: &str) -> Result<Trip, TripError> {
        text.parse::<TripFile>().and_then(TripFile::trip)
    }
}
