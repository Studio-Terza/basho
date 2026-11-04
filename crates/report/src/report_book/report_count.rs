#[derive(
    newtype::Newtype,
    newtype::Arbitrary,
    newtype::Display,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
)]
pub struct ReportCount(usize);
