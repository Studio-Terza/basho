#[derive(newtype::Newtype, newtype::Display, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionIndex(usize);
