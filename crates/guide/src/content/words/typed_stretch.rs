use std::ops::Range;
use std::str::FromStr;

use crate::content::{ContentError, DistanceUnit, Figure, SaidClock};
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

    fn opens(text: &str, at_index: usize) -> bool {
        !text
            .get(..at_index)
            .and_then(|before| before.chars().next_back())
            .is_some_and(|letter| {
                letter.is_ascii_alphanumeric() || Self::FIGURE_MARKS.contains(&letter)
            })
    }

    fn clock_length(text: &str, figure: &Figure) -> Option<usize> {
        let clock_end = AsRef::<Spelling>::as_ref(figure).len() + Self::CLOCK_TAIL_LENGTH;
        SaidClock::read(text.get(..clock_end)?)?;
        (!text
            .get(clock_end..)?
            .starts_with(|letter: char| letter.is_ascii_digit() || letter == ':'))
        .then_some(clock_end)
    }

    fn distance_length(text: &str, figure_length: usize) -> Option<usize> {
        let rest = text.get(figure_length..)?;
        let unit = rest.strip_prefix(' ').unwrap_or(rest);
        let unit_end = unit
            .find(|letter: char| !letter.is_ascii_alphanumeric())
            .unwrap_or(unit.len());
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
            .filter(|(at_index, letter)| letter.is_ascii_digit() && Self::opens(text, *at_index))
            .find_map(|(at_index, _)| {
                text.get(at_index..)
                    .and_then(|rest| Self::at_start(rest, at_index))
            })
            .ok_or_else(|| ContentError::BadMeasure(Written::new(text.to_owned())))
    }
}
