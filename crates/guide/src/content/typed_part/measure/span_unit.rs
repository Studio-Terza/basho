use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum SpanUnit {
    #[strum(serialize = "min")]
    Min,
    #[strum(serialize = "minute")]
    Minute,
    #[strum(serialize = "minutes")]
    Minutes,
    #[strum(serialize = "hour")]
    Hour,
    #[strum(serialize = "hours")]
    Hours,
}
