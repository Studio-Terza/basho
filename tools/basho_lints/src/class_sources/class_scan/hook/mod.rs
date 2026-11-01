mod placeholders;

use std::fmt;

use proc_macro2::{Ident, TokenTree};

use super::class_problem::ClassProblem;
use crate::source_file::SourceText;
pub use placeholders::Placeholders;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Hook {
    Class,
    Style,
    Free,
    Fixed(SourceText),
}

impl Hook {
    const CLASS: &str = "class";
    const STYLE: &str = "style";
    const FREE: [&str; 5] = ["id", "href", "title", "alt", "key"];
    const FREE_PREFIXES: [&str; 2] = ["aria-", "on"];

    pub fn named_by(tree: &TokenTree, geometry: impl Fn(&SourceText) -> bool) -> Option<Hook> {
        let name = match tree {
            TokenTree::Ident(name) => Some(SourceText::new(name.to_string())),
            TokenTree::Literal(_) => Placeholders::string_text(tree),
            TokenTree::Group(_) | TokenTree::Punct(_) => None,
        }?;
        Some(if &*name == Self::CLASS {
            Hook::Class
        } else if &*name == Self::STYLE {
            Hook::Style
        } else if Self::FREE.contains(&&*name)
            || geometry(&name)
            || Self::FREE_PREFIXES
                .iter()
                .any(|prefix| name.starts_with(prefix))
        {
            Hook::Free
        } else {
            Hook::Fixed(name)
        })
    }

    pub fn refusal(&self, value: &[TokenTree], poses: &[Ident]) -> Option<ClassProblem> {
        let literal = match value {
            [TokenTree::Literal(literal)] => Some(Placeholders::in_literal(literal)),
            _ => None,
        };
        let varying = || ClassProblem::Varying {
            attribute: SourceText::new(self.to_string()),
            value: value.to_vec(),
        };
        match (self, literal) {
            (Hook::Free, _) => None,
            (Hook::Class, Some(placeholders)) => {
                placeholders
                    .first_unclosed()
                    .map(|placeholder| ClassProblem::Filled {
                        literal: placeholders.literal().clone(),
                        placeholder: placeholder.clone(),
                    })
            }
            (Hook::Class, None) => {
                (!Placeholders::closed_value(value)).then(|| ClassProblem::Built(value.to_vec()))
            }
            (Hook::Style, Some(placeholders)) => {
                placeholders
                    .first_outside(poses)
                    .map(|placeholder| ClassProblem::Unposed {
                        literal: placeholders.literal().clone(),
                        placeholder: placeholder.clone(),
                    })
            }
            (Hook::Fixed(_), Some(placeholders)) => {
                placeholders.first_outside(&[]).map(|_| varying())
            }
            (Hook::Fixed(_), None) => (!Placeholders::closed_value(value)).then(varying),
            (Hook::Style, None) => Some(varying()),
        }
    }
}

impl fmt::Display for Hook {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hook::Class => formatter.write_str(Self::CLASS),
            Hook::Style => formatter.write_str(Self::STYLE),
            Hook::Free => formatter.write_str("a free attribute"),
            Hook::Fixed(name) => write!(formatter, "{name}"),
        }
    }
}
