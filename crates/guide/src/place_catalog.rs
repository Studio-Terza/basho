use crate::itinerary::{Day, Finding, Itinerary, Problem};
use crate::leg::LegIndex;
use crate::place::Place;
use newtype::Newtype;
use newtype::proptest::collection::vec;

#[derive(Newtype, newtype::Arbitrary, Debug)]
#[newtype(strategy = vec(Place::drawn(), 0..Self::MOST_DRAWN))]
pub struct PlaceCatalog {
    places: Vec<Place>,
}

impl PlaceCatalog {
    const MOST_DRAWN: usize = 3;

    #[must_use]
    pub fn places(&self) -> &[Place] {
        &self.places
    }

    #[must_use]
    pub fn findings(&self, itinerary: &Itinerary) -> Vec<Finding> {
        itinerary
            .days()
            .iter()
            .flat_map(|day| self.unknown_on(day))
            .chain(self.repeated())
            .collect()
    }

    fn repeated(&self) -> impl Iterator<Item = Finding> + '_ {
        self.places
            .iter()
            .enumerate()
            .filter(|(index, place)| {
                self.places[..*index]
                    .iter()
                    .any(|earlier| earlier.slug() == place.slug())
            })
            .map(|(_, place)| {
                Finding::whole(Problem::DuplicatePlace {
                    slug: place.slug().to_owned(),
                })
            })
    }

    fn unknown_on<'day>(&'day self, day: &'day Day) -> impl Iterator<Item = Finding> + 'day {
        day.legs()
            .iter()
            .enumerate()
            .filter_map(|(index, leg)| leg.place().map(|slug| (index, slug)))
            .filter(|(_, slug)| !self.places.iter().any(|place| place.slug() == *slug))
            .map(|(index, slug)| {
                Finding::on_leg(
                    day.number(),
                    LegIndex::new(index),
                    Problem::UnknownPlace {
                        slug: slug.to_owned(),
                    },
                )
            })
    }
}
