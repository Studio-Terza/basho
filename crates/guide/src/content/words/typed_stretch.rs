use std::ops::Range;
use std::str::FromStr;

use crate::content::DistanceUnit;
use crate::content::{ContentError, Figure, SaidClock};
use crate::ruby_text::{LetterCount, LetterIndex};
use crate::words::{Spelling, Written};
use newtype::Counter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StretchKind {
    Clock,
    Distance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedStretch {
    kind: StretchKind,
    index_range: Range<LetterIndex>,
}

impl TypedStretch {
    const CLOCK_TAIL_LENGTH: LetterCount = LetterCount::new(3);
    const FIGURE_MARKS: [char; 3] = [':', ',', '.'];

    #[must_use]
    pub fn kind(&self) -> StretchKind {
        self.kind
    }

    #[must_use]
    pub fn index_range(&self) -> Range<LetterIndex> {
        self.index_range.clone()
    }

    fn opens(text: &Spelling, at_index: LetterIndex) -> Option<LetterIndex> {
        Some(at_index).filter(|at_index| {
            !LetterIndex::default()
                .between(*at_index, text)
                .chars()
                .next_back()
                .is_some_and(|letter| {
                    letter.is_ascii_alphanumeric() || Self::FIGURE_MARKS.contains(&letter)
                })
        })
    }

    fn clock_length(text: &Spelling, figure: &Figure) -> Option<LetterCount> {
        let clock_end = LetterCount::letters_in(figure.as_ref()).plus(Self::CLOCK_TAIL_LENGTH);
        let end = LetterIndex::default() + clock_end;
        Some(LetterIndex::default().between(end, text))
            .filter(|clock| LetterCount::letters_in(clock) == clock_end)
            .and_then(|clock| SaidClock::read(&clock))?;
        (!end
            .between(LetterIndex::END, text)
            .starts_with(|letter: char| letter.is_ascii_digit() || letter == ':'))
        .then_some(clock_end)
    }

    fn distance_length(text: &Spelling, figure_length: LetterCount) -> Option<LetterCount> {
        let rest = (LetterIndex::default() + figure_length).between(LetterIndex::END, text);
        let unit = rest.strip_prefix(' ').unwrap_or(&rest);
        let unit_end = unit
            .find(|letter: char| !letter.is_ascii_alphanumeric())
            .unwrap_or(unit.len());
        unit.get(..unit_end)?.parse::<DistanceUnit>().ok()?;
        Some(LetterCount::new(
            figure_length.count() + rest.len() - unit.len() + unit_end,
        ))
    }

    fn at_start(text: &Spelling, start_index: LetterIndex) -> Option<TypedStretch> {
        let figure = Figure::leading(text)?;
        let stretch = |kind: StretchKind, length: LetterCount| TypedStretch {
            kind,
            index_range: start_index..start_index + length,
        };
        Self::clock_length(text, &figure)
            .map(|length| stretch(StretchKind::Clock, length))
            .or_else(|| {
                Self::distance_length(text, LetterCount::letters_in(figure.as_ref()))
                    .map(|length| stretch(StretchKind::Distance, length))
            })
    }
}

impl FromStr for TypedStretch {
    type Err = ContentError;

    fn from_str(text: &str) -> Result<TypedStretch, ContentError> {
        let spelling = Spelling::new(text.to_owned());
        text.chars()
            .enumerate()
            .filter(|(_, letter)| letter.is_ascii_digit())
            .filter_map(|(at_index, _)| Self::opens(&spelling, LetterIndex::new(at_index)))
            .find_map(|at_index| {
                Self::at_start(&at_index.between(LetterIndex::END, &spelling), at_index)
            })
            .ok_or_else(|| ContentError::BadMeasure(Written::new(text.to_owned())))
    }
}
