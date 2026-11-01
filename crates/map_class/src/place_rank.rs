use crate::zoom::Zoom;

#[derive(strum::IntoStaticStr, strum::EnumString, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaceRank {
    #[strum(to_string = "city")]
    City,
    #[strum(to_string = "town")]
    Town,
    #[strum(to_string = "village", serialize = "suburb")]
    Village,
    #[strum(
        to_string = "hamlet",
        serialize = "neighbourhood",
        serialize = "quarter",
        serialize = "locality"
    )]
    Hamlet,
}

impl From<PlaceRank> for Zoom {
    fn from(rank: PlaceRank) -> Zoom {
        match rank {
            PlaceRank::City => Zoom::LEAST,
            PlaceRank::Town => Zoom::MAJOR_ROADS,
            PlaceRank::Village => Zoom::STATIONS,
            PlaceRank::Hamlet => Zoom::CELL,
        }
    }
}
