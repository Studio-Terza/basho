use std::fmt;

use guide::evidence::Evidence;
use newtype::Newtype;

#[derive(Newtype)]
pub struct UnverifiedMarkup<'evidence> {
    evidence: &'evidence Evidence,
}

impl fmt::Display for UnverifiedMarkup<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.evidence.mark().map_or(Ok(()), |mark| {
            write!(formatter, " <span class=\"unverified\">{mark}</span>")
        })
    }
}
