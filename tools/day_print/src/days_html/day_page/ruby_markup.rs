use newtype::Newtype;
use std::fmt;

use guide::ruby_text::RubyText;

use super::escaped::Escaped;

#[derive(Newtype)]
pub struct RubyMarkup<'text>(&'text RubyText);

impl fmt::Display for RubyMarkup<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.segments().iter().try_for_each(|segment| {
            let marked = segment.reading().map_or_else(
                || Escaped::new(segment.text()).to_string(),
                |reading| {
                    format!(
                        "<ruby>{}<rt>{}</rt></ruby>",
                        Escaped::new(segment.text()),
                        Escaped::new(reading)
                    )
                },
            );
            formatter.write_str(&marked)
        })
    }
}
