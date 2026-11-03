use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum DistanceUnit {
    #[strum(serialize = "km")]
    Kilometers,
    #[strum(serialize = "m")]
    Meters,
}
