mod uncommented;

use std::convert::Infallible;
use std::str::FromStr;

use basho_lints::LineNumber;

use super::boundary::Quoting;
use super::style_rule::{Context, Declaration, Selector, StyleRule};
use uncommented::Uncommented;

pub struct StyleRules(Vec<StyleRule>);

impl StyleRules {
    const NESTING: [&str; 4] = ["@media", "@supports", "@layer", "@container"];
    const STARTING: &str = "@starting-style";

    #[must_use]
    pub fn into_vec(self) -> Vec<StyleRule> {
        self.0
    }

    fn blocks(text: &str, first: LineNumber, context: Context) -> Vec<StyleRule> {
        Self::unquoted_index(text, '{').map_or_else(Vec::new, |open| {
            let header = &text[..open];
            let body_start = open + 1;
            let close = Self::unquoted_index(&text[body_start..], '}')
                .map_or(text.len(), |at| body_start + at);
            let body = &text[body_start..close.min(text.len())];
            let leading = header.len() - header.trim_start().len();
            let header_line = first.past_lines_in(&header[..leading]);
            let consumed = (close + 1).min(text.len());
            Self::block(header.trim(), body, header_line, context)
                .into_iter()
                .chain(Self::blocks(
                    &text[consumed..],
                    first.past_lines_in(&text[..consumed]),
                    context,
                ))
                .collect()
        })
    }

    fn block(selector: &str, body: &str, line: LineNumber, context: Context) -> Vec<StyleRule> {
        if Self::NESTING.iter().any(|at| selector.starts_with(at)) {
            Self::blocks(body, line, context)
        } else if selector.starts_with(Self::STARTING) {
            Self::blocks(body, line, Context::Starting)
        } else if Self::unquoted_index(body, '{').is_some() {
            vec![StyleRule::new(
                Selector::from(selector),
                Self::declarations(body),
                line,
                Context::Nested,
            )]
        } else {
            vec![StyleRule::new(
                Selector::from(selector),
                Self::declarations(body),
                line,
                context,
            )]
        }
    }

    fn declarations(body: &str) -> Vec<Declaration> {
        body.split(';')
            .filter_map(|part| part.parse().ok())
            .collect()
    }

    fn unquoted_index(text: &str, wanted: char) -> Option<usize> {
        text.char_indices()
            .scan(Quoting::default(), |state, (at, character)| {
                let before = *state;
                *state = before.after(character, '{', '}');
                Some((at, character, *state, before))
            })
            .find_map(|(at, character, after, before)| {
                after.reaching(before, character, wanted).map(|_| at)
            })
    }
}

impl FromStr for StyleRules {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<StyleRules, Infallible> {
        let Ok(plain) = text.parse::<Uncommented>();
        Ok(StyleRules(Self::blocks(
            &plain,
            LineNumber::FIRST,
            Context::Plain,
        )))
    }
}
