use std::convert::Infallible;
use std::str::FromStr;

use proc_macro2::{Ident, Literal, TokenTree};

use crate::source_file::SourceText;

pub struct Placeholders {
    literal: SourceText,
    filled: Vec<SourceText>,
}

impl Placeholders {
    const CLOSED: &str = ".class()";
    const CHOICES: [&str; 2] = ["if", "match"];
    const CLOSED_CALLS: [&str; 2] = [".class()", ".flag()"];
    const KEYWORDS: [&str; 5] = ["else", "loop", "move", "unsafe", "async"];

    pub fn in_literal(literal: &Literal) -> Placeholders {
        let Ok(placeholders) = literal.to_string().parse();
        placeholders
    }

    pub fn literal(&self) -> &SourceText {
        &self.literal
    }

    pub fn first_unclosed(&self) -> Option<&SourceText> {
        self.filled
            .iter()
            .find(|placeholder| !placeholder.ends_with(Self::CLOSED))
    }

    pub fn first_outside(&self, names: &[Ident]) -> Option<&SourceText> {
        self.filled
            .iter()
            .find(|placeholder| !names.iter().any(|name| name == placeholder))
    }

    pub fn closed_value(value: &[TokenTree]) -> bool {
        let shown: String = value.iter().map(ToString::to_string).collect();
        !value.iter().any(|tree| {
            matches!(tree, TokenTree::Ident(word) if Self::CHOICES.iter().any(|choice| word == choice))
        }) && Self::CLOSED_CALLS
            .iter()
            .any(|call| shown.replace(' ', "").ends_with(call))
    }

    pub fn opens_element(before: Option<&TokenTree>) -> bool {
        before.is_some_and(|tree| {
            matches!(tree, TokenTree::Ident(name) if !Self::KEYWORDS.iter().any(|keyword| name == keyword)
                && name.to_string().starts_with(|first: char| first.is_ascii_lowercase()))
        })
    }

    pub fn string_text(tree: &TokenTree) -> Option<SourceText> {
        let text = tree.to_string();
        text.strip_prefix('"')
            .and_then(|rest| rest.strip_suffix('"'))
            .map(|inside| SourceText::new(inside.to_owned()))
    }
}

impl FromStr for Placeholders {
    type Err = Infallible;

    fn from_str(literal: &str) -> Result<Placeholders, Infallible> {
        let text = literal.replace("{{", "").replace("}}", "");
        Ok(Placeholders {
            filled: text
                .split('{')
                .skip(1)
                .filter_map(|rest| rest.split('}').next())
                .map(|placeholder| SourceText::new(placeholder.trim().to_owned()))
                .collect(),
            literal: SourceText::new(text),
        })
    }
}
