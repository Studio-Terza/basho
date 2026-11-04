use std::fmt;

#[derive(newtype::Newtype, newtype::Arbitrary, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionAnchor {
    position_index: usize,
}

impl fmt::Display for SectionAnchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "section-{}", self.position_index)
    }
}
