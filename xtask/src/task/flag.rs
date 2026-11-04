#[derive(newtype::Newtype, newtype::Arbitrary, Clone, Debug, PartialEq, Eq)]
pub struct Flag(String);
