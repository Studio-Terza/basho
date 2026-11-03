use std::convert::Infallible;
use std::str::FromStr;

use crate::day_number::DayNumber;
use crate::english_name::EnglishName;
use crate::landmark::Landmarks;
use crate::reading::Reading;
use crate::ruby_text::RubyText;
use crate::words::Spelling;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NameForms {
    forms: Vec<Spelling>,
}

impl NameForms {
    const LEAST_JAPANESE_LEN: usize = 2;
    const LEAST_LATIN_LEN: usize = 4;

    #[must_use]
    pub fn of_name(name: &RubyText) -> NameForms {
        name.text(Reading::Written)
            .parse()
            .unwrap_or_else(|never| match never {})
    }

    #[must_use]
    pub fn joined(self, other: NameForms) -> NameForms {
        NameForms {
            forms: self.forms.into_iter().chain(other.forms).collect(),
        }
    }

    #[must_use]
    pub fn also(self, english: &EnglishName) -> NameForms {
        NameForms {
            forms: self
                .forms
                .into_iter()
                .chain([Spelling::new(english.to_string())])
                .collect(),
        }
    }

    #[must_use]
    pub fn of_features(landmarks: &Landmarks) -> Vec<Spelling> {
        let named = |name: &RubyText, english: &EnglishName| {
            NameForms::of_name(name).also(english).into_forms()
        };
        let peaks = landmarks
            .peaks()
            .iter()
            .flat_map(|peak| named(peak.as_ref(), peak.as_ref()));
        let ranges = landmarks
            .ranges()
            .iter()
            .flat_map(|range| named(range.as_ref(), range.as_ref()));
        let seas = landmarks
            .seas()
            .iter()
            .flat_map(|sea| named(sea.as_ref(), sea.as_ref()));
        let rivers = landmarks
            .rivers()
            .iter()
            .flat_map(|river| named(river.as_ref(), river.as_ref()));
        peaks.chain(ranges).chain(seas).chain(rivers).collect()
    }

    #[must_use]
    pub fn of_day(day: DayNumber) -> NameForms {
        NameForms {
            forms: vec![Spelling::new(format!("Day {day}"))],
        }
    }

    #[must_use]
    pub fn into_forms(self) -> Vec<Spelling> {
        self.forms
    }

    #[must_use]
    pub fn head(name: &RubyText) -> Spelling {
        let written = name.text(Reading::Written);
        Spelling::new(
            written
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_owned(),
        )
    }

    fn japanese(letter: char) -> bool {
        ('\u{3040}'..='\u{9FFF}').contains(&letter)
    }

    fn latin(text: &str) -> Option<&str> {
        Some(text).filter(|text| !text.chars().any(Self::japanese))
    }

    fn long_enough(form: &Spelling) -> bool {
        form.chars().count()
            >= Self::latin(form).map_or(Self::LEAST_JAPANESE_LEN, |_| Self::LEAST_LATIN_LEN)
    }
}

impl FromStr for NameForms {
    type Err = Infallible;

    fn from_str(whole: &str) -> Result<NameForms, Infallible> {
        let head = whole
            .split_whitespace()
            .next()
            .filter(|head| Self::latin(head).is_none());
        Ok(NameForms {
            forms: std::iter::once(whole)
                .chain(head)
                .map(|form| Spelling::new(form.to_owned()))
                .filter(Self::long_enough)
                .collect(),
        })
    }
}
