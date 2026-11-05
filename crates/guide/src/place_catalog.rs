use crate::itinerary::{Day, Finding, Itinerary, Problem};
use crate::leg::LegIndex;
use crate::place::Place;
use crate::words::Slug;
use newtype::Newtype;

#[derive(Newtype, Debug)]
pub struct PlaceCatalog {
    places: Vec<Place>,
}

impl PlaceCatalog {
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
        day.legs().iter().enumerate().filter_map(|(index, leg)| {
            leg.place()
                .and_then(|slug| self.unlisted(day, LegIndex::new(index), slug))
        })
    }

    fn unlisted(&self, day: &Day, leg: LegIndex, slug: &Slug) -> Option<Finding> {
        match self.places.iter().find(|place| place.slug() == slug) {
            Some(_) => None,
            None => Some(Finding::on_leg(
                day.number(),
                leg,
                Problem::UnknownPlace {
                    slug: slug.to_owned(),
                },
            )),
        }
    }
}
