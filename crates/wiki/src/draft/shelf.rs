use guide::place::Fact;
use guide::reading::Reading;

use crate::draft::stop_ref::StopRef;
use crate::entry::Heading;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shelf {
    Supplies,
    Souvenirs,
}

impl Shelf {
    const SUPPLIES: [&str; 11] = [
        "konbini",
        "convenience store",
        "supermarket",
        "grocery",
        "7-eleven",
        "セブン",
        "lawson",
        "ローソン",
        "familymart",
        "ファミリーマート",
        "bakery",
    ];

    #[must_use]
    pub fn of_stop(stop: &StopRef<'_>) -> Shelf {
        std::iter::once(stop.stop_name())
            .chain(stop.along().detail().map(Fact::text))
            .map(|text| text.text(Reading::Written).to_lowercase())
            .find(|text| Self::SUPPLIES.iter().any(|word| text.contains(word)))
            .map_or(Shelf::Souvenirs, |_| Shelf::Supplies)
    }

    #[must_use]
    pub fn heading(self) -> Heading {
        match self {
            Shelf::Supplies => Heading::Supplies,
            Shelf::Souvenirs => Heading::Souvenirs,
        }
    }
}
