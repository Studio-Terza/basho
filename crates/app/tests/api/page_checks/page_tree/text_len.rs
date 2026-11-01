use std::convert::Infallible;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextLen(usize);

impl TextLen {
    const WIDE_FROM: char = '\u{2E80}';
    const WIDE_LEN: usize = 2;
    const NARROW_LEN: usize = 1;
}

impl Add for TextLen {
    type Output = TextLen;

    fn add(self, other: TextLen) -> TextLen {
        TextLen(self.0 + other.0)
    }
}

impl FromStr for TextLen {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<TextLen, Infallible> {
        Ok(TextLen(
            text.chars()
                .filter(|character| !character.is_whitespace())
                .map(|character| {
                    if character >= Self::WIDE_FROM {
                        Self::WIDE_LEN
                    } else {
                        Self::NARROW_LEN
                    }
                })
                .sum(),
        ))
    }
}
