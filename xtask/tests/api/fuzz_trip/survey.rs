use day_print::DaysHtml;
use guide::Trip;
use guide::TripSource;
use thiserror::Error;
use wiki::Wiki;

#[derive(Debug, Error)]
pub enum Unsafe {
    #[error("a refusal with no words")]
    SilentRefusal,
    #[error("an accepted trip printed nothing")]
    BlankPrint,
}

pub struct Survey;

impl Survey {
    pub fn of_text(text: &str) -> Result<(), Unsafe> {
        let outcome = match text.parse::<Trip>() {
            Ok(trip) => Self::of(&trip),
            Err(error) if error.to_string().is_empty() => Err(Unsafe::SilentRefusal),
            Err(_) => Ok(()),
        };
        TripSource::new(text).findings().len();
        outcome
    }

    pub fn of(trip: &Trip) -> Result<(), Unsafe> {
        let wiki = Wiki::new(
            trip.itinerary(),
            trip.places().places(),
            trip.overview().landmarks(),
        );
        let _ = wiki.names();
        Trip::contents(trip.itinerary(), trip.places(), trip.overview())
            .iter()
            .map(ToString::to_string)
            .count();
        let _ = trip.unresolved_names();
        Some(DaysHtml::new(trip.itinerary()).to_string())
            .filter(|printed| !printed.is_empty())
            .map(drop)
            .ok_or(Unsafe::BlankPrint)
    }
}
