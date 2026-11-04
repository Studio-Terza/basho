mod boundary;

use newtype::Newtype;
use std::fmt;

use boundary::Piece;

#[derive(Newtype, Clone, Copy)]
pub struct Escaped<'text>(&'text str);

impl fmt::Display for Escaped<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .chars()
            .try_for_each(|character| write!(formatter, "{}", Piece::from(character)))
    }
}
