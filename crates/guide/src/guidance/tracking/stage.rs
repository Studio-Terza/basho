#[derive(
    strum::Display, strum::EnumString, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[strum(serialize_all = "lowercase")]
pub enum Stage {
    Boarding,
    Under,
    Finished,
}
