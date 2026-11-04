use newtype::Newtype;
use std::fmt;

use guide::ruby_text::RubyText;

use super::escaped::Escaped;

#[derive(Newtype)]
pub struct RubyMarkup<'text>(&'text RubyText);

impl fmt::Display for RubyMarkup<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.segments().iter().try_for_each(|segment| {
            Self::write_segment(
                formatter,
                Escaped::new(segment.text()),
                segment.reading().map(|reading| Escaped::new(reading)),
            )
        })
    }
}

impl RubyMarkup<'_> {
    fn write_segment(
        formatter: &mut fmt::Formatter<'_>,
        text: Escaped<'_>,
        reading: Option<Escaped<'_>>,
    ) -> fmt::Result {
        match reading {
            Some(reading) => write!(formatter, "<ruby>{text}<rt>{reading}</rt></ruby>"),
            None => write!(formatter, "{text}"),
        }
    }
}
