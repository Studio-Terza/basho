use newtype::Newtype;
use std::fmt::{self, Write};

#[derive(Newtype)]
pub struct Escaped<'text>(&'text str);

impl fmt::Display for Escaped<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.chars().try_for_each(|character| match character {
            '&' => formatter.write_str("&amp;"),
            '<' => formatter.write_str("&lt;"),
            '>' => formatter.write_str("&gt;"),
            '"' => formatter.write_str("&quot;"),
            other => formatter.write_char(other),
        })
    }
}
