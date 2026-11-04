use guide::words::Label;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Span {
    Clock,
}

impl Span {
    pub const ALL: &'static [Span] = &[Span::Clock];
    const WIDEST_CLOCK: &str = "23:59";

    pub fn name(self) -> Label {
        match self {
            Span::Clock => Label::new("span-clock"),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Span::Clock => write!(formatter, "{}ch", Self::WIDEST_CLOCK.chars().count()),
        }
    }
}
