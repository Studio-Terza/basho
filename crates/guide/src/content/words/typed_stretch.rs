use std::ops::Range;
use std::str::FromStr;

use crate::content::DistanceUnit;
use crate::content::{ContentError, Figure, SaidClock};
use crate::words::{Spelling, Written};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StretchKind {
    Clock,
    Distance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedStretch {
    kind: StretchKind,
    index_range: Range<usize>,
}

impl TypedStretch {
    const CLOCK_TAIL_LENGTH: usize = 3;
    const FIGURE_MARKS: [char; 3] = [':', ',', '.'];

    #[must_use]
    pub fn kind(&self) -> StretchKind {
        self.kind
    }

    #[must_use]
    pub fn index_range(&self) -> Range<usize> {
        self.index_range.clone()
    }

    fn opens(text: &str, at_index: usize) -> Option<&str> {
        let before = text
            .get(..at_index)
            .and_then(|before| before.chars().next_back());
        before
            .filter(char::is_ascii_alphanumeric)
            .or(before.filter(|letter| Self::FIGURE_MARKS.contains(letter)))
            .map_or_else(|| text.get(at_index..), |_| None)
    }

    fn clock_length(text: &str, figure: &Figure) -> Option<usize> {
        let clock_end = AsRef::<Spelling>::as_ref(figure).len() + Self::CLOCK_TAIL_LENGTH;
        SaidClock::read(text.get(..clock_end)?)?;
        let after = text.get(clock_end..)?.chars().next();
        after
            .filter(char::is_ascii_digit)
            .or(after.filter(|letter| *letter == ':'))
            .is_none()
            .then_some(clock_end)
    }

    fn distance_length(text: &str, figure_length: usize) -> Option<usize> {
        let rest = text.get(figure_length..)?;
        let unit = rest.strip_prefix(' ').unwrap_or(rest);
        let unit_end = unit.len()
            - unit
                .trim_start_matches(|letter: char| letter.is_ascii_alphanumeric())
                .len();
        unit.get(..unit_end)?.parse::<DistanceUnit>().ok()?;
        Some(text.len() - unit.len() + unit_end)
    }

    fn at_start(text: &str, start_index: usize) -> Option<TypedStretch> {
        let figure = Figure::leading(text)?;
        let stretch = |kind: StretchKind, length: usize| TypedStretch {
            kind,
            index_range: start_index..start_index + length,
        };
        Self::clock_length(text, &figure)
            .map(|length| stretch(StretchKind::Clock, length))
            .or_else(|| {
                Self::distance_length(text, AsRef::<Spelling>::as_ref(&figure).len())
                    .map(|length| stretch(StretchKind::Distance, length))
            })
    }
}

impl FromStr for TypedStretch {
    type Err = ContentError;

    fn from_str(text: &str) -> Result<TypedStretch, ContentError> {
        text.char_indices()
            .filter(|(_, letter)| letter.is_ascii_digit())
            .find_map(|(at_index, _)| {
                Self::opens(text, at_index).and_then(|rest| Self::at_start(rest, at_index))
            })
            .ok_or_else(|| ContentError::BadMeasure(Written::new(text.to_owned())))
    }
}
