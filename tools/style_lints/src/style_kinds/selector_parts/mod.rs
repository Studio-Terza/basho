mod name_length;
mod simple_part;

use std::convert::Infallible;
use std::str::FromStr;

use crate::style_sheet::{Context, Quoting, StyleRule};
use name_length::NameLength;
pub use simple_part::SimplePart;

pub struct SelectorParts {
    parts: Vec<SimplePart>,
}

impl SelectorParts {
    const ELEMENT_MARK: &str = "::";
    const AT_RULE: &str = "@";

    pub fn of_rule(rule: &StyleRule) -> SelectorParts {
        let selector = rule.selector();
        if selector.starts_with(Self::AT_RULE) {
            SelectorParts {
                parts: vec![SimplePart::AtRule],
            }
        } else if rule.context() == Context::Nested {
            SelectorParts {
                parts: vec![SimplePart::Nested],
            }
        } else {
            let Ok(parts) = selector.to_string().parse();
            parts
        }
    }

    pub fn refused(&self) -> Vec<SimplePart> {
        self.parts.iter().flat_map(SimplePart::refused).collect()
    }

    fn split(text: &str) -> Vec<SimplePart> {
        text.chars().next().map_or_else(Vec::new, |first| {
            let token_len = Self::token_len(text, first).max(first.len_utf8());
            SimplePart::try_from(&text[..token_len])
                .into_iter()
                .chain(Self::split(&text[token_len..]))
                .collect()
        })
    }

    fn token_len(text: &str, first: char) -> usize {
        match first {
            '[' => Self::closing_len(text, '[', ']'),
            ':' | '#' | '.' => {
                let marks_len = if text.starts_with(Self::ELEMENT_MARK) {
                    Self::ELEMENT_MARK.len()
                } else {
                    first.len_utf8()
                };
                let named_len = marks_len + usize::from(NameLength::from(&text[marks_len..]));
                named_len + Self::closing_len(&text[named_len..], '(', ')')
            }
            '*' | '+' | '~' | '&' => first.len_utf8(),
            _ => usize::from(NameLength::from(text)),
        }
    }

    fn closing_len(text: &str, open: char, close: char) -> usize {
        if text.starts_with(open) {
            text.char_indices()
                .scan(Quoting::default(), |state, (index, character)| {
                    *state = state.after(character, open, close);
                    Some((index + character.len_utf8(), *state))
                })
                .find(|(_, state)| state.balanced())
                .map_or(text.len(), |(end_len, _)| end_len)
        } else {
            0
        }
    }
}

impl FromStr for SelectorParts {
    type Err = Infallible;

    fn from_str(selector: &str) -> Result<SelectorParts, Infallible> {
        Ok(SelectorParts {
            parts: Self::split(selector),
        })
    }
}
