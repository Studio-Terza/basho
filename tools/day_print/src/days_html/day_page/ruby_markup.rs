use newtype::Newtype;
use std::fmt;

use guide::ruby_text::RubyText;

use super::escaped::Escaped;

#[derive(Newtype)]
pub struct RubyMarkup<'text>(&'text RubyText);

impl fmt::Display for RubyMarkup<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .segments()
            .iter()
            .try_for_each(|segment| match segment.reading() {
                Some(reading) => write!(
                    formatter,
                    "<ruby>{}<rt>{}</rt></ruby>",
                    Escaped::new(segment.text()),
                    Escaped::new(reading)
                ),
                None => write!(formatter, "{}", Escaped::new(segment.text())),
            })
    }
}
