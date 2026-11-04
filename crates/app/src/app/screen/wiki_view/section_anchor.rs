use super::section_index::SectionIndex;
use std::fmt;

#[derive(newtype::Newtype, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionAnchor {
    position_index: SectionIndex,
}

impl fmt::Display for SectionAnchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "section-{}", self.position_index)
    }
}
