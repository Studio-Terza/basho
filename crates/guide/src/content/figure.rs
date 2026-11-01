use std::fmt;

use crate::content::ContentError;
use crate::words::{Spelling, Written};

#[derive(derive_more::AsRef, Clone, Debug, PartialEq, Eq)]
pub struct Figure(Spelling);

impl Figure {
    const MARKS: [char; 2] = [',', '.'];

    pub(super) fn leading(text: &str) -> Option<Figure> {
        let length = text
            .find(|letter: char| !letter.is_ascii_digit() && !Self::MARKS.contains(&letter))
            .unwrap_or(text.len());
        text.get(..length)
            .map(|digits| digits.trim_end_matches(Self::MARKS))
            .filter(|digits| digits.starts_with(|letter: char| letter.is_ascii_digit()))
            .map(|digits| Figure(Spelling::new(digits.to_owned())))
    }
}

impl TryFrom<&str> for Figure {
    type Error = ContentError;

    fn try_from(text: &str) -> Result<Figure, ContentError> {
        Self::leading(text)
            .filter(|figure| figure.0.len() == text.len())
            .ok_or_else(|| ContentError::BadMeasure(Written::new(text.to_owned())))
    }
}

impl fmt::Display for Figure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}
