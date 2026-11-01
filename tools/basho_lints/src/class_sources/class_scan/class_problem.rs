use std::fmt;

use proc_macro2::{Ident, TokenTree};

use crate::source_file::SourceText;

pub enum ClassProblem {
    Filled {
        literal: SourceText,
        placeholder: SourceText,
    },
    Unposed {
        literal: SourceText,
        placeholder: SourceText,
    },
    Varying {
        attribute: SourceText,
        value: Vec<TokenTree>,
    },
    Built(Vec<TokenTree>),
    Returned,
    Unclosed(Ident),
    OpenClassed(Ident),
    HandConverted,
}

impl ClassProblem {
    fn shown(value: &[TokenTree]) -> String {
        value.iter().map(ToString::to_string).collect()
    }
}

impl fmt::Display for ClassProblem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassProblem::Filled {
                literal,
                placeholder,
            } => write!(
                formatter,
                "class {literal} fills in `{placeholder}`; a class part comes only from .class() on a closed enum"
            ),
            ClassProblem::Unposed {
                literal,
                placeholder,
            } => write!(
                formatter,
                "style {literal} fills in `{placeholder}`; inline style is static or a pose local typed PoseStyle or PageShift"
            ),
            ClassProblem::Varying { attribute, value } => write!(
                formatter,
                "{attribute} is `{}`; a selectable attribute is one static string per element",
                Self::shown(value)
            ),
            ClassProblem::Built(value) => write!(
                formatter,
                "class is `{}`; use a string literal or .class() on a closed enum, with no if or match",
                Self::shown(value)
            ),
            ClassProblem::Returned => formatter.write_str("fn class must return &'static str"),
            ClassProblem::Unclosed(owner) => write!(
                formatter,
                "fn class on {owner} must be `match self` over a workspace enum with a string literal per arm"
            ),
            ClassProblem::OpenClassed(owner) => write!(
                formatter,
                "Classed on {owner} needs a workspace enum that derives strum::IntoStaticStr"
            ),
            ClassProblem::HandConverted => formatter.write_str(
                "a conversion to &'static str is written by hand; derive strum::IntoStaticStr, so each variant has its literal",
            ),
        }
    }
}
